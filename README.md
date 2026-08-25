# skill-md-clarifier

[日本語](#日本語) | [English](#english)

## 日本語

`skill-md-clarifier` は、単体スキルおよびプラグイン内の `SKILL.md` を、
ルーティングと実行契約の両面から監査し、必要な範囲で改訂するスキルです。

### 監査・改訂する内容

- frontmatter のルーティング強度、文字数予算、切り詰め耐性
- `SKILL.md`、`agents/openai.yaml`、プラグインメタデータ、ツール説明のずれ
- プラグイン内スキルの網羅性と、ソース・キャッシュ・実行環境の境界
- 実行可能な強制箇所を持たない production decision logic
- producer ownership、verification topology、推論、ワークフロー、停止条件
- `tool_search` の未検出、過剰発火、発火不足、スコープ漏れ

UI メタデータは簡潔に保ち、成果物の有効性に関する判断は、ソース、スキーマ、
validator、compiler、state machine、またはテストなどの実行可能な箇所に置きます。

### 正本、ローカル実行版、GitHub

この Git checkout がメンテナ開発の正本です。ローカルで有効な Codex スキルと
GitHub の `origin` は、どちらもこの正本から保守し、別々の実装ではなく、同等の
スキルとして保ちます。

変更はこの checkout で行い、同じ Git 状態をローカル実行版へ refresh して
GitHub へ反映します。インストール済みコピーや実行コピーを独立した正本として
直接編集しません。

### インストール

通常の直接インストールでは、公開リポジトリを Codex の単体スキルディレクトリへ
clone します。

```bash
git clone https://github.com/mlabo-org/skill-md-clarifier.git ~/.agents/skills/skill-md-clarifier
```

インストール後は Codex を再起動するか、新しいタスクを開始してください。この
方式では、clone した Git checkout がそのままローカルで有効なスキルディレクトリに
なります。

### 使い方

```text
$skill-md-clarifier を使って、この SKILL.md を監査・改訂し、ルーティングメタデータを整合させてください。
```

このスキルは、1つの単体スキルまたは1つのプラグインルートを解決し、対象の実行契約と
隣接メタデータを読み、依頼された編集またはレビューの範囲だけを扱い、実装上の強制箇所と
検証境界を報告します。

### リポジトリ構成

```text
.
├── SKILL.md
├── agents/
│   └── openai.yaml
├── LICENSE
└── README.md
```

### ライセンス

MIT

## English

`skill-md-clarifier` audits and revises standalone and plugin-contained
`SKILL.md` files as routing and execution contracts.

### What it audits and revises

- frontmatter routing strength, budget, and truncation resilience
- drift across `SKILL.md`, `agents/openai.yaml`, plugin metadata, and tool descriptions
- plugin-contained skill coverage and source/cache/runtime boundaries
- production decision logic that lacks executable enforcement
- producer ownership, verification topology, reasoning, workflow, and stop conditions
- `tool_search` misses, over-triggering, under-triggering, and scope leakage

The skill keeps UI metadata concise and leaves artifact-validity decisions in
executable source, schemas, validators, compilers, state machines, or tests.

### Source of truth, local runtime, and GitHub

This Git checkout is the authoritative source for maintainer development. The
locally active Codex skill and the GitHub `origin` are both maintained from this
source so that they remain equivalent versions of the skill rather than separate
implementations.

Make changes in this checkout, refresh the local runtime from the same Git state,
and publish that state to GitHub. Do not edit an installed or runtime copy as an
independent source of truth.

### Installation

For a normal direct installation, clone the public repository into the Codex
standalone-skill directory:

```bash
git clone https://github.com/mlabo-org/skill-md-clarifier.git ~/.agents/skills/skill-md-clarifier
```

Restart Codex or begin a new task after installation. In this installation model,
the cloned Git checkout is also the locally active skill directory.

### Usage

```text
Use $skill-md-clarifier to audit and revise this SKILL.md and align its routing metadata.
```

The skill resolves one standalone skill or one plugin root, reads the selected
contract and adjacent metadata, applies only the requested edit or review scope,
and reports enforcement and validation boundaries.

### Repository layout

```text
.
├── SKILL.md
├── agents/
│   └── openai.yaml
├── LICENSE
└── README.md
```

### License

MIT
