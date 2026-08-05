# skill-md-clarifier

`skill-md-clarifier` audits standalone and plugin-contained `SKILL.md` files as
routing and execution contracts.

## What it audits

- frontmatter routing strength, budget, and truncation resilience;
- `SKILL.md`, `agents/openai.yaml`, plugin metadata, and tool-description drift;
- plugin-contained skill coverage and source/cache boundaries;
- production decision logic that lacks executable enforcement;
- producer ownership, verification topology, reasoning, workflow, and stop
  conditions;
- `tool_search` misses, over-triggering, under-triggering, and scope leakage.

The skill keeps UI metadata concise and leaves artifact validity in executable
source, schemas, validators, compilers, state machines, or tests.

## Installation

Clone the public repository into the Codex standalone-skill directory:

```bash
git clone https://github.com/mlabo-org/skill-md-clarifier.git ~/.codex/skills/skill-md-clarifier
```

Restart Codex or begin a new task after installation. For source development,
edit the Git checkout and treat installed/runtime copies as derived artifacts.

## Usage

```text
Use $skill-md-clarifier to audit this SKILL.md and its routing metadata.
```

The skill resolves one standalone skill or one plugin root, reads the selected
contract and adjacent metadata, applies only the requested edit or review
scope, and reports enforcement and validation boundaries.

## Repository layout

```text
.
├── SKILL.md
├── agents/
│   └── openai.yaml
├── LICENSE
└── README.md
```

## License

MIT
