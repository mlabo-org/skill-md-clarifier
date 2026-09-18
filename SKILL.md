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

Invoke it directly with one standalone skill root, one `SKILL.md`, or one plugin root. For whole-host budget admission:

```bash
./bin/darwin-arm64/skill-md-clarifier-validator <source-target> --rollout <target-host-rollout> --expected-skills <complete-host-count> --context-window <resolved-model-window>
```

The validator owns deterministic checks for target kind, plugin manifest skills-path containment and recursive enumeration, YAML frontmatter parsing, the individual format limit, whole-host metadata budget, generic description openings, execution-contract declaration presence, cache-only targets, and adjacent `agents/openai.yaml` structure. Missing host budget evidence is an error in the default whole-host scope.

Use `--structure-only` for changes that leave the catalog contribution unchanged. Its receipt plus the final semantic decision can complete that source change; it does not establish current whole-host fit or admit a new skill. For an existing description change under unchanged catalog conditions, use the separate comparison scope with one exact pre-edit file per audited skill:

```bash
./bin/darwin-arm64/skill-md-clarifier-validator <source-target> --baseline-skill /absolute/source/SKILL.md=/absolute/task-temp/before-SKILL.md
```

Repeat `--baseline-skill` for every enumerated skill in plugin mode. The native comparison checks unchanged names and nonincreasing normalized description costs in both UTF-8 bytes and Unicode scalar counts. A missing, duplicate, out-of-target, or same-file baseline fails; a name change or increase requires whole-host evidence. The caller establishes that each baseline is the exact pre-edit source and that the catalog conditions in Catalog Impact And Budget are unchanged. `contextBudget: null` and `hostBudgetStatus: not_measured` remain explicit: comparison success proves only the source contribution did not increase.

Exit status is part of the contract:

- `0`: deterministic checks passed
- `1`: deterministic findings were emitted as JSON
- `2`: usage, target resolution, read, or parse failure was emitted as JSON

Do not use `cargo run` as the normal route. If the materialized binary is absent or its native source changed, materialize it from this skill root using a new task-owned temporary build directory. Confirm the manifest owner and exact target directory in the metadata output before building or cleaning:

```bash
clarifier_manifest="$PWD/Cargo.toml"
clarifier_build_dir="$(mktemp -d "${TMPDIR:-/tmp}/skill-md-clarifier-build.XXXXXX")"
CARGO_TARGET_DIR="$clarifier_build_dir" cargo metadata --locked --no-deps --format-version 1 --manifest-path "$clarifier_manifest"
cargo build --locked --release --manifest-path "$clarifier_manifest" --target-dir "$clarifier_build_dir"
mkdir -p bin/darwin-arm64
install -m 755 "$clarifier_build_dir/release/skill-md-clarifier-validator" bin/darwin-arm64/skill-md-clarifier-validator
cargo clean --manifest-path "$clarifier_manifest" --target-dir "$clarifier_build_dir"
rmdir "$clarifier_build_dir"
```

If construction fails or is interrupted, clean that same verified temporary target with the shown `cargo clean` command and remove its empty task-created directory; preserve the stable binary and pre-existing build directories. The example is a materialization route, not an instruction to build for prose-only edits.

For a review-only request, run the selected scope on the current target and incorporate its receipt. For edits, select the scope and producer inputs before construction, then run its final invocation once. A pre-production capacity measurement is needed only when the selected change requires a whole-host allocation and applicable evidence is not already available.

## Catalog Impact And Budget

