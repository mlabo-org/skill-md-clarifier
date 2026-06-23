---
name: skill-md-clarifier
description: Use when the user wants to create, review, tighten, validate, or refactor a SKILL.md so the skill acts as an explicit execution contract rather than soft guidance. Covers frontmatter description/body alignment and 1024-character limit, trigger and non-trigger clarification, workflow hardening, reasoning and verification contract checks, tool/subagent/file-boundary rules, output-shape tightening, and converting weak skill wording into operational conditions.
---

# SKILL.md Clarifier

This `SKILL.md` is the local execution contract for this skill when the skill is selected.
Codex must treat this file's trigger assumptions, workflow, tool boundaries, file boundaries, and output shape as binding instructions within this skill's scope.
This file does not override system instructions, developer instructions, explicit user requests, applicable `AGENTS.md` files, or more specific local execution contracts.

## Overview

This skill audits `SKILL.md` as a local execution contract for the skill that contains it.
Use it to make a skill easier for Codex to trigger, load, follow, and stop using at the correct boundary.

## Core Rule

Treat every operative `SKILL.md` statement as an execution condition within that skill's scope unless it is clearly explanatory context, an example, or an intentionally non-binding note.
If wording leaves escape hatches such as "consider", "prefer", "as needed", "typically", or "when useful", assume the instruction is weaker than intended unless the surrounding context clearly requires discretion.

## Scope Boundary

`SKILL.md` is not a global constitution. It binds Codex only after the skill is selected or explicitly invoked.

- Do not make a `SKILL.md` claim authority over system instructions, developer instructions, explicit user requests, applicable `AGENTS.md` files, or more specific local execution contracts.
- Do not move global policy into `SKILL.md` unless the policy applies only when this skill is active.
- Do not use `SKILL.md` to create a second `AGENTS.md`.
- Keep plugin-, repo-, or workflow-specific rules in the skill only when they are necessary for that skill to execute correctly.

## Execution-Contract Declaration

When a target `SKILL.md` lacks an equivalent declaration that it is the execution contract for that skill, add one near the start of the body before tightening individual rules.

- Insert after YAML frontmatter and the H1 title if present.
- Do not duplicate an equivalent declaration. Treat "execution contract", "binding instructions", "local operating contract", or "source of truth for this skill" as equivalent when it clearly binds Codex behavior.
- Preserve priority order. The declaration must not claim authority over higher-priority instructions or applicable `AGENTS.md` files.

Use this template unless the target needs narrower wording:

```md
This `SKILL.md` is the local execution contract for this skill when the skill is selected.
Codex must treat this file's trigger assumptions, workflow, tool boundaries, file boundaries, and output shape as binding instructions within this skill's scope.
This file does not override system instructions, developer instructions, explicit user requests, applicable `AGENTS.md` files, or more specific local execution contracts.
```

Japanese template:

```md
この `SKILL.md` は、この skill が選択された場合に適用される局所実行契約である。
Codex は、本書の発火前提、作業手順、ツール境界、ファイル境界、出力形式を、この skill のスコープ内で拘束力のある実行条件として扱う。
本書は、システム指示、開発者指示、ユーザーの明示要求、適用される `AGENTS.md`、より局所の実行契約を上書きしない。
```

## When To Use

Use this skill when the user wants any of the following:

- Create or revise a `SKILL.md` as a Codex-readable execution contract.
- Review a `SKILL.md` for weak wording, trigger ambiguity, workflow gaps, or hard-to-apply rules.
- Audit a skill's operational validity, including whether it applies the applicable `AGENTS.md` reasoning and verification contract.
- Split a skill concept out from an `AGENTS.md` policy or from another skill.
- Clarify when a skill should trigger and when it must not trigger.
- Align a skill's YAML frontmatter `description` with the workflow and limits in its body.
- Tighten tool, browser, subagent, Python, file-edit, plugin/cache, or Git boundaries inside a skill.
- Convert skill prose from human-facing explanation into operational text Codex can execute.

