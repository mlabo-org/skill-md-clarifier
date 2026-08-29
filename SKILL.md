---
name: skill-md-clarifier
description: "Audit or edit standalone and plugin-contained SKILL.md routing/truncation, target-domain production-logic leaks, tool/file/cache boundaries, tool_search misses, and agents/openai.yaml metadata drift. Uses the bundled native validator for deterministic checks."
---

# SKILL.md Clarifier

This `SKILL.md` is the local execution contract for this skill when the skill is selected.
Codex must treat this file's trigger assumptions, workflow, tool boundaries, file boundaries, and output shape as binding instructions within this skill's scope.
This file does not override system instructions, developer instructions, explicit user requests, applicable `AGENTS.md` files, or more specific local execution contracts.

## Overview

Audit or edit exactly one standalone `SKILL.md` or one local plugin source root containing skill contracts.
This skill owns `SKILL.md`-specific routing, target resolution, metadata alignment, tool and file boundaries, target-domain production-logic leakage detection, and the audit output shape.
The bundled native validator owns deterministic routing and metadata checks.

## Scope Boundary

`SKILL.md` is not a global constitution. It binds Codex only after the skill is selected or explicitly invoked.

- Read the applicable `AGENTS.md` before auditing or editing and report only concrete conflicts between that authority and the target.
- Do not restate cross-cutting reasoning, acceptance, delegation, Git, or safety policy inside a target `SKILL.md`.
- Do not require a target skill to duplicate higher-authority rules merely to pass this audit.
- Keep only skill-specific triggers, workflow mechanics, tool and file boundaries, validation routes, output contracts, and stop conditions in a target skill.
- Use `agents-md-clarifier`, not this skill, when the target to create or revise is an `AGENTS.md`.

## Target-Domain Production Logic

Detect rules in the target skill that decide the validity or content of the target workflow's production artifact, including compile behavior, decoration or content selection, lint status, freshness, publication readiness, or target-domain gate outcomes.

When such a rule exists only in `SKILL.md`, README text, comments, handoff notes, or UI metadata:

1. Identify the exact target-domain decision and the artifact or completion state it controls.
2. Locate its executable owner in source, schema, validator, compiler, generator, state machine, contract test, or golden test.
3. If no owner exists, report a production-logic leak and the missing enforcement point.
4. When editing is authorized, move the rule only into the authoritative executable owner; keep routing, invocation, and stop instructions in `SKILL.md`.
5. Stop short of declaring the target production-ready while an identified artifact-validity rule remains prose-only.

This boundary concerns the target workflow's production semantics. The clarifier's own audit categories are skill-domain mechanics, and its deterministic subset is implemented by the bundled native validator rather than recalculated from prose.

## When To Use

Use this skill to:

- create or revise a `SKILL.md` as a Codex-readable local execution contract
- review weak triggers, routing-budget problems, truncation risk, workflow gaps, or hard-to-apply wording
- align frontmatter with `agents/openai.yaml`, plugin metadata, or tool descriptions
- diagnose `tool_search` misses, over-triggering, under-triggering, or body-only trigger terms
- audit plugin-contained skills through the plugin manifest's declared skills directory
- separate target-domain production decisions from routing and invocation prose
- tighten skill-specific browser, subagent, Python, file-edit, plugin/cache, or Git mechanics

Do not select this skill for general prose editing, an `AGENTS.md` audit, or an application artifact whose owning workflow has no `SKILL.md` contract in scope.

## Target Resolution

Resolve exactly one target before reading or editing target files:

1. Use an explicit `SKILL.md` path supplied by the user.
2. Otherwise use a structured skill reference containing a path.
3. Otherwise, if the current directory contains `SKILL.md`, use that file.
4. Otherwise use an explicitly named local plugin source root and its `.codex-plugin/plugin.json`.
5. Otherwise use a named skill's verified source-of-truth path.
6. If more than one target remains plausible, stop and ask for the exact target.

Do not search parent directories for a target `SKILL.md` by default.

For a plugin root:

- read `.codex-plugin/plugin.json`
- resolve its string `skills` path as a contained relative directory
- recursively enumerate every `SKILL.md` below that directory, regardless of nesting depth or directory name
- audit every enumerated skill and its adjacent `agents/openai.yaml` when present
- report an empty declared skills directory as a finding rather than silently passing it

### Cache And Experiment Boundary

Treat a path under `/Users/suzukimakoto/.codex/plugins/cache/` as generated plugin runtime cache.

- A review may cite observed cache bytes, but an edit must resolve the authoritative plugin source first.
- Do not copy a cache skill into `/Users/suzukimakoto/.agents/skills/` under the same identity.
- For an explicitly requested experiment, use a separate non-discovery temporary directory, or create a distinctly named standalone skill only when the user authorizes that new source identity.
- Do not create a source-to-runtime copy or refresh workflow for a standalone skill. Its directory under `/Users/suzukimakoto/.agents/skills/` is both source and USER-scope discovery entry.

