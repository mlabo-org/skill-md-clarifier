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
- スキル固有の担当範囲、推論・ワークフロー・停止条件の漏れ
- `tool_search` の未検出、過剰発火、発火不足、スコープ漏れ
- native validator による決定的な構造・メタデータ検査

UI メタデータは簡潔に保ち、成果物の有効性に関する判断は、ソース、スキーマ、
validator、compiler、state machine、またはテストなどの実行可能な箇所に置きます。

### 正本と discovery entry

この Git checkout が、メンテナ開発の物理正本であり、ローカルで有効な Codex
skill の discovery entry です。別の standalone runtime copy や同期用の正本は
作りません。GitHub の `origin` は、この checkout から公開する別ライフサイクルの
統合先です。

変更はこの checkout に適用します。Codex が source directory を直接 discovery
するため、インストール済みコピーや実行コピーを独立した正本として編集したり、
別の runtime へ反映したりしません。

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

### native validator

決定的な検査は materialized binary を直接呼び出します。

```bash
./bin/darwin-arm64/skill-md-clarifier-validator <skill-root-or-SKILL.md>
```

終了コードは、`0` が決定的検査の成功、`1` が検出事項あり、`2` が使い方・
読み取り・解析エラーです。validator が未 materialize の場合は、checkout の
ルートで次を実行して生成します。

```bash
cargo build --locked --release
mkdir -p bin/darwin-arm64
install -m 755 target/release/skill-md-clarifier-validator bin/darwin-arm64/skill-md-clarifier-validator
```

### リポジトリ構成

```text
.
├── SKILL.md
├── agents/
│   └── openai.yaml
├── local-capability-manifest.json
├── schemas/
│   └── local-capability/
│       ├── audit-edit-skill-md-input.schema.json
│       └── audit-edit-skill-md-output.schema.json
├── Cargo.toml
├── Cargo.lock
├── src/
├── tests/
├── bin/
│   └── darwin-arm64/
│       └── skill-md-clarifier-validator
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
- skill-specific ownership, reasoning, workflow, and stop-condition gaps
- `tool_search` misses, over-triggering, under-triggering, and scope leakage
- deterministic structural and metadata checks through the native validator

The skill keeps UI metadata concise and leaves artifact-validity decisions in
executable source, schemas, validators, compilers, state machines, or tests.

### Source of truth and discovery entry

This Git checkout is both the physical source of truth for maintainer development
and the discovery entry for the locally active Codex skill. No separate standalone
runtime copy or synchronization source is created. GitHub `origin` is a separate
publication and integration lifecycle for this checkout.

Make changes in this checkout. Codex discovers the source directory directly, so do
not edit an installed or runtime copy as an independent source of truth or propagate
the result to another runtime.

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

### Native validator

Run the materialized binary directly for deterministic checks:

```bash
./bin/darwin-arm64/skill-md-clarifier-validator <skill-root-or-SKILL.md>
```

Exit code `0` means the deterministic checks pass, `1` means findings were
detected, and `2` means a usage, read, or parse failure. If the validator has not
been materialized, build it from the checkout root:

```bash
cargo build --locked --release
mkdir -p bin/darwin-arm64
install -m 755 target/release/skill-md-clarifier-validator bin/darwin-arm64/skill-md-clarifier-validator
```

### Repository layout

```text
.
├── SKILL.md
├── agents/
│   └── openai.yaml
├── local-capability-manifest.json
├── schemas/
│   └── local-capability/
│       ├── audit-edit-skill-md-input.schema.json
│       └── audit-edit-skill-md-output.schema.json
├── Cargo.toml
├── Cargo.lock
├── src/
├── tests/
├── bin/
│   └── darwin-arm64/
│       └── skill-md-clarifier-validator
├── LICENSE
└── README.md
```

### License

MIT