## Target Resolution

Before auditing or editing, resolve exactly one target `SKILL.md`.

Use this order:

1. If the user provides an explicit `SKILL.md` path, use that path.
2. If the user provides a structured skill reference with a `path`, use that path.
3. If the current working directory contains `SKILL.md`, treat `./SKILL.md` as the target.
4. If the user names a skill and its source-of-truth path is known, use that source-of-truth `SKILL.md`.
5. If more than one target is plausible, stop and ask for the target path.

Do not search parent directories for `SKILL.md` as a default. Unlike `AGENTS.md`, a skill file is normally scoped to one skill directory, not inherited through a directory tree.

When the resolved path is under `~/.codex/plugins/cache/`, treat it as a cache copy unless the user has explicitly asked for cache repair. For cache copies:

- review and suggest changes if requested
- do not patch the cache file as the primary edit target
- ask for or locate the source-of-truth plugin/skill directory before editing
- if the user wants an experiment, copy the skill into a local source-of-truth skill directory first

## Audit Workflow

1. Resolve exactly one target `SKILL.md` using the Target Resolution rules.
2. Read the target `SKILL.md` completely, including YAML frontmatter.
3. Identify the skill's intended job, triggering situations, expected inputs, expected outputs, and non-goals.
4. Check frontmatter/body alignment:
   - `description` must include the real trigger conditions because Codex sees it before loading the body.
   - The YAML frontmatter `description` value must be 1024 characters or fewer.
   - The body must support everything the `description` promises.
   - The body must not quietly expand the skill beyond the `description` unless the `description` is updated.
5. Check whether the execution-contract declaration is present. If missing and the user asked for edits, add it before other rewrites. If the user asked for review only, report it as the first finding and include the declaration to add.
6. Isolate behavior-controlling sections: triggers, exclusions, workflow order, tool use, subagent use, file ownership, validation, output shape, and stop conditions.
7. Classify issues:
   - Trigger ambiguity: Codex may fail to select the skill or may select it too broadly.
   - Body mismatch: the body does not fulfill or overextends the frontmatter `description`.
   - Workflow ambiguity: Codex can follow multiple orders or skip a required phase.
   - Boundary ambiguity: tool, file, repo, plugin/cache, or subagent responsibilities are under-specified.
   - Reasoning contract gap: the skill performs high-risk judgment but does not separate premises, evidence, counterevidence or exception checks, unknowns, validation results, and stop conditions.
   - Weak execution wording: the sentence sounds strict to a human but leaves enough discretion for non-action.
   - Scope leak: the skill tries to govern behavior outside its active skill scope.
8. For skills that create, edit, repair, validate, publish, commit, refresh, install, delete, externally submit, or audit source-of-truth files, check that the skill names the evidence it must gather, the validation it must run or report as unavailable, and the conditions that require stopping instead of improvising.
9. Rewrite only as much as needed to preserve the intended skill while making the action path explicit.
10. If editing, patch the target files directly. Also update `agents/openai.yaml` when UI metadata or default prompts become stale.
11. If reviewing only, report findings first with precise file/section references and provide tighter replacement wording.

## SKILL.md-Specific Checks

### Frontmatter `description`

The `description` is the trigger surface. It must be specific enough for Codex to select the skill without reading the body first.
The YAML frontmatter `description` value must be 1024 characters or fewer. Keep trigger-critical terms in the `description`; move workflow detail, examples, and non-trigger operational rules into the body.

Ensure it states:

- what the skill does
- concrete user requests or task contexts that trigger it
- key file types, systems, or workflows it covers
- major exclusions when over-triggering is likely

Do not hide trigger-critical conditions only in the body.

### Body

The body is the execution surface after trigger. Ensure it states:

- first action after trigger
- required files or resources to read
- ordered workflow
- exact tool or runtime boundaries when they matter
- what to edit and what not to edit
- validation requirements
- output shape
- stop or escalation conditions

### Reasoning And Verification Contract

