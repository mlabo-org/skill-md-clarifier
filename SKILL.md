---
name: skill-md-clarifier
description: "Audit/edit SKILL.md or plugin skills: 解釈ぶれ, routing, context-budget warnings, metadata, and ownership. Native validator."
---

# SKILL.md Clarifier

This `SKILL.md` is the local execution contract for this skill when the skill is selected.
Codex must treat this file's trigger assumptions, workflow, tool boundaries, file boundaries, and output shape as binding instructions within this skill's scope.
This file does not override system instructions, developer instructions, explicit user requests, applicable `AGENTS.md` files, or more specific local execution contracts.

## Overview

Audit or edit exactly one standalone `SKILL.md` or one local plugin source root containing skill contracts.
This skill owns `SKILL.md`-specific routing, target resolution, metadata alignment, tool and file boundaries, target-domain production-logic leakage detection, and the audit output shape.
The bundled native validator owns deterministic routing, metadata, description-change comparison, and whole-host catalog budget admission. Select the applicable scope in Catalog Impact And Budget before authoring.

## Shared Clarity Criteria

Before authoring or auditing, read [指示文の解釈を揃える基準](references/instruction-clarity.md). This skill owns that single source resource; `agents-md-clarifier` also consumes it. Keep shared criteria there and skill-specific application here.

Apply them to the selected skill's first action, required inputs, branch and fallback conditions, tool/producer ownership, complete output, handoff, and end state. Keep user-required actions binding, including instructions phrased as "推奨"; preserve only explicitly granted choices. In edit mode, use these criteria as authoring inputs before constructing the complete candidate. Final clarification is read-only: combine the native receipt with the semantic decision on those final bytes, without editing during that decision. Any failed admitted decision follows the applicable AGENTS.md recovery rule, not a routine rewrite pass.

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

Treat a path under `/Users/suzukimakoto/.claude/plugins/cache/` as generated plugin runtime cache.

- A review may cite observed cache bytes, but an edit must resolve the authoritative plugin source first.
- Do not copy a cache skill into `/Users/suzukimakoto/.claude/skills/` under the same identity.
- For an explicitly requested experiment, use a separate non-discovery temporary directory, or create a distinctly named standalone skill only when the user authorizes that new source identity.
- Do not create a source-to-runtime copy or refresh workflow for a standalone skill. Its directory under `/Users/suzukimakoto/.claude/skills/` is both source and USER-scope discovery entry.

## Native Validator

Invoke the materialized binary directly, with one standalone skill root, one `SKILL.md`, or one plugin root:

```text
<skill-md-clarifier-root>/bin/darwin-arm64/skill-md-clarifier-validator
```

Select the scope in Catalog Impact And Budget before authoring or invoking. From this skill root, the minimum commands are:

```bash
# Unchanged catalog contribution
./bin/darwin-arm64/skill-md-clarifier-validator <source-target> --structure-only
# Existing description comparison; read Description Comparison first
./bin/darwin-arm64/skill-md-clarifier-validator <source-target> --baseline-skill /absolute/source/SKILL.md=/absolute/task-temp/before-SKILL.md
# Whole-host decision; read Whole-Host Catalog Budget first
./bin/darwin-arm64/skill-md-clarifier-validator <source-target> --rollout <target-host-rollout> --expected-skills <complete-host-count> --context-window <resolved-model-window>
```

The validator owns deterministic target resolution, plugin skills-path containment and enumeration, frontmatter, the individual format limit, whole-host budget, generic description openings, execution-contract declaration presence, cache-only targets, and adjacent `agents/openai.yaml` structure. Without a scope flag it uses whole-host mode, where missing required host evidence is an error. Structure-only does not establish host fit or admit a new skill; comparison proves only nonincreasing source contribution.

Exit status: `0` means deterministic checks passed, `1` means findings, and `2` means usage, target, read, or parse failure. Findings and failures are emitted as JSON. If the binary is missing or native source changed, read [Native Validator Materialization](references/validator-build.md) and materialize it from source; prose-only edits do not require rebuilding.

For a review-only request, run the selected scope on the current target and incorporate its receipt. For edits, select the scope and producer inputs before construction, then run its final invocation once. A pre-production capacity measurement is needed only when the selected change requires a whole-host allocation and applicable evidence is not already available.

## Catalog Impact And Budget

Select the needed decision from the requested result and actual changed surfaces:

- **Catalog contribution unchanged:** body, script, reference, asset, or UI-support edits with unchanged catalog name, description, locator, membership, root aliases, renderer, model window, and configured budget use structure-only validation plus the final semantic decision. Keep accurate routing metadata unchanged. Do not require host/model discovery, complete-catalog collection, allocation, or a repeated total measurement for that source change.
- **Existing description changed, other catalog conditions unchanged:** bind the original meaning and exact pre-edit descriptions before authoring. A candidate constrained to nonincreasing bytes and scalar counts can use the native baseline comparison, without collecting the whole host or allocating again. Preserve the original files only as task-temporary comparison data outside discovery, then remove the task-created copies. Before authoring for this scope, read [Description Comparison](references/description-comparison.md). This route neither proves nor clears an existing or unknown whole-host overflow. If growth is needed, select whole-host allocation before authoring that growth; do not use the comparison route for name/path/registration or catalog changes.
- **Whole-host decision:** a new skill, increased contribution, changed catalog identity/registration/list/root layout, changed renderer/model window/budget, or a request to prove or remove host truncation requires evidence for the affected host conditions. Resolve only the changed or missing inputs. Reuse an existing measurement when its host/cwd, catalog membership, full metadata, locator/alias layout, renderer, model window, override, and source/runtime mapping still match. An unchanged source stage does not require a second measurement. Before authoring or measuring for this scope, read [Whole-Host Catalog Budget](references/catalog-budget.md). Old numerical limits or another host's result do not establish current fit.