## Native Validator

Use the materialized validator at:

```text
<skill-md-clarifier-root>/bin/darwin-arm64/skill-md-clarifier-validator
```

Invoke it directly with one standalone skill root, one `SKILL.md`, or one plugin root:

```bash
./bin/darwin-arm64/skill-md-clarifier-validator <skill-root-or-SKILL.md>
```

The validator owns deterministic checks for target kind, plugin manifest skills-path containment and recursive enumeration, YAML frontmatter parsing, description routing budget, generic description openings, execution-contract declaration presence, cache-only targets, and adjacent `agents/openai.yaml` structure.

Exit status is part of the contract:

- `0`: deterministic checks passed
- `1`: deterministic findings were emitted as JSON
- `2`: usage, target resolution, read, or parse failure was emitted as JSON

Do not use `cargo run` as the normal route. If the materialized binary is absent after source changes, stop the runtime claim and materialize it from this skill root:

```bash
cargo build --locked --release
mkdir -p bin/darwin-arm64
install -m 755 target/release/skill-md-clarifier-validator bin/darwin-arm64/skill-md-clarifier-validator
```

For a review-only request, run the validator once on the current target and incorporate its receipt.
For an edit request, inspect the current source directly, construct the complete requested edit, then run the validator once against the final target bytes as the deterministic validation assigned to this skill.

## Audit And Edit Workflow

1. Resolve the target and confirm its source/cache boundary.
2. Read the target `SKILL.md` completely. In plugin mode, read the manifest and every recursively enumerated contained `SKILL.md`.
3. Identify the skill's task class, natural-language triggers, inputs, complete outputs, primary tool or workflow, non-use boundary, and stop conditions.
4. Incorporate the native validator receipt at the point specified above.
5. Check the semantic surfaces the validator cannot decide:
   - frontmatter/body promise alignment
   - strongest trigger terms appearing early enough to route before body load
   - alignment across `SKILL.md`, `agents/openai.yaml`, plugin metadata, and exposed tool descriptions
   - `tool_search` terms, aliases, product names, and failure words needed for discovery
   - unambiguous workflow order and first action
   - exact tool, browser, file, source/cache, external-effect, and stop boundaries
   - target-domain production rules and their executable enforcement points
   - concrete conflicts with applicable higher authority, without copying that authority into the skill
6. When editing is authorized, update only the resolved source target and directly stale adjacent metadata. Do not change target-domain artifact semantics unless the request includes that owning source.
7. Report the completed result in the required output shape. If the target cannot be resolved or safely edited, return a typed stop result instead of inventing a target or enforcement point.

## Routing And Metadata Alignment

Treat frontmatter `description` as the pre-load routing surface and `agents/openai.yaml` as concise UI/routing support, not the execution contract.

- Put the task class and primary object first.
- Keep aliases, product names, tool names, file types, and failure terms that materially drive selection near the beginning.
- Keep workflow detail and non-trigger instructions in the body.
- Ensure the body implements every capability promised by the description.
- Ensure `interface.short_description` and `interface.default_prompt` share the core trigger terms without copying the body.
- Ensure `interface.default_prompt` names the skill explicitly.
- Do not claim that one description alone can prevent corpus-wide skills-budget truncation; active skill count and total metadata volume remain separate facts to report when relevant.

The native validator owns deterministic length, opening, and required-field decisions. Do not duplicate those numeric rules here.

## Editing Boundary

- Add the execution-contract declaration near the start only when an equivalent declaration is absent.
- Preserve the intended skill scope, user intent, priority order, safety constraints, local terminology, and authoritative resource paths.
- Rewrite only what is needed to remove an observed routing, workflow, metadata, boundary, or target-domain enforcement defect.
- Update adjacent `agents/openai.yaml` only when it is missing or stale for the changed capability.
- Never edit plugin cache as the primary source.
- Do not create a second audit, reviewer, or validator for a decision already owned by the native validator or an applicable higher contract.

## Output Shape

For a review, return each finding with:

- issue and category
- why it weakens execution
- exact file and location
- tighter replacement wording or the executable owner required
- native-validator receipt and any unavailable semantic evidence

For an edit, return:

- changed authoritative paths
- standalone or plugin-root target kind
- plugin-contained `SKILL.md` paths enumerated in plugin mode
- execution-contract declaration state
- routing/body/metadata/boundary defects fixed
- target-domain production rules moved, enforced, or still prose-only
- `agents/openai.yaml` state
- native-validator command, status, and materialized binary path
- typed stop reason when completion is impossible

When invoked through `local-capability-manifest.json`, serialize the result against `skill-md-clarifier/audit-edit-skill-md-output/v2`. Use its `completed` and `stopped` branches exactly; do not coerce a stop into a completed result.
