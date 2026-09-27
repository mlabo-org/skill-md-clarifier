# Description Comparison

Read this before authoring an existing description change selected for nonincreasing comparison under [Catalog Impact And Budget](../SKILL.md#catalog-impact-and-budget). This scope does not measure whole-host fit.

Invoke the materialized validator from the skill root with one exact pre-edit file per audited skill:

```bash
./bin/darwin-arm64/skill-md-clarifier-validator <source-target> --baseline-skill /absolute/source/SKILL.md=/absolute/task-temp/before-SKILL.md
```

Repeat `--baseline-skill` for every enumerated skill in plugin mode. The native comparison checks unchanged names and nonincreasing normalized description costs in both UTF-8 bytes and Unicode scalar counts. A missing, duplicate, out-of-target, or same-file baseline fails; a name change or increase requires whole-host evidence. The caller establishes that each baseline is the exact pre-edit source and that the catalog conditions in Catalog Impact And Budget are unchanged. When the resolved host is Claude Code, the compared text is the normalized `description` plus `when_to_use`, and each part must be nonincreasing on its own. `contextBudget: null` and `hostBudgetStatus: not_measured` remain explicit: comparison success proves only the source contribution did not increase.


