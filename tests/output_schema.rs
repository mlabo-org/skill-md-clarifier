use jsonschema::validator_for;
use serde_json::{Value, json};

fn output_schema() -> Value {
    serde_json::from_str(include_str!(
        "../schemas/local-capability/audit-edit-skill-md-output.schema.json"
    ))
    .expect("output schema must be valid JSON")
}

#[test]
fn host_budget_evidence_requires_count_and_one_window_decision() {
    let schema: Value = serde_json::from_str(include_str!(
        "../schemas/local-capability/audit-edit-skill-md-input.schema.json"
    ))
    .unwrap();
    let validator = validator_for(&schema).unwrap();
    let mut request = json!({"target":{"skillPath":"/skills/sample/SKILL.md"},"operation":"audit","budgetEvidence":{"hostPromptPath":"/tmp/host.txt","expectedSkills":94,"contextWindow":272000}});
    assert!(validator.is_valid(&request));
    request["budgetEvidence"]["unknownContext"] = json!(true);
    assert!(!validator.is_valid(&request));
    request["budgetEvidence"]
        .as_object_mut()
        .unwrap()
        .remove("contextWindow");
    assert!(validator.is_valid(&request));
    request["budgetEvidence"]
        .as_object_mut()
        .unwrap()
        .remove("expectedSkills");
    assert!(!validator.is_valid(&request));
}

fn completed_result() -> Value {
    json!({
        "resultStatus": "completed",
        "operation": "audit",
        "findings": [],
        "changes": [],
        "metadata": {
            "targetKind": "standalone_skill",
            "executionContractDeclaration": "already_present",
            "agentsOpenaiYaml": "already_aligned",
            "pluginSkills": []
        },
        "enforcement": [{
            "status": "enforced",
            "ruleOrConcern": "routing budget",
            "enforcementPoint": "bin/darwin-arm64/skill-md-clarifier-validator"
        }],
        "validation": {
            "authoritativeSkillRead": true,
            "targetResolved": true,
            "targetRead": true,
            "checks": [{"name": "native-validator", "status": "passed"}]
        }
    })
}

#[test]
fn completed_and_stopped_results_validate() {
    let schema = output_schema();
    let validator = validator_for(&schema).expect("compile output schema");
    let completed = completed_result();
    assert!(validator.is_valid(&completed));

    let stopped = json!({
        "resultStatus": "stopped",
        "operation": "edit",
        "stopReason": "The requested target is available only in plugin cache.",
        "changes": [],
        "enforcement": [],
        "validation": {
            "authoritativeSkillRead": true,
            "targetResolved": false,
            "targetRead": false,
            "checks": [{"name": "target-resolution", "status": "unverified", "detail": "source unresolved"}]
        }
    });
    assert!(validator.is_valid(&stopped));
}

#[test]
fn schema_rejects_unproved_completed_results() {
    let schema = output_schema();
    let validator = validator_for(&schema).expect("compile output schema");

    let mut missing_operation = completed_result();
    missing_operation
        .as_object_mut()
        .expect("object")
        .remove("operation");
    assert!(!validator.is_valid(&missing_operation));

    let mut missing_enforcement_point = completed_result();
    missing_enforcement_point["enforcement"][0]
        .as_object_mut()
        .expect("enforcement item")
        .remove("enforcementPoint");
    assert!(!validator.is_valid(&missing_enforcement_point));

    let mut standalone_with_plugin_skills = completed_result();
    standalone_with_plugin_skills["metadata"]["pluginSkills"] = json!(["skills/a/SKILL.md"]);
    assert!(!validator.is_valid(&standalone_with_plugin_skills));

    let mut audit_with_changes = completed_result();
    audit_with_changes["changes"] = json!([{"path": "SKILL.md", "summary": "changed"}]);
    assert!(!validator.is_valid(&audit_with_changes));
}