The executable calculation follows [Codex rust-v0.153.4 render.rs](https://github.com/openai/codex/blob/rust-v0.153.4/codex-rs/ext/skills/src/render.rs) and the [official skills documentation](https://learn.chatgpt.com/docs/build-skills). The initial catalog, not each file separately, shares the budget. The validator uses per-line UTF-8 byte costs for token budgets, Unicode scalar counts for the unknown-window fallback, and the captured host alias table and its overhead. It counts complete descriptions, names, paths, separators, and newlines. It does not use an English/Japanese character multiplier as a substitute for counting the actual text. Version-pinned named constants in `src/budget.rs` own the shared-budget percentage, explicit cap, unknown-context fallback, UTF-8 token approximation, and individual format limit. The model context window is a runtime input, never a fixed model-size constant. Update those constants against the official source when adopting a changed specification; do not duplicate numeric rules in prose.

Select the needed decision from the requested result and actual changed surfaces:

- **Catalog contribution unchanged:** body, script, reference, asset, or UI-support edits with unchanged catalog name, description, locator, membership, root aliases, renderer, model window, and configured budget use structure-only validation plus the final semantic decision. Keep accurate routing metadata unchanged. Do not require host/model discovery, complete-catalog collection, allocation, or a repeated total measurement for that source change.
- **Existing description changed, other catalog conditions unchanged:** bind the original meaning and exact pre-edit descriptions before authoring. A candidate constrained to nonincreasing bytes and scalar counts can use the native baseline comparison, without collecting the whole host or allocating again. Preserve the original files only as task-temporary comparison data outside discovery, then remove the task-created copies. This route neither proves nor clears an existing or unknown whole-host overflow. If growth is needed, select whole-host allocation before authoring that growth; do not use the comparison route for name/path/registration or catalog changes.
- **Whole-host decision:** a new skill, increased contribution, changed catalog identity/registration/list/root layout, changed renderer/model window/budget, or a request to prove or remove host truncation requires evidence for the affected host conditions. Resolve only the changed or missing inputs. Reuse an existing measurement when its host/cwd, catalog membership, full metadata, locator/alias layout, renderer, model window, override, and source/runtime mapping still match. An unchanged source stage does not require a second measurement. Old numerical limits or another host's result do not establish current fit.

For the whole-host decision:

1. Resolve the target host, cwd, CLI renderer version, and actual resolved model context window from applicable current metadata/configuration or still-valid evidence. The smaller usable window reported by token usage is not the resolved window. A configured skills token override is supplied only when it actually exists, using `--max-context-tokens`. Use `--unknown-context` only when the window cannot be resolved; do not silently replace a known window with the fallback or alter runtime settings to obtain a pass.
2. Obtain or reuse the actual host Skills fragment with its root aliases, names, and paths. The native `--rollout` reader extracts only the latest developer Skills fragment from the exact authorized local session; `--host-prompt` accepts an already captured fragment. Establish completeness for that host and pass `--expected-skills`. A catalog with omitted skills, unknown additional environment/orchestrator catalogs, inaccessible sources, or uncertain completeness cannot receive a whole-host pass. The App Server `skills/list` response alone is insufficient: it can omit plugin-provided entries present in the actual prompt. Do not scan unrelated sessions or copy private conversation contents into source.
3. For an existing skill whose contribution must grow, obtain or reuse `targetDescriptionAllowance` for the unchanged pre-edit catalog. For a new skill, reserve its actual catalog name and future absolute `SKILL.md` path without creating a file:
   ```bash
   ./bin/darwin-arm64/skill-md-clarifier-validator /absolute/new-skill/SKILL.md --plan-new new-skill --host-prompt /absolute/host-skills.txt --expected-skills <count> --context-window <window>
   ```
   Give the producer that available budget in its reported units, the intended host/cwd, and required trigger words before authoring. A zero allowance or existing overflow blocks an unaccommodated addition or increase, not a separately valid body change or proven nonincreasing description change. Shortening just a new skill cannot fix pre-existing overflow. New plugin entries need their actual namespaced names and runtime paths reserved; do not substitute an unqualified source name.
4. The validator rereads full descriptions from the captured files, so it never mistakes already-shortened prompt text for the original cost. For an installed plugin source edit, map each audited source path to its corresponding captured runtime path with repeated `--map SOURCE=RUNTIME`. Do not guess by matching names, double-count the source plus cache, or edit the cache. Measure the final candidate against the same unaffected host evidence; replace only evidence invalidated by changed conditions. Report source admission separately from subsequent authorized materialization and the live host's uptake. Unsupported locator formats fail explicitly.
5. Meaning preservation is mandatory for every compression. Before authoring, bind the original task meaning, scope, applicable and excluded cases, distinctions from neighboring skills, required invocation terms in their original language, and strength of constraints as producer inputs. Never satisfy the budget by losing, weakening, broadening, or changing those meanings; never translate Japanese invocation terms away to save bytes. Move workflow detail, examples, or repeated explanation into the body or existing references only when selection-critical meaning remains available in the pre-load description. A distinction restored only after loading the body does not repair an ambiguous routing description. If a complete meaning-preserving description cannot fit, report the capacity blocker and leave the affected original unchanged; do not force a shorter candidate. `uniformExistingPrefixCharacters`, ASCII, and three-byte-Japanese figures are capacity measurements, not automatic truncation instructions or universal limits.
6. A whole-host pass requires the final complete catalog to fit without description truncation or skill omission and without target findings. Missing evidence, a format-limit violation, or total overflow is not that pass merely because the CLI can still run. Keep whole-host findings visible and report their effect on the requested result: an unresolved host-overflow request remains incomplete, while a valid unchanged-contribution or proven nonincreasing source repair can complete with the environment overflow or unknown state reported separately. Do not hide warnings, change their colors, increase limits, disable skills, or rewrite other skills to force success without corresponding user authority. Never report that a warning disappeared without actual host evidence.

Whole-host measurement supports the captured host filesystem catalog under the pinned renderer semantics. For a changed CLI renderer, root layout, or additional resource catalogs, resolve that difference before claiming exact parity. No fixed per-description length guarantees a fit at every skill count. The official individual format maximum is not a recommended authoring length.

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
