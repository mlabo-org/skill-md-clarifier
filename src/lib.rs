use serde::Serialize;
use serde_yaml::Value as YamlValue;
use std::ffi::OsStr;
use std::fs;
use std::path::{Component, Path, PathBuf};

pub const REPORT_SCHEMA_VERSION: &str = "skill-md-clarifier-validator/v1";
pub mod budget;

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportStatus {
    Passed,
    Findings,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetKind {
    StandaloneSkill,
    PluginRoot,
}

#[derive(Debug, Serialize)]
pub struct Finding {
    pub code: &'static str,
    pub severity: &'static str,
    pub message: String,
    pub location: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillReport {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_characters: Option<usize>,
    pub findings: Vec<Finding>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditReport {
    pub validation_scope: &'static str,
    pub schema_version: &'static str,
    pub status: ReportStatus,
    pub target_kind: TargetKind,
    pub target: String,
    pub findings: Vec<Finding>,
    pub skills: Vec<SkillReport>,
    pub context_budget: Option<budget::BudgetReport>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_change: Option<budget::DescriptionChangeReport>,
}

impl AuditReport {
    pub fn exit_code(&self) -> i32 {
        match self.status {
            ReportStatus::Passed => 0,
            ReportStatus::Findings => 1,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct OperationalError {
    pub code: &'static str,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorEnvelope<'a> {
    pub schema_version: &'static str,
    pub status: &'static str,
    pub error: &'a OperationalError,
}

impl OperationalError {
    fn new(code: &'static str, message: impl Into<String>, path: Option<&Path>) -> Self {
        Self {
            code,
            message: message.into(),
            path: path.map(display_path),
        }
    }

    pub fn envelope(&self) -> ErrorEnvelope<'_> {
        ErrorEnvelope {
            schema_version: REPORT_SCHEMA_VERSION,
            status: "error",
            error: self,
        }
    }
}

pub fn audit_target(input: &Path) -> Result<AuditReport, OperationalError> {
    if !input.exists() {
        return Err(OperationalError::new(
            "TARGET_NOT_FOUND",
            "The requested target does not exist.",
            Some(input),
        ));
    }

    let canonical = fs::canonicalize(input).map_err(|error| {
        OperationalError::new(
            "TARGET_RESOLUTION_FAILED",
            format!("Could not resolve the target: {error}"),
            Some(input),
        )
    })?;

    if is_plugin_cache_path(&canonical) {
        return Err(OperationalError::new(
            "CACHE_ONLY_TARGET",
            "The target is inside a generated plugin cache. Resolve the authoritative plugin source before editing or accepting it as source.",
            Some(&canonical),
        ));
    }

    let (target_kind, skill_paths, top_level_findings) = if canonical.is_file() {
        if canonical.file_name() != Some(OsStr::new("SKILL.md")) {
            return Err(OperationalError::new(
                "TARGET_FILE_NOT_SKILL_MD",
                "A file target must be named SKILL.md.",
                Some(&canonical),
            ));
        }
        (
            TargetKind::StandaloneSkill,
            vec![canonical.clone()],
            Vec::new(),
        )
    } else if canonical.join("SKILL.md").is_file() {
        (
            TargetKind::StandaloneSkill,
            vec![canonical.join("SKILL.md")],
            Vec::new(),
        )
    } else if canonical.join(".claude-plugin").is_dir() || canonical.join("skills").is_dir() {
        let skills_root = resolve_plugin_skills_root(&canonical)?;
        let mut paths = Vec::new();
        collect_skill_files(&skills_root, &mut paths).map_err(|error| {
            OperationalError::new(
                "PLUGIN_SKILLS_ENUMERATION_FAILED",
                format!("Could not enumerate the plugin skills directory: {error}"),
                Some(&skills_root),
            )
        })?;
        paths.sort();
        let findings = if paths.is_empty() {
            vec![Finding {
                code: "PLUGIN_SKILLS_EMPTY",
                severity: "error",
                message: "The plugin skills directory contains no SKILL.md files."
                    .to_string(),
                location: display_path(&skills_root),
            }]
        } else {
            Vec::new()
        };
        (TargetKind::PluginRoot, paths, findings)
    } else {
        return Err(OperationalError::new(
            "TARGET_KIND_UNRESOLVED",
            "The target is neither a standalone skill directory, a SKILL.md file, nor a Claude Code plugin root with a .claude-plugin or skills directory.",
            Some(&canonical),
        ));
    };

    let skills = skill_paths
        .iter()
        .map(|path| audit_skill(path))
        .collect::<Vec<_>>();
    let has_findings =
        !top_level_findings.is_empty() || skills.iter().any(|report| !report.findings.is_empty());

    Ok(AuditReport {
        validation_scope: "structure_only",
        context_budget: None,
        description_change: None,
        schema_version: REPORT_SCHEMA_VERSION,
        status: if has_findings {
            ReportStatus::Findings
        } else {
            ReportStatus::Passed
        },
        target_kind,
        target: display_path(&canonical),
        findings: top_level_findings,
        skills,
    })
}

// Plugin sources ship no plugin.json (claude-plugin-refresh generates it in staging), so the
// skills directory is the default `skills/` unless a manifest declares a string `skills` path.
fn resolve_plugin_skills_root(plugin_root: &Path) -> Result<PathBuf, OperationalError> {
    let manifest_path = plugin_root.join(".claude-plugin/plugin.json");
    let declared = if manifest_path.is_file() {
        let bytes = fs::read(&manifest_path).map_err(|error| {
            OperationalError::new(
                "PLUGIN_MANIFEST_READ_FAILED",
                format!("Could not read the plugin manifest: {error}"),
                Some(&manifest_path),
            )
        })?;
        let manifest: serde_json::Value = serde_json::from_slice(&bytes).map_err(|error| {
            OperationalError::new(
                "PLUGIN_MANIFEST_PARSE_FAILED",
                format!("Could not parse the plugin manifest as JSON: {error}"),
                Some(&manifest_path),
            )
        })?;
        match manifest.get("skills") {
            None => "skills".to_string(),
            Some(value) => value.as_str().map(str::to_owned).ok_or_else(|| {
                OperationalError::new(
                    "PLUGIN_SKILLS_PATH_NOT_STRING",
                    "The plugin manifest `skills` field is not a string path.",
                    Some(&manifest_path),
                )
            })?,
        }
    } else {
        "skills".to_string()
    };

    let relative = Path::new(&declared);
    if relative.as_os_str().is_empty()
        || relative
            .components()
            .any(|component| !matches!(component, Component::CurDir | Component::Normal(_)))
    {
        return Err(OperationalError::new(
            "PLUGIN_SKILLS_PATH_UNSAFE",
            "The manifest-declared skills path must be a contained relative path without parent traversal.",
            Some(&manifest_path),
        ));
    }

    let joined = plugin_root.join(relative);
    let canonical = fs::canonicalize(&joined).map_err(|error| {
        OperationalError::new(
            "PLUGIN_SKILLS_PATH_UNAVAILABLE",
            format!("Could not resolve the manifest-declared skills path: {error}"),
            Some(&joined),
        )
    })?;
    if !canonical.starts_with(plugin_root) || !canonical.is_dir() {
        return Err(OperationalError::new(
            "PLUGIN_SKILLS_PATH_INVALID",
            "The manifest-declared skills path is not a contained directory.",
            Some(&canonical),
        ));
    }
    Ok(canonical)
}

fn collect_skill_files(root: &Path, paths: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let path = entry.path();
        if file_type.is_dir() {
            collect_skill_files(&path, paths)?;
        } else if file_type.is_file() && entry.file_name() == OsStr::new("SKILL.md") {
            paths.push(path);
        }
    }
    Ok(())
}

fn audit_skill(path: &Path) -> SkillReport {
    let mut report = SkillReport {
        path: display_path(path),
        name: None,
        description_characters: None,
        findings: Vec::new(),
    };

    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => {
            report.findings.push(Finding {
                code: "SKILL_READ_FAILED",
                severity: "error",
                message: format!("Could not read SKILL.md as UTF-8 text: {error}"),
                location: report.path.clone(),
            });
            return report;
        }
    };

    let (frontmatter, body) = match split_frontmatter(&content) {
        Ok(parts) => parts,
        Err(message) => {
            report.findings.push(Finding {
                code: "FRONTMATTER_MISSING_OR_UNCLOSED",
                severity: "error",
                message: message.to_string(),
                location: report.path.clone(),
            });
            return report;
        }
    };

    let metadata: YamlValue = match serde_yaml::from_str(frontmatter) {
        Ok(metadata) => metadata,
        Err(error) => {
            report.findings.push(Finding {
                code: "FRONTMATTER_INVALID_YAML",
                severity: "error",
                message: format!("Could not parse YAML frontmatter: {error}"),
                location: report.path.clone(),
            });
            return report;
        }
    };

    let name = yaml_string(&metadata, "name");
    match name {
        Some(value) if !value.trim().is_empty() => report.name = Some(value.to_string()),
        _ => report.findings.push(Finding {
            code: "NAME_MISSING",
            severity: "error",
            message: "Frontmatter `name` must be a non-empty string.".to_string(),
            location: report.path.clone(),
        }),
    }

    let description = yaml_string(&metadata, "description");
    if let Some(description) = description {
        let characters = description.chars().count();
        report.description_characters = Some(characters);
        if characters > budget::DESCRIPTION_CHARACTER_LIMIT {
            report.findings.push(Finding {
                code: "DESCRIPTION_OVER_FORMAT_LIMIT",
                severity: "error",
                message: format!("Frontmatter description exceeds the {}-character format limit.", budget::DESCRIPTION_CHARACTER_LIMIT),
                location: report.path.clone(),
            });
        }
        if starts_with_generic_routing(description) {
            report.findings.push(Finding {
                code: "DESCRIPTION_GENERIC_OPENING",
                severity: "warning",
                message: "Frontmatter description opens with generic routing prose instead of the task class and primary object."
                    .to_string(),
                location: report.path.clone(),
            });
        }
    } else {
        report.findings.push(Finding {
            code: "DESCRIPTION_MISSING",
            severity: "error",
            message: "Frontmatter `description` must be a string.".to_string(),
            location: report.path.clone(),
        });
    }

    let body_lower = body.to_lowercase();
    let has_contract_declaration = body_lower.contains("local execution contract")
        || body_lower.contains("binding instructions")
        || body.contains("局所実行契約")
        || body.contains("拘束力のある実行条件");
    if !has_contract_declaration {
        report.findings.push(Finding {
            code: "EXECUTION_CONTRACT_DECLARATION_MISSING",
            severity: "error",
            message: "The body does not declare that SKILL.md is the local execution contract when selected."
                .to_string(),
            location: report.path.clone(),
        });
    }

    audit_agents_metadata(path, report.name.as_deref(), &mut report.findings);
    report
}

fn audit_agents_metadata(skill_path: &Path, name: Option<&str>, findings: &mut Vec<Finding>) {
    let Some(skill_root) = skill_path.parent() else {
        return;
    };
    // agents/openai.yaml is optional Codex UI metadata that Claude Code does not read.
    let metadata_path = skill_root.join("agents/openai.yaml");
    if !metadata_path.exists() {
        return;
    }

    let content = match fs::read_to_string(&metadata_path) {
        Ok(content) => content,
        Err(error) => {
            findings.push(Finding {
                code: "AGENTS_OPENAI_YAML_READ_FAILED",
                severity: "error",
                message: format!("Could not read agents/openai.yaml: {error}"),
                location: display_path(&metadata_path),
            });
            return;
        }
    };
    let metadata: YamlValue = match serde_yaml::from_str(&content) {
        Ok(metadata) => metadata,
        Err(error) => {
            findings.push(Finding {
                code: "AGENTS_OPENAI_YAML_INVALID",
                severity: "error",
                message: format!("Could not parse agents/openai.yaml: {error}"),
                location: display_path(&metadata_path),
            });
            return;
        }
    };

    for field in ["display_name", "short_description", "default_prompt"] {
        if nested_yaml_string(&metadata, "interface", field)
            .is_none_or(|value| value.trim().is_empty())
        {
            findings.push(Finding {
                code: "AGENTS_OPENAI_FIELD_MISSING",
                severity: "error",
                message: format!(
                    "agents/openai.yaml interface.{field} must be a non-empty string."
                ),
                location: display_path(&metadata_path),
            });
        }
    }

    if let (Some(name), Some(default_prompt)) = (
        name,
        nested_yaml_string(&metadata, "interface", "default_prompt"),
    ) && !default_prompt.contains(&format!("${name}"))
        && !default_prompt.contains(name)
    {
        findings.push(Finding {
            code: "DEFAULT_PROMPT_SKILL_NAME_MISSING",
            severity: "warning",
            message: format!("interface.default_prompt does not name ${name}."),
            location: display_path(&metadata_path),
        });
    }
}

fn split_frontmatter(content: &str) -> Result<(&str, &str), &'static str> {
    let content = content
        .strip_prefix("---\n")
        .ok_or("SKILL.md must start with YAML frontmatter delimited by lines containing `---`.")?;
    let marker = "\n---\n";
    let end = content
        .find(marker)
        .ok_or("SKILL.md YAML frontmatter is not closed by a `---` line.")?;
    Ok((&content[..end], &content[end + marker.len()..]))
}

fn yaml_string<'a>(value: &'a YamlValue, key: &str) -> Option<&'a str> {
    value
        .as_mapping()?
        .get(YamlValue::String(key.to_string()))?
        .as_str()
}

fn nested_yaml_string<'a>(value: &'a YamlValue, parent: &str, key: &str) -> Option<&'a str> {
    let parent_value = value
        .as_mapping()?
        .get(YamlValue::String(parent.to_string()))?;
    yaml_string(parent_value, key)
}

