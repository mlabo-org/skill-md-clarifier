# skill-md-clarifier

`skill-md-clarifier` is a Codex skill for tightening `SKILL.md` files so they behave like local execution contracts instead of loose guidance.

It focuses on the failure mode that appears when a skill exists, but Codex still has room to mis-trigger it, skip parts of it, or treat its workflow as optional:

> "The skill says what it is for, but does Codex know exactly when to use it and how to follow it?"

This skill audits and rewrites `SKILL.md` with explicit trigger conditions, workflow order, tool boundaries, file boundaries, validation requirements, and output shapes.

## What It Does

- Aligns YAML frontmatter `description` with the body.
- Adds a local execution-contract declaration when missing.
- Clarifies when the skill should trigger and when it must not trigger.
- Converts weak wording such as "consider", "as needed", or "if appropriate" into explicit operating conditions.
- Tightens workflow order, tool boundaries, subagent boundaries, file-edit boundaries, and output requirements.
- Keeps `agents/openai.yaml` as UI metadata, not as the execution contract.
- Preserves the intended skill scope while reducing interpretive drift.

## Repository Layout

```text
.
├── SKILL.md
├── agents/
│   └── openai.yaml
├── LICENSE
└── README.md
```

## Installation

Clone this repository into your Codex skills directory:

```bash
git clone https://github.com/mlabo-org/skill-md-clarifier.git ~/.codex/skills/skill-md-clarifier
```

Then restart Codex or open a new session so the skill list is refreshed.

## Usage

Ask Codex to use the skill against a target `SKILL.md`:

```text
Use $skill-md-clarifier to review this SKILL.md and tighten weak execution wording.
```

For direct editing:

```text
Use $skill-md-clarifier to align the description with the body and rewrite this SKILL.md as a local execution contract.
```

## Core Idea

A `SKILL.md` is read by Codex after the skill is selected. It should not be written like human-facing documentation alone.

An effective `SKILL.md` needs:

- visible trigger conditions in frontmatter
- a body that fulfills the frontmatter promise
- a clear scope boundary
- an explicit first action
- deterministic workflow order
- narrow exceptions
- clear tool, file, and subagent ownership
- observable output and validation conditions

Without those, Codex can treat skill text as optional guidance or fill missing decisions from context.

## License

MIT
