//! Host catalog admission: Codex rust-v0.153.4 render.rs units and full lines.
use crate::{AuditReport, Finding, OperationalError, ReportStatus, host::AgentHost};
use serde::Serialize;
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};
// Version-pinned upstream rules. Update this group when adopting a new Codex
// specification; the model context window remains a runtime input.
pub const UPSTREAM_SPEC: &str = "openai/codex rust-v0.153.4 ext/skills/src/render.rs";
// Claude Code lists `description` followed by `when_to_use` for each skill. Its
// shared listing budget has no published counting formula, so only the
// nonincreasing comparison of that listed text is supported for this host.
pub const CLAUDE_CODE_LISTING_SPEC: &str = "code.claude.com/docs/en/skills: listing text is description plus when_to_use";
pub const CONTEXT_BUDGET_PERCENT: usize = 2;
pub const PERCENT_DENOMINATOR: usize = 100;
pub const EXPLICIT_TOKEN_BUDGET_CAP: usize = 10_000;
pub const UNKNOWN_CONTEXT_CHARACTER_BUDGET: usize = 8_000;
pub const APPROX_TOKEN_UTF8_BYTES: usize = 4;
// Individual frontmatter format limit, independent of the shared catalog budget.
pub const DESCRIPTION_CHARACTER_LIMIT: usize = 1_024;
const ABS_INTRO: &str = "A skill is a set of instructions provided through a `SKILL.md` source. Below is the list of skills that can be used. Each entry includes a name, description, and source locator. `file` locators are on the host filesystem, `executor package` locators are owned by their execution environment, `orchestrator package` locators are opaque package identifiers, and `custom resource` locators use their provider's access mechanism.";
const HOST_INTRO: &str = "A skill is a set of local instructions to follow that is stored in a `SKILL.md` file. Below is the list of skills that can be used. Each entry includes a name, description, and a short path that can be expanded into an absolute path using the skill roots table.";
#[derive(Clone, Copy, Debug, Serialize)]
#[serde(tag = "unit", content = "limit", rename_all = "snake_case")]
pub enum Budget {
    ApproxTokens(usize),
    Characters(usize),
}
impl Budget {
    pub fn from_context(
        window: Option<usize>,
        override_tokens: Option<usize>,
    ) -> Result<Self, String> {
        if window == Some(0) || override_tokens == Some(0) {
            return Err("Budget inputs must be positive; use --unknown-context explicitly.".into());
        }
        Ok(match (window, override_tokens) {
            (_, Some(n)) => Self::ApproxTokens(n.min(EXPLICIT_TOKEN_BUDGET_CAP)),
            (Some(n), None) => Self::ApproxTokens(n.saturating_mul(CONTEXT_BUDGET_PERCENT).saturating_div(PERCENT_DENOMINATOR).max(1)),
            _ => Self::Characters(UNKNOWN_CONTEXT_CHARACTER_BUDGET),
        })
    }
    fn cost(self, s: &str) -> usize {
        match self {
            Self::ApproxTokens(_) => s.len().div_ceil(APPROX_TOKEN_UTF8_BYTES),
            Self::Characters(_) => s.chars().count(),
        }
    }
    fn limit(self) -> usize {
        match self {
            Self::ApproxTokens(n) | Self::Characters(n) => n,
        }
    }
}
#[derive(Clone)]
struct Entry {
    name: String,
    path: PathBuf,
    locator: String,
    description: String,
    target: bool,
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Contribution {
    name: String,
    path: PathBuf,
    description_characters: usize,
    description_bytes: usize,
    line_cost: usize,
    target: bool,
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BudgetReport {
    basis: &'static str,
    upstream: &'static str,
    budget: Budget,
    active_skills: usize,
    full_cost: usize,
    minimum_cost: usize,
    alias_overhead: usize,
    excess: usize,
    remaining: usize,
    target_description_allowance: usize,
    fits_without_truncation: bool,
    uniform_existing_prefix_characters: usize,
    uniform_ascii_characters: usize,
    uniform_three_byte_japanese_characters: usize,
    contributions: Vec<Contribution>,
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DescriptionChange {
    source: PathBuf,
    baseline: PathBuf,
    name_unchanged: bool,
    before_bytes: usize,
    after_bytes: usize,
    before_characters: usize,
    after_characters: usize,
    nonincreasing: bool,
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DescriptionChangeReport {
    upstream: &'static str,
    host_budget_status: &'static str,
    nonincreasing: bool,
    comparisons: Vec<DescriptionChange>,
}

/// A description that grows in neither counting unit cannot increase a line's
/// cost under the pinned renderer when its name, locator and catalog are stable.
/// This proves a source delta, never the current host's complete budget fit.
pub fn compare_descriptions(
    r: &mut AuditReport,
    baselines: &[(PathBuf, PathBuf)],
    host: AgentHost,
) -> Result<(), OperationalError> {
    if r.skills.is_empty() || baselines.len() != r.skills.len() {
        return Err(error("Supply exactly one pre-edit baseline for every audited skill"));
    }
    let mut seen = HashSet::new();
    let mut comparisons = Vec::new();
    for (source, baseline) in baselines {
        if !source.is_absolute() || !baseline.is_absolute() || same_path(source, baseline) {
            return Err(error("Baseline mapping requires absolute source and separate pre-edit file paths"));
        }
        let skill = r.skills.iter().find(|s| same_path(Path::new(&s.path), source))
            .ok_or_else(|| error("Baseline source outside the audited target"))?;
        if !seen.insert(skill.path.clone()) {
            return Err(error("Duplicate baseline source"));
        }
        let (before_name, before) = metadata(baseline)?;
        let (after_name, after) = metadata(Path::new(&skill.path))?;
        let name_unchanged = before_name == after_name;
        // Each listed part must be nonincreasing on its own, so the unpublished
        // separator between description and when_to_use never needs counting.
        let mut before_parts = vec![before];
        let mut after_parts = vec![after];
        if host == AgentHost::ClaudeCode {
            before_parts.push(when_to_use(baseline)?);
            after_parts.push(when_to_use(Path::new(&skill.path))?);
        }
        let bytes = |p: &[String]| p.iter().map(String::len).sum::<usize>();
        let characters = |p: &[String]| p.iter().map(|s| s.chars().count()).sum::<usize>();
        let nonincreasing = name_unchanged
            && before_parts.iter().zip(&after_parts).all(|(b, a)| {
                a.len() <= b.len() && a.chars().count() <= b.chars().count()
            });
        if !nonincreasing {
            r.status = ReportStatus::Findings;
            r.findings.push(Finding {
                code: "DESCRIPTION_CHANGE_REQUIRES_HOST_BUDGET",
                severity: "error",
                message: match host {
                    AgentHost::Codex => "The name changed or the description increased in a renderer counting unit. This comparison cannot admit the change; use complete host budget evidence for the affected catalog conditions.".into(),
                    AgentHost::ClaudeCode => "The name changed or the listed description or when_to_use text increased in UTF-8 bytes or characters. This comparison cannot admit the change, and whole-host measurement is not supported for claude_code; check the live listing with /context or /doctor.".into(),
                },
                location: skill.path.clone(),
            });
        }
        comparisons.push(DescriptionChange {
            source: PathBuf::from(&skill.path),
            baseline: baseline.clone(),
            name_unchanged,
            before_bytes: bytes(&before_parts),
            after_bytes: bytes(&after_parts),
            before_characters: characters(&before_parts),
            after_characters: characters(&after_parts),
            nonincreasing,
        });
    }
    r.description_change = Some(DescriptionChangeReport {
        upstream: match host {
            AgentHost::Codex => UPSTREAM_SPEC,
            AgentHost::ClaudeCode => CLAUDE_CODE_LISTING_SPEC,
        },
        host_budget_status: "not_measured",
        nonincreasing: comparisons.iter().all(|c| c.nonincreasing),
        comparisons,
    });
    Ok(())
}
fn error(s: impl Into<String>) -> OperationalError {
    OperationalError {
        code: "CONTEXT_BUDGET_INPUT_INVALID",
        message: s.into(),
        path: None,
    }
}
fn same_path(a: &Path, b: &Path) -> bool {
    a == b || matches!((fs::canonicalize(a),fs::canonicalize(b)),(Ok(a),Ok(b)) if a == b)
}
fn frontmatter(path: &Path) -> Result<serde_yaml::Value, OperationalError> {
    let text = fs::read_to_string(path).map_err(|e| error(format!("{}: {e}", path.display())))?;
    let front = text
        .strip_prefix("---")
        .and_then(|s| s.split_once("\n---").map(|(f, _)| f))
        .ok_or_else(|| error("Missing frontmatter"))?;
    serde_yaml::from_str(front).map_err(|e| error(e.to_string()))
}
fn metadata(path: &Path) -> Result<(String, String), OperationalError> {
    let y = frontmatter(path)?;
    Ok((
        y["name"]
            .as_str()
            .ok_or_else(|| error("Missing name"))?
            .into(),
        y["description"]
            .as_str()
            .ok_or_else(|| error("Missing description"))?
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" "),
    ))
}
/// Normalized `when_to_use` text, empty when the key is absent.
fn when_to_use(path: &Path) -> Result<String, OperationalError> {
    Ok(match &frontmatter(path)?["when_to_use"] {
        serde_yaml::Value::Null => String::new(),
        v => v
            .as_str()
            .ok_or_else(|| error("when_to_use must be a string"))?
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" "),
    })
}
fn body(intro: &str, roots: &[String]) -> String {
    let mut l = vec!["## Skills".into(), intro.into()];
    if !roots.is_empty() {
        l.push("### Skill roots".into());
        l.extend_from_slice(roots)
    }
    l.push("### Available skills".into());
    format!("\n{}\n", l.join("\n"))
}
fn line(e: &Entry, d: &str, alias: bool) -> String {
    let p = if alias {
        e.locator.clone()
    } else {
        e.path.display().to_string()
    };
    if d.is_empty() {
        format!("- {}: (file: {p})\n", e.name)
    } else {
        format!("- {}: {d} (file: {p})\n", e.name)
    }
}
fn cost(es: &[Entry], b: Budget, o: usize, a: bool, f: impl Fn(&Entry) -> String) -> usize {
    es.iter()
        .fold(o, |s, e| s.saturating_add(b.cost(&line(e, &f(e), a))))
}
fn max_uniform(es: &[Entry], b: Budget, o: usize, a: bool, kind: u8) -> usize {
    let (mut lo, mut hi) = (0, DESCRIPTION_CHARACTER_LIMIT + 1);
    while lo + 1 < hi {
        let n = (lo + hi) / 2;
        let c = cost(es, b, o, a, |e| match kind {
            0 => e.description.chars().take(n).collect(),
            1 => "a".repeat(n),
            _ => "あ".repeat(n),
        });
        if c <= b.limit() { lo = n } else { hi = n }
    }
    lo
}
pub fn require_catalog(r: &mut AuditReport) {
    r.status = ReportStatus::Findings;
    r.findings.push(Finding{code:"CONTEXT_BUDGET_UNVERIFIED",severity:"error",message:"Supply the complete actual host Skills fragment and resolved model context before whole-host budget acceptance. Individual length, a nonincreasing description, and skills/list alone cannot establish a warning-free catalog. --structure-only and --baseline-skill have separate source scopes.".into(),location:r.target.clone()})
}
pub fn apply(
    r: &mut AuditReport,
    prompt: &str,
    b: Budget,
    expected: usize,
    maps: &[(PathBuf, PathBuf)],
) -> Result<(), OperationalError> {
    apply_with_reservation(r, prompt, b, expected, maps, None)
}

pub fn apply_with_reservation(
    r: &mut AuditReport,
    prompt: &str,
    b: Budget,
    expected: usize,
    maps: &[(PathBuf, PathBuf)],
    reserve: Option<(&Path, &str)>,
) -> Result<(), OperationalError> {
    if prompt.contains("additional skills omitted")
        || prompt.contains("skills context budget was exceeded")
        || prompt.contains("additional skill omitted")
    {
        return Err(error("Omitted catalog cannot establish complete coverage"));
    }
    let (mut roots, mut root_lines, mut es) =
        (Vec::<(String, String)>::new(), Vec::new(), Vec::new());
    let mut active = false;
    let mut seen = HashSet::new();
    for l in prompt.lines() {
        if let Some(s) = l.strip_prefix("- `") {
            if let Some((k, v)) = s.split_once("` = `") {
                let v = v.strip_suffix('`').ok_or_else(|| error("Invalid root"))?;
                if !Path::new(v).is_absolute() {
                    return Err(error("Root must be absolute"));
                }
                roots.push((k.into(), v.into()));
                root_lines.push(l.into());
                continue;
            }
        }
        if l == "### Available skills" {
            active = true;
            continue;
        }
        if !active {
            continue;
        }
        if l.starts_with("### ") || l.starts_with("## ") || l.starts_with("</skills_instructions>")
        {
            break;
        }
        if !l.starts_with("- ") {
            continue;
        }
        let (h, p) = l[2..].rsplit_once(" (file: ").ok_or_else(|| {
            error("Only complete host file entries are supported; do not omit other sources")
        })?;
        let p = p
            .strip_suffix(')')
            .ok_or_else(|| error("Invalid locator"))?;
        let (name, _) = h.split_once(": ").ok_or_else(|| error("Invalid entry"))?;
        let path = if Path::new(p).is_absolute() {
            PathBuf::from(p)
        } else {
            let (k, t) = p.split_once('/').ok_or_else(|| error("Unresolved alias"))?;
            let root = roots
                .iter()
                .find(|(key, _)| key == k)
                .ok_or_else(|| error("Unknown alias"))?;
            if Path::new(t)
                .components()
                .any(|c| matches!(c, std::path::Component::ParentDir))
            {
                return Err(error("Invalid alias suffix"));
            }
            Path::new(&root.1).join(t)
        };
        if !seen.insert(fs::canonicalize(&path).unwrap_or(path.clone())) {
            return Err(error("Duplicate catalog path"));
        }
        let (_, description) = metadata(&path)?;
        es.push(Entry {
            name: name.into(),
            path,
            locator: p.into(),
            description,
            target: false,
        });
    }
    if expected == 0 || es.len() != expected {
        return Err(error(format!(
            "Expected {expected} complete entries, parsed {}. Completeness must be established from the target host, not skills/list alone.",
            es.len()
        )));
    }
    if let Some((path, name)) = reserve {
        if !path.is_absolute()
            || name.trim().is_empty()
            || name.contains(['\n', '\r'])
            || es.iter().any(|e| e.path == path)
        {
            return Err(error(
                "Reserve requires a new absolute skill path and a nonempty single-line name",
            ));
        }
        let locator = roots
            .iter()
            .filter_map(|(key, root)| {
                path.strip_prefix(root)
                    .ok()
                    .map(|tail| (root.len(), format!("{key}/{}", tail.display())))
            })
            .max_by_key(|(n, _)| *n)
            .map(|(_, p)| p)
            .unwrap_or(path.display().to_string());
        es.push(Entry {
            name: name.into(),
            path: path.into(),
            locator,
            description: String::new(),
            target: true,
        });
    }
    for s in &r.skills {
        let src = PathBuf::from(&s.path);
        let runtime = maps
            .iter()
            .find(|(p, _)| p == &src)
            .map(|(_, p)| p.clone())
            .unwrap_or(src.clone());
        let (name, description) = metadata(&src)?;
        if let Some(e) = es.iter_mut().find(|e| same_path(&e.path, &runtime)) {
            e.name = e
                .name
                .rsplit_once(':')
                .map(|(prefix, _)| format!("{prefix}:{name}"))
                .unwrap_or(name);
            e.description = description;
            e.target = true
        } else {
            if runtime != src {
                return Err(error("Mapped runtime skill absent"));
            }
            let locator = roots
                .iter()
                .filter_map(|(k, p)| {
                    src.strip_prefix(p)
                        .ok()
                        .map(|t| (p.len(), format!("{k}/{}", t.display())))
                })
                .max_by_key(|(n, _)| *n)
                .map(|(_, p)| p)
                .unwrap_or(src.display().to_string());
            es.push(Entry {
                name,
                path: src,
                locator,
                description,
                target: true,
            })
        }
    }
    for (s, _) in maps {
        if !r.skills.iter().any(|x| Path::new(&x.path) == s) {
            return Err(error("Mapping source outside target"));
        }
    }
    let o = if roots.is_empty() {
        0
    } else {
        b.cost(&body(HOST_INTRO, &root_lines))
            .saturating_sub(b.cost(&body(ABS_INTRO, &[])))
    };
    let abs = cost(&es, b, 0, false, |e| e.description.clone());
    let ali = cost(&es, b, o, true, |e| e.description.clone());
    let a = !roots.is_empty() && ali < abs;
    let o = if a { o } else { 0 };
    let full = if a { ali } else { abs };
    let min = cost(&es, b, o, a, |_| String::new());
    let other = cost(&es, b, o, a, |e| {
        if e.target {
            String::new()
        } else {
            e.description.clone()
        }
    });
    let fits = full <= b.limit() && es.iter().all(|e| e.description.chars().count() <= 1024);
    let mut cs = es
        .iter()
        .map(|e| Contribution {
            name: e.name.clone(),
            path: e.path.clone(),
            description_characters: e.description.chars().count(),
            description_bytes: e.description.len(),
            line_cost: b.cost(&line(e, &e.description, a)),
            target: e.target,
        })
        .collect::<Vec<_>>();
    cs.sort_by(|a, b| b.line_cost.cmp(&a.line_cost).then(a.name.cmp(&b.name)));
    r.context_budget = Some(BudgetReport {
        basis: if a {
            "captured_host_aliases_full_descriptions"
        } else {
            "absolute_host_paths_full_descriptions"
        },
        upstream: UPSTREAM_SPEC,
        budget: b,
        active_skills: es.len(),
        full_cost: full,
        minimum_cost: min,
        alias_overhead: o,
        excess: full.saturating_sub(b.limit()),
        remaining: b.limit().saturating_sub(full),
        target_description_allowance: b.limit().saturating_sub(other),
        fits_without_truncation: fits,
        uniform_existing_prefix_characters: max_uniform(&es, b, o, a, 0),
        uniform_ascii_characters: max_uniform(&es, b, o, a, 1),
        uniform_three_byte_japanese_characters: max_uniform(&es, b, o, a, 2),
        contributions: cs,
    });
    if !fits {
        r.status = ReportStatus::Findings;
        r.findings.push(Finding{code:"CONTEXT_BUDGET_EXCEEDED",severity:"error",message:format!("Full catalog cost {full}, limit {}. Truncation and omission are not admitted. Allocate remaining capacity before authoring while preserving trigger meaning. Do not hide warnings, enlarge runtime limits, disable skills, or edit unrelated sources to pass.",b.limit()),location:r.target.clone()})
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn upstream_units_and_limits() {
        let b = Budget::from_context(Some(272000), None).unwrap();
        assert_eq!(b.limit(), 5440);
        assert_eq!(b.cost("日本語\n"), 3);
        assert_eq!(b.cost("abcd\n"), 2);
        assert_eq!(Budget::from_context(None, None).unwrap().limit(), 8000);
        assert_eq!(
            Budget::from_context(Some(272000), Some(12000))
                .unwrap()
                .limit(),
            10000
        );
        assert!(Budget::from_context(Some(0), None).is_err())
    }
    #[test]
    fn utf8_boundary_and_aggregate() {
        let es = vec![
            Entry {
                name: "one".into(),
                path: "/one".into(),
                locator: "/one".into(),
                description: "あいう".into(),
                target: true
            };
            10
        ];
        let c = cost(&es, Budget::ApproxTokens(1), 0, false, |e| {
            e.description.clone()
        });
        assert_eq!(c, 80);
        assert_eq!(c, Budget::ApproxTokens(c).limit());
        assert!(c > Budget::ApproxTokens(c - 1).limit())
    }
    #[test]
    fn candidate_and_complete_snapshot() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("SKILL.md");
        fs::write(&p, "---\nname: sample\ndescription: Audit 日本語.\n---\n").unwrap();
        let prompt = format!(
            "### Available skills\n- sample: Audit (file: {})\n",
            p.display()
        );
        let mut r = crate::audit_target(&p).unwrap();
        apply(&mut r, &prompt, Budget::Characters(8000), 1, &[]).unwrap();
        assert!(r.context_budget.as_ref().unwrap().fits_without_truncation);
        assert_eq!(r.context_budget.as_ref().unwrap().active_skills, 1);
        assert!(apply(&mut r, &prompt, Budget::Characters(8000), 2, &[]).is_err());
        let mut overflow = crate::audit_target(&p).unwrap();
        apply(&mut overflow, &prompt, Budget::ApproxTokens(1), 1, &[]).unwrap();
        assert!(
            !overflow
                .context_budget
                .as_ref()
                .unwrap()
                .fits_without_truncation
        );
        assert!(
            overflow
                .findings
                .iter()
                .any(|f| f.code == "CONTEXT_BUDGET_EXCEEDED")
        );
        let future = d.path().join("future/SKILL.md");
        let mut plan = crate::audit_target(&p).unwrap();
        plan.skills.clear();
        apply_with_reservation(
            &mut plan,
            &prompt,
            Budget::Characters(8000),
            1,
            &[],
            Some((&future, "new-skill")),
        )
        .unwrap();
        assert_eq!(plan.context_budget.as_ref().unwrap().active_skills, 2);
        assert!(
            plan.context_budget
                .as_ref()
                .unwrap()
                .target_description_allowance
                > 0
        );
        assert!(!future.exists());
        require_catalog(&mut r);
        assert_eq!(r.exit_code(), 1)
    }
}