When a skill performs high-risk judgment or modifies source-of-truth files, ensure the skill states how it applies the applicable `AGENTS.md` reasoning contract within the skill's own scope.

Require local handling for:

- root-cause debugging, review, security analysis, source editing, plugin/skill maintenance, hooks, MCP/app settings, browser/tool boundaries, external writes, deletion, persistent data changes, commits, publishing, or runtime/cache refreshes
- claims that depend on logs, tests, generated artifacts, official docs, external pages, user-visible UI state, or repository source files
- workflows where fallback, skip, logging, confidence scoring, persona debate, or subagent output could be mistaken for validation

The skill must define, when relevant:

- premises or required inputs Codex must confirm before acting
- evidence sources such as files, logs, test output, official docs, browser observations, or user-provided material
- counterevidence, exception, or alternate-hypothesis checks
- unknowns that must remain visible instead of being filled by inference
- validation commands, visual checks, readback checks, or explicit unavailable-validation reporting
- stop conditions for unresolved source-of-truth paths, missing permissions, conflicting authorities, failed validation, cache/runtime-only targets, or unsafe side effects

Do not copy the global `AGENTS.md` wholesale into a skill. Add only the skill-specific execution steps needed for Codex to obey the global contract while that skill is active. Do not require a local reasoning section for a skill with no high-risk judgment surface.

### `agents/openai.yaml`

Treat `agents/openai.yaml` as UI metadata, not the execution contract.

- Keep `display_name`, `short_description`, and `default_prompt` aligned with `SKILL.md`.
- Do not put binding execution rules only in `agents/openai.yaml`.
- If `SKILL.md` changes the skill's purpose, update stale UI metadata.

## Wording Rules

Prefer verbs that directly constrain execution:

- Strong: `use`, `do not use`, `treat as`, `must`, `always`, `only when`, `unless`, `first`, `after`, `before`, `prefer X over Y`, `if A, do B`
- Weak: `consider`, `review`, `ideally`, `generally`, `basically`, `as needed`, `if appropriate`, `candidate`, `recommended`, `desirable`

When tightening wording:

- Replace weak modality with explicit triggers and outcomes.
- Replace vague sequencing with ordered steps.
- Replace overlapping responsibilities with named boundaries.
- Replace "may" style language unless the flexibility is intentional and necessary.
- Keep exceptions explicit and narrow.
- Keep `SKILL.md` concise enough to load as operational context.

## What To Preserve

Do not rewrite for style alone.
Preserve the original:

- intended skill scope
- user intent
- priority order
- safety constraints
- local terminology
- established resource paths and scripts
- progressive-disclosure structure

Only strengthen wording when it reduces interpretive drift without changing policy.

## Output Shape

When reviewing, output:

- the issue
- why it weakens execution
- the exact section or line reference
- a tighter rewrite

When editing, make the patch and summarize:

- whether an execution-contract declaration was added or already present
- what frontmatter/body mismatch was fixed
- what trigger, workflow, or boundary ambiguity was removed
- whether `agents/openai.yaml` was updated
- any remaining areas intentionally left flexible

## Special Attention Areas

Pay extra attention to these recurrent failure patterns:

- `description` promises a broader workflow than the body supports
- `description` is too narrow, so the skill will not trigger for its real use case
- `description` grows beyond 1024 characters while trying to include every trigger detail
- body-only trigger rules that Codex cannot see before skill selection
- "soft" verbs that lead to non-action
- instructions that describe intent but not trigger conditions
- high-risk workflows that do not separate premises, evidence, counterevidence, unknowns, validation, and stop conditions
- confidence scores, persona debate, fallback, logging, or subagent output being treated as validation without concrete evidence
- workflows that do not state the first action
- tool or runtime instructions without fallback or stop conditions
- subagent instructions that do not define ownership, context sharing, or output shape
- file-edit instructions that do not define the source-of-truth directory
- skill text that tries to override global `AGENTS.md`
- output requirements that are described as preferences rather than deliverable conditions