Meaning preservation is mandatory for every compression. Before authoring, bind the original task meaning, scope, applicable and excluded cases, distinctions from neighboring skills, required invocation terms in their original language, and strength of constraints as producer inputs. Never satisfy the budget by losing, weakening, broadening, or changing those meanings; never translate Japanese invocation terms away to save bytes. Move workflow detail, examples, or repeated explanation into the body or existing references only when selection-critical meaning remains available in the pre-load description. A distinction restored only after loading the body does not repair an ambiguous routing description. If a complete meaning-preserving description cannot fit, report the capacity blocker and leave the affected original unchanged; do not force a shorter candidate.

## Audit And Edit Workflow

1. Resolve the target and confirm its source/cache boundary.
2. Read the target `SKILL.md` completely. In plugin mode, read the manifest and every recursively enumerated contained `SKILL.md`.
3. Identify the skill's task class, natural-language triggers, inputs, complete outputs, primary tool or workflow, non-use boundary, and stop conditions.
4. In edit mode, supply the selected Catalog Impact And Budget scope, its required original descriptions or capacity evidence, and Shared Clarity Criteria before constructing the complete authorized source candidate. Change only the resolved source target, in-scope supporting resources, and directly stale adjacent metadata. Do not change target-domain artifact semantics unless the request includes that owning source. In review mode, keep the current target unchanged.
5. Run and incorporate the native validator receipt for the current review target or completed edit candidate as specified in Native Validator.
6. Check the semantic surfaces the validator cannot decide, read-only on those same bytes:
   - frontmatter/body promise alignment
   - strongest trigger terms appearing early enough to route before body load
   - alignment across `SKILL.md`, `agents/openai.yaml`, plugin metadata, and exposed tool descriptions
   - `tool_search` terms, aliases, product names, and failure words needed for discovery
   - workflow and first-action clarity under Shared Clarity Criteria, including concrete input-to-output decisions and any interpretation-changing missing relation
   - exact tool, browser, file, source/cache, external-effect, and stop boundaries
   - target-domain production rules and their executable enforcement points
   - concrete conflicts with applicable higher authority, without copying that authority into the skill
   - preservation of policy strength, permitted discretion, exception scope, and completion boundaries when editing
   - for compression, compare the final description and any relocated content with the original semantic inputs: no lost or changed task meaning, applicability, exclusions, neighboring-skill distinctions, invocation terms, or constraint strength; fitting the numeric budget alone cannot pass this decision. This is part of this same semantic acceptance, not an added review or rewrite loop.
7. Report the completed result in the required output shape. If the target cannot be resolved or safely edited, return a typed stop result instead of inventing a target or enforcement point.

## Routing And Metadata Alignment

Treat frontmatter `description` as the pre-load routing surface and `agents/openai.yaml` as concise UI/routing support, not the execution contract.

- Put the task class and primary object first.
- Keep aliases, product names, tool names, file types, and failure terms that materially drive selection near the beginning.
- Keep workflow detail and non-trigger instructions in the body.
- Ensure the body implements every capability promised by the description.
- Ensure `interface.short_description` and `interface.default_prompt` share the core trigger terms without copying the body.
- Ensure `interface.default_prompt` names the skill explicitly.
- Apply Catalog Impact And Budget to the changed routing surface. Individual validity or a nonincreasing contribution cannot substitute for whole-host fit. Preserve Japanese and other actual invocation terms while removing non-routing detail.

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
- source-grounded replacement wording, the executable owner required, or the unresolved policy decision under the shared criteria's output branch
- native-validator receipt and any unavailable semantic evidence
- selected evidence scope and source/runtime distinction; for whole-host measurement include host/cwd, units, limit, full cost/count, and remaining capacity or excess; for baseline comparison include the pre-edit identity, unchanged catalog conditions, and native delta receipt. Absent whole-host evidence is unverified, never warning-free.

For an interpretation finding, use those existing fields to state the concrete condition, source location, competing actions or missing relation, and the meaning preserved by the proposed rewrite. When the policy is unresolved, put that decision in `tighterRewrite` instead of supplying invented conditions. Separate document-based decision evidence from actual model execution evidence; the native validator does not measure model comprehension or guarantee behavior across models.

For an edit, return:

- changed authoritative paths
- standalone or plugin-root target kind
- plugin-contained `SKILL.md` paths enumerated in plugin mode
- execution-contract declaration state
- routing/body/metadata/boundary defects fixed
- target-domain production rules moved, enforced, or still prose-only
- `agents/openai.yaml` state
- native-validator command, status, and materialized binary path
- selected catalog-impact scope, evidence reused or measured, and whole-host state separately from source-change completion
- typed stop reason when completion is impossible

When invoked through `local-capability-manifest.json`, serialize the result against `skill-md-clarifier/audit-edit-skill-md-output/v2`. Use its `completed` and `stopped` branches exactly; do not coerce a stop into a completed result.