fn starts_with_generic_routing(description: &str) -> bool {
    let lower = description.trim_start().to_lowercase();
    lower.starts_with("use when ")
        || lower.starts_with("use this skill when ")
        || lower.starts_with("help with ")
}

fn is_plugin_cache_path(path: &Path) -> bool {
    let parts = path
        .components()
        .filter_map(|component| match component {
            Component::Normal(part) => part.to_str(),
            _ => None,
        })
        .collect::<Vec<_>>();
    parts
        .windows(3)
        .any(|window| {
            matches!(window[0], ".codex" | ".claude") && window[1] == "plugins" && window[2] == "cache"
        })
}

fn display_path(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn write_valid_skill(root: &Path, description: &str) {
        fs::create_dir_all(root.join("agents")).expect("create agents directory");
        fs::write(
            root.join("SKILL.md"),
            format!(
                "---\nname: sample-skill\ndescription: {description:?}\n---\n\n# Sample\n\nThis `SKILL.md` is the local execution contract for this skill when selected.\n"
            ),
        )
        .expect("write SKILL.md");
        fs::write(
            root.join("agents/openai.yaml"),
            "interface:\n  display_name: Sample\n  short_description: Audit sample routing.\n  default_prompt: Use $sample-skill to audit this skill.\n",
        )
        .expect("write agents/openai.yaml");
    }

    #[test]
    fn valid_standalone_skill_passes() {
        let temp = tempfile::tempdir().expect("create temp directory");
        write_valid_skill(temp.path(), "Audit sample SKILL.md routing metadata.");
        let report = audit_target(temp.path()).expect("audit target");
        assert_eq!(report.exit_code(), 0);
        assert!(report.findings.is_empty());
        assert!(report.skills[0].findings.is_empty());
    }

    #[test]
    fn long_description_is_a_finding() {
        let temp = tempfile::tempdir().expect("create temp directory");
        let description = format!("Audit {}", "x".repeat(1024));
        write_valid_skill(temp.path(), &description);
        let report = audit_target(temp.path()).expect("audit target");
        assert_eq!(report.exit_code(), 1);
        assert!(
            report.skills[0]
                .findings
                .iter()
                .any(|finding| finding.code == "DESCRIPTION_OVER_FORMAT_LIMIT")
        );
    }

    #[test]
    fn plugin_source_without_manifest_enumerates_default_skills() {
        let temp = tempfile::tempdir().expect("create temp directory");
        fs::create_dir_all(temp.path().join(".claude-plugin")).expect("create plugin dir");
        write_valid_skill(
            &temp.path().join("skills/nested/sample"),
            "Audit nested sample SKILL.md routing metadata.",
        );
        let report = audit_target(temp.path()).expect("audit plugin");
        assert_eq!(report.exit_code(), 0);
        assert_eq!(report.skills.len(), 1);
    }

    #[test]
    fn plugin_source_with_only_skills_directory_is_a_plugin_root() {
        let temp = tempfile::tempdir().expect("create temp directory");
        write_valid_skill(
            &temp.path().join("skills/sample"),
            "Audit sample SKILL.md routing metadata.",
        );
        let report = audit_target(temp.path()).expect("audit plugin");
        assert_eq!(report.exit_code(), 0);
        assert_eq!(report.skills.len(), 1);
    }

    #[test]
    fn plugin_manifest_declared_skills_path_is_used() {
        let temp = tempfile::tempdir().expect("create temp directory");
        fs::create_dir_all(temp.path().join(".claude-plugin")).expect("create plugin dir");
        fs::write(
            temp.path().join(".claude-plugin/plugin.json"),
            r#"{"name":"sample","skills":"./custom"}"#,
        )
        .expect("write manifest");
        write_valid_skill(
            &temp.path().join("custom/sample"),
            "Audit custom sample SKILL.md routing metadata.",
        );
        let report = audit_target(temp.path()).expect("audit plugin");
        assert_eq!(report.exit_code(), 0);
        assert_eq!(report.skills.len(), 1);
    }
}
