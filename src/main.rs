use skill_md_clarifier_validator::{OperationalError, audit_target, budget};
use std::{
    io::{self, Write},
    path::{Path, PathBuf},
    process::ExitCode,
};
fn main() -> ExitCode {
    match run() {
        Ok((v, code)) => {
            let mut out = io::stdout().lock();
            if serde_json::to_writer_pretty(&mut out, &v).is_err() || writeln!(out).is_err() {
                return ExitCode::from(2);
            }
            ExitCode::from(code)
        }
        Err(e) => {
            let mut out = io::stderr().lock();
            let _ = serde_json::to_writer_pretty(&mut out, &e.envelope());
            let _ = writeln!(out);
            ExitCode::from(2)
        }
    }
}
fn bad(s: impl Into<String>) -> OperationalError {
    OperationalError {
        code: "INVALID_ARGUMENTS",
        message: s.into(),
        path: None,
    }
}
fn run() -> Result<(serde_json::Value, u8), OperationalError> {
    let mut args = std::env::args().skip(1);
    let (mut target, mut prompt, mut rollout, mut count, mut window, mut max_tokens) =
        (None, None, None, None, None, None);
    let (mut unknown, mut structural) = (false, false);
    let mut maps = Vec::new();
    let mut baselines = Vec::new();
    let mut reserve = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" | "-h" => {
                println!(
                    "USAGE: skill-md-clarifier-validator TARGET [--host-prompt FILE | --rollout FILE] --expected-skills N (--context-window TOKENS | --unknown-context) [--max-context-tokens N] [--map SOURCE=RUNTIME] [--plan-new NAME]\n       skill-md-clarifier-validator TARGET --structure-only\n       skill-md-clarifier-validator TARGET --baseline-skill SOURCE=BEFORE [--baseline-skill SOURCE=BEFORE ...]\n\nDefault: missing catalog is a finding, never a host-budget pass. Baseline mode proves only nonincreasing descriptions for unchanged skill identities; supply one pre-edit source copy per audited skill and preserve catalog conditions. It does not measure host fit. Snapshot must be complete for the intended host/cwd; do not use skills/list as proof. Context is the resolved window, not the smaller usable window. Exit 0: selected scope passes; 1: findings; 2: invalid or unavailable input."
                );
                std::process::exit(0)
            }
            "--version" => {
                println!("skill-md-clarifier-validator {}", env!("CARGO_PKG_VERSION"));
                std::process::exit(0)
            }
            "--host-prompt" => {
                prompt = Some(args.next().ok_or_else(|| bad("Missing prompt path"))?)
            }
            "--rollout" => rollout = Some(args.next().ok_or_else(|| bad("Missing rollout path"))?),
            "--expected-skills" => {
                count = Some(
                    args.next()
                        .ok_or_else(|| bad("Missing skill count"))?
                        .parse::<usize>()
                        .map_err(|_| bad("Invalid skill count"))?,
                )
            }
            "--context-window" => {
                window = Some(
                    args.next()
                        .ok_or_else(|| bad("Missing context window"))?
                        .parse::<usize>()
                        .map_err(|_| bad("Invalid context window"))?,
                )
            }
            "--max-context-tokens" => {
                max_tokens = Some(
                    args.next()
                        .ok_or_else(|| bad("Missing configured budget"))?
                        .parse::<usize>()
                        .map_err(|_| bad("Invalid configured budget"))?,
                )
            }
            "--unknown-context" => unknown = true,
            "--structure-only" => structural = true,
            "--plan-new" => {
                reserve = Some(
                    args.next()
                        .ok_or_else(|| bad("--plan-new requires the future catalog name"))?,
                )
            }
            "--map" => {
                let m = args.next().ok_or_else(|| bad("Missing mapping"))?;
                let (s, r) = m
                    .split_once('=')
                    .ok_or_else(|| bad("Mapping requires SOURCE=RUNTIME"))?;
                maps.push((PathBuf::from(s), PathBuf::from(r)))
            }
            "--baseline-skill" => {
                let m = args.next().ok_or_else(|| bad("Missing baseline mapping"))?;
                let (s, p) = m
                    .split_once('=')
                    .ok_or_else(|| bad("Baseline requires SOURCE=BEFORE"))?;
                baselines.push((PathBuf::from(s), PathBuf::from(p)))
            }
            _ if arg.starts_with('-') => return Err(bad(format!("Unknown option {arg}"))),
            _ => {
                if target.replace(arg).is_some() {
                    return Err(bad("Only one target is allowed"));
                }
            }
        }
    }
    if prompt.is_some() && rollout.is_some() || unknown && window.is_some() {
        return Err(bad("Conflicting evidence options"));
    }
    if structural
        && (prompt.is_some()
            || rollout.is_some()
            || window.is_some()
            || unknown
            || max_tokens.is_some()
            || count.is_some()
            || !maps.is_empty())
    {
        return Err(bad(
            "--structure-only cannot be combined with budget evidence",
        ));
    }
    if !baselines.is_empty()
        && (structural
            || prompt.is_some()
            || rollout.is_some()
            || window.is_some()
            || unknown
            || max_tokens.is_some()
            || count.is_some()
            || !maps.is_empty()
            || reserve.is_some())
    {
        return Err(bad("--baseline-skill is a separate description-change scope; do not combine it with structure-only, host evidence, or planning"));
    }
    let target = target.ok_or_else(|| bad("Target required"))?;
    if reserve.is_some() && (structural || !maps.is_empty() || Path::new(&target).exists()) {
        return Err(bad(
            "--plan-new requires an absent future SKILL.md, complete budget evidence, and no mappings",
        ));
    }
    let mut report = if reserve.is_some() {
        skill_md_clarifier_validator::AuditReport {
            validation_scope: "budget_plan",
            schema_version: skill_md_clarifier_validator::REPORT_SCHEMA_VERSION,
            status: skill_md_clarifier_validator::ReportStatus::Passed,
            target_kind: skill_md_clarifier_validator::TargetKind::StandaloneSkill,
            target: target.clone(),
            findings: vec![],
            skills: vec![],
            context_budget: None,
            description_change: None,
        }
    } else {
        audit_target(Path::new(&target))?
    };
    let content = if let Some(p) = prompt {
        Some(std::fs::read_to_string(p).map_err(|e| bad(e.to_string()))?)
    } else if let Some(p) = rollout {
        let data = std::fs::read_to_string(p).map_err(|e| bad(e.to_string()))?;
        let mut found = None;
        for l in data.lines() {
            let v: serde_json::Value = serde_json::from_str(l).map_err(|e| bad(e.to_string()))?;
            if v["type"] == "response_item" && v["payload"]["role"] == "developer" {
                if let Some(cs) = v["payload"]["content"].as_array() {
                    for c in cs {
                        if let Some(s) = c["text"].as_str() {
                            if s.contains("<skills_instructions>") {
                                found = Some(s.to_string())
                            }
                        }
                    }
                }
            }
        }
        Some(found.ok_or_else(|| bad("No host Skills fragment in this rollout"))?)
    } else {
        None
    };
    if let Some(content) = content {
        if window.is_none() && !unknown {
            return Err(bad(
                "Resolve the model window, or explicitly declare --unknown-context; no silent fallback",
            ));
        }
        let b = budget::Budget::from_context(window, max_tokens).map_err(bad)?;
        budget::apply_with_reservation(
            &mut report,
            &content,
            b,
            count.ok_or_else(|| bad("Expected complete skill count required"))?,
            &maps,
            reserve.as_deref().map(|name| (Path::new(&target), name)),
        )?;
        report.validation_scope = if reserve.is_some() {
            "budget_plan"
        } else {
            "structure_and_host_budget"
        };
    } else if !baselines.is_empty() {
        budget::compare_descriptions(&mut report, &baselines)?;
        report.validation_scope = "structure_and_description_change";
    } else if !structural {
        budget::require_catalog(&mut report)
    }
    let exit = report.exit_code() as u8;
    Ok((
        serde_json::to_value(report).map_err(|e| bad(e.to_string()))?,
        exit,
    ))
}
