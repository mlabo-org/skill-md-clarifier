use serde_json::Value;
use std::{fs, path::Path, process::Command};

fn skill(root: &Path, name: &str, description: &str) {
    fs::create_dir_all(root.join("agents")).unwrap();
    fs::write(root.join("SKILL.md"), format!("---\nname: {name}\ndescription: {description}\n---\nThis is a local execution contract.\n")).unwrap();
    fs::write(root.join("agents/openai.yaml"), format!("interface:\n  display_name: Audit fixture\n  short_description: Audit fixture routing\n  default_prompt: Apply ${name} to audit fixture routing\n")).unwrap();
}

fn run(args: &[&str]) -> (i32, Value) {
    let output = Command::new(env!("CARGO_BIN_EXE_skill-md-clarifier-validator"))
        .args(args).output().unwrap();
    let bytes = if output.stderr.is_empty() { &output.stdout } else { &output.stderr };
    (output.status.code().unwrap(), serde_json::from_slice(bytes).unwrap())
}

#[test]
fn description_comparison_admits_only_a_nonincreasing_existing_identity() {
    let dir = tempfile::tempdir().unwrap();
    let before = dir.path().join("before");
    let source = dir.path().join("source");
    skill(&before, "sample", "Audit 日本語の経路と入力条件。");
    skill(&source, "sample", "Audit 日本語の経路。");
    let target = source.join("SKILL.md").display().to_string();
    let mapping = format!("{target}={}", before.join("SKILL.md").display());
    let args = [&target, "--baseline-skill", &mapping];
    let (code, report) = run(&args);
    assert_eq!(code, 0);
    assert_eq!(report["validationScope"], "structure_and_description_change");
    assert!(report["contextBudget"].is_null());
    assert_eq!(report["descriptionChange"]["hostBudgetStatus"], "not_measured");
    assert_eq!(report["descriptionChange"]["nonincreasing"], true);

    // Fewer UTF-8 bytes alone must not admit growth in the unknown-window unit.
    skill(&before, "sample", "日本語の経路");
    skill(&source, "sample", "Audit paths");
    let (code, report) = run(&args);
    assert_eq!(code, 1);
    assert_eq!(report["findings"][0]["code"], "DESCRIPTION_CHANGE_REQUIRES_HOST_BUDGET");

    skill(&source, "renamed", "短縮");
    assert_eq!(run(&args).0, 1);
    assert_eq!(run(&[&target, "--baseline-skill", &format!("{target}={target}")]).0, 2);
    fs::remove_file(before.join("SKILL.md")).unwrap();
    assert_eq!(run(&args).0, 2);
}

#[test]
fn source_scope_does_not_turn_unverified_or_overflowing_host_budget_into_a_pass() {
    let dir = tempfile::tempdir().unwrap();
    skill(dir.path(), "sample", "Audit 日本語の経路。");
    let target = dir.path().join("SKILL.md").display().to_string();
    assert_eq!(run(&[&target, "--structure-only"]).0, 0);
    let (code, report) = run(&[&target]);
    assert_eq!(code, 1);
    assert_eq!(report["findings"][0]["code"], "CONTEXT_BUDGET_UNVERIFIED");
    let prompt = dir.path().join("host.txt");
    fs::write(&prompt, format!("### Available skills\n- sample: Audit (file: {target})\n")).unwrap();
    let (code, report) = run(&[&target, "--host-prompt", prompt.to_str().unwrap(), "--expected-skills", "1", "--context-window", "1"]);
    assert_eq!(code, 1);
    assert_eq!(report["contextBudget"]["fitsWithoutTruncation"], false);
    assert_eq!(report["findings"][0]["code"], "CONTEXT_BUDGET_EXCEEDED");
}
