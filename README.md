# skill-md-clarifier

[日本語](#日本語) | [English](#english)

## 日本語

`skill-md-clarifier` は、単体スキルおよびプラグイン内の `SKILL.md` を、
ルーティングと実行契約の両面から監査し、必要な範囲で改訂するスキルです。

### 拘束力を保ち、解釈の違いを減らす

モデルや推論深度が変わっても、同じ入力と権限から、同じ必須行動・禁止・停止判断を読み取り、
同じ要求結果と完了条件へ到達できる指示を目指します。条件や関係を指示文に持たせ、
読み手の補完に頼る箇所を減らします。

- **拘束力を弱めない。** 明確化や短文化を理由に、義務を任意にしたり、省略・例外・裁量を広げたりする変更は不合格です。
- **単語の印象で任意扱いにしない。** 「推奨」「should」「prefer」でも、ユーザーが実行を求める意味なら、その義務と認められた例外を明示します。
- **行動の流れを追えるようにする。** 誰が、どの入力を使い、どの条件で動き、何を渡し、どこで終わるかを具体化します。方法の選択が明示的に許されている場合は、その範囲を維持します。
- **不足する方針を作らない。** 正本から判断が確定する箇所は根拠を示して明確化します。続行条件などが未確定なら、監査者が条件を足して完成させず、不足する決定として返します。

たとえば「必要な情報があれば続行できる」の十分条件が定まっていない場合、
監査者が独自の必須項目や停止条件を作ることは、意味を保った明確化にはなりません。
書き換え欄が必須でも、そこに未確定の方針判断を示せます。

これは文書の作成・監査基準です。別モデルや低・高推論設定での動作一致を保証するものではなく、
モデル間の比較実測は別の証拠として扱います。

### 作成と監査の流れ

1. 作成・実質変更の前に、対象の正本・依頼・[共通基準](references/instruction-clarity.md)を作成担当へ渡します。
2. 作成担当が、既知の要件を含む完全な候補を作ります。
3. 既存の最終監査で、ネイティブ検査の結果と意味の確認を同じ候補に対して扱います。監査中に候補を編集しません。

レビュー依頼は読み取り専用で、指摘と根拠を返します。編集を依頼された場合にだけ正本を変更します。
共通基準の導入によって、別の監査工程、モデル比較、自動的な書き直しループは増えません。

検証範囲は一覧容量への影響で選びます。説明・登録などが変わらない本文修正は `--structure-only` と意味確認で完了できます。既存説明文の修正は、他の一覧条件が同じなら `--baseline-skill SOURCE=BEFORE` で変更前の正本と比べ、容量を増やしていないことを確認できます。全体容量はこの比較だけでは未測定のままです。新規追加・容量増加・一覧やモデル条件の変更、および警告の解消確認には実際の一覧全体の証拠が必要で、同条件の証拠は再利用します。日本語の呼び出し語と意味を保持し、切り詰め・欠落を全体容量の合格としません。新規作成前は `--plan-new NAME` で容量を予約できます。詳細は [Catalog Impact And Budget](SKILL.md#catalog-impact-and-budget) を参照してください。

### 監査・改訂する内容

- frontmatter のルーティング強度、文字数予算、切り詰め耐性
- 拘束力を維持した条件・分岐・受け渡し・完了条件の明確さ
- `SKILL.md`、`agents/openai.yaml`、プラグインメタデータ、ツール説明のずれ
- プラグイン内スキルの網羅性と、ソース・キャッシュ・実行環境の境界
- 実行可能な強制箇所を持たない production decision logic
- スキル固有の担当範囲、推論・ワークフロー・停止条件の漏れ
- `tool_search` の未検出、過剰発火、発火不足、スコープ漏れ
- native validator による決定的な構造・メタデータ検査

UI メタデータは簡潔に保ち、成果物の有効性に関する判断は、ソース、スキーマ、
validator、compiler、state machine、またはテストなどの実行可能な箇所に置きます。

### 共通基準と関連スキル

[指示文の解釈を揃える基準](references/instruction-clarity.md)の正本は、このリポジトリにあります。
[`agents-md-clarifier`](https://github.com/mlabo-org/agents-md-clarifier) も同じファイルを参照し、
`AGENTS.md` の権限・適用範囲・担当の監査に使います。基準の本文を両方へコピーする構成にはしていません。

`SKILL.md` の監査には本スキルを使い、`AGENTS.md` の監査には `agents-md-clarifier` を使います。
両方を使う場合は、同じ親ディレクトリへ次の名前で配置してください。

```text
<user-skill-directory>/
├── skill-md-clarifier/
│   └── references/instruction-clarity.md
└── agents-md-clarifier/
```

実行契約は [SKILL.md](SKILL.md) と、該当する分岐から参照する資料です。このREADMEは利用説明です。

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
mkdir -p "$HOME/.agents/skills"
git clone https://github.com/mlabo-org/skill-md-clarifier.git "$HOME/.agents/skills/skill-md-clarifier"
```

`AGENTS.md` も監査する場合は、同じ親ディレクトリに関連スキルを追加します。

```bash
git clone https://github.com/mlabo-org/agents-md-clarifier.git "$HOME/.agents/skills/agents-md-clarifier"
```

上記は初回インストール用です。独自のスキルディレクトリを使う場合も、両方を同じ親へ配置してください。
両方を導入済みの場合は、それぞれのcheckoutを更新し、共有ファイルを片方へ複製しないでください。

インストール後は Codex を再起動するか、新しいタスクを開始してください。この
方式では、clone した Git checkout がそのままローカルで有効なスキルディレクトリに
なります。

### 使い方

```text
$skill-md-clarifier で /path/to/my-skill/SKILL.md を読み取り専用で監査してください。
拘束力を保ったまま、条件から行動・終了まで追えるかを確認してください。
```

編集も依頼する場合:

```text
$skill-md-clarifier で /path/to/my-skill/SKILL.md を明確化してください。
実行義務と例外の範囲を維持し、正本から確定できない方針は未確定として報告してください。
```

このスキルは、1つの単体スキルまたは1つのプラグインルートを解決し、対象の実行契約と
隣接メタデータを読み、依頼された編集またはレビューの範囲だけを扱い、実装上の強制箇所と
検証境界を報告します。

### native validator

このリポジトリには macOS / Apple Silicon 向けのネイティブ検査バイナリが含まれています。
リポジトリのルートで直接呼び出します。対象には、スキルディレクトリ、`SKILL.md`、またはプラグインルートを指定できます。

```bash
./bin/darwin-arm64/skill-md-clarifier-validator /path/to/target-skill --rollout /path/to/host-rollout.jsonl --expected-skills 94 --context-window 272000
```

終了コードは、`0` が決定的検査の成功、`1` が検出事項あり、`2` が使い方・
読み取り・解析エラーです。対象解決、frontmatter、ルーティング用説明文、実行契約の宣言、
隣接メタデータなどを検査します。指示の理解やモデル間の結果一致を測定する検査ではありません。

バイナリがない場合の正本からのビルド手順は、[Native Validator Materialization](references/validator-build.md)を参照してください。
同梱バイナリの対象以外のOS・CPUで動作するという主張はしていません。

### リポジトリ構成

```text
.
├── SKILL.md
├── agents/
│   └── openai.yaml
├── local-capability-manifest.json
├── references/
│   ├── instruction-clarity.md
│   ├── description-comparison.md
│   ├── catalog-budget.md
│   └── validator-build.md
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

### Preserve binding force and reduce interpretation drift

The goal is to let different models and reasoning settings derive the same required actions,
prohibitions, stop decisions, required results, and completion conditions from the same inputs
and authority. The instructions carry the necessary conditions and relationships, reducing
what the reader must infer.

- **Never weaken binding force.** Clarification or shortening must not turn obligations into options or expand permission to skip, exceptions, or discretion. Such changes fail the audit.
- **Do not infer optionality from wording alone.** If the user intends an action to be required, wording such as “recommended,” “should,” or “prefer” must retain that obligation and its authorized exceptions.
- **Make the action flow traceable.** State who uses which inputs, when they act, what they hand over, and where they finish. Preserve the bounds of explicitly authorized choices of method.
- **Do not invent missing policy.** Clarify decisions established by authoritative inputs and cite their basis. If a condition for continuing is unresolved, report the missing decision instead of adding conditions to complete the rule.

For example, if “continue when the necessary information is available” does not establish what
is sufficient, an auditor cannot invent a required checklist or a stopping threshold and call
it a meaning-preserving rewrite. Even a required rewrite field can contain the unresolved
policy decision.

These are authoring and audit criteria. They do not guarantee identical behavior across
models or lower and higher reasoning settings. Actual model comparisons remain separate evidence.

### Authoring and audit flow

1. Before creating or materially changing a contract, give the author the authoritative inputs, request, and [shared criteria](references/instruction-clarity.md).
2. The author constructs a complete candidate incorporating the known requirements.
3. The existing final audit combines native results with semantic examination of that same candidate. It does not edit the candidate while checking it.

A review is read-only and returns findings with evidence. Source changes require an edit request.
The shared criteria do not add another audit stage, model comparison, or automatic rewrite loop.

### What it audits and revises

- frontmatter routing strength, budget, and truncation resilience
- binding-force-preserving clarity of conditions, branches, handoffs, and completion
- drift across `SKILL.md`, `agents/openai.yaml`, plugin metadata, and tool descriptions
- plugin-contained skill coverage and source/cache/runtime boundaries
- production decision logic that lacks executable enforcement
- skill-specific ownership, reasoning, workflow, and stop-condition gaps
- `tool_search` misses, over-triggering, under-triggering, and scope leakage
- deterministic structural and metadata checks through the native validator

The skill keeps UI metadata concise and leaves artifact-validity decisions in
executable source, schemas, validators, compilers, state machines, or tests.

### Shared criteria and the related skill

This repository owns the [shared instruction-clarity criteria](references/instruction-clarity.md).
[`agents-md-clarifier`](https://github.com/mlabo-org/agents-md-clarifier) reads the same file
when auditing authority, scope, and ownership in `AGENTS.md`. The criteria are maintained once,
not copied into both repositories.

Use this skill for `SKILL.md` and `agents-md-clarifier` for `AGENTS.md`. To use both,
install them under the same parent directory with these names:

```text
<user-skill-directory>/
├── skill-md-clarifier/
│   └── references/instruction-clarity.md
└── agents-md-clarifier/
```

The execution contract is [SKILL.md](SKILL.md) and the resources referenced by the applicable branch.
This README explains how to use them.

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
mkdir -p "$HOME/.agents/skills"
git clone https://github.com/mlabo-org/skill-md-clarifier.git "$HOME/.agents/skills/skill-md-clarifier"
```

To audit `AGENTS.md` as well, add the related skill under the same parent directory:

```bash
git clone https://github.com/mlabo-org/agents-md-clarifier.git "$HOME/.agents/skills/agents-md-clarifier"
```

These commands are for a first installation. If you use another skill directory, keep both
repositories under that same parent. When both are already installed, update their respective
checkouts without copying the shared file into the other repository.

Restart Codex or begin a new task after installation. In this installation model,
the cloned Git checkout is also the locally active skill directory.

### Usage

```text
Use $skill-md-clarifier to audit /path/to/my-skill/SKILL.md read-only.
Check whether its conditions lead clearly through action and completion while preserving binding force.
```

To request edits:

```text
Use $skill-md-clarifier to clarify /path/to/my-skill/SKILL.md.
Preserve required actions and the scope of exceptions. Report policy that cannot be established from authoritative inputs as unresolved.
```

The skill resolves one standalone skill or one plugin root, reads the selected
contract and adjacent metadata, applies only the requested edit or review scope,
and reports enforcement and validation boundaries.

### Native validator

The count and context in the example are illustrative: supply verified values for the target host. Whole-host admission rereads full descriptions, uses the pinned Codex renderer formula, and rejects truncation or omission. Select the scope by catalog impact: unchanged-contribution source edits use `--structure-only` plus semantic acceptance; existing descriptions under unchanged catalog conditions can use `--baseline-skill SOURCE=BEFORE` to prove nonincrease in both byte and character costs. Neither scope proves current host fit. New skills, growth, changed catalog/model conditions, and requests to clear host warnings require complete host evidence; reuse it when the relevant conditions still match. Use `--plan-new NAME` with a future absolute SKILL.md path to reserve new capacity before writing. See [Catalog Impact And Budget](SKILL.md#catalog-impact-and-budget).

This repository includes a native validator for macOS on Apple Silicon. Run it directly
from the repository root. The target may be a skill directory, a `SKILL.md`, or a plugin root:

```bash
./bin/darwin-arm64/skill-md-clarifier-validator /path/to/target-skill --rollout /path/to/host-rollout.jsonl --expected-skills 94 --context-window 272000
```

Exit code `0` means the deterministic checks pass, `1` means findings were
detected, and `2` means a usage, read, or parse failure. It checks target resolution,
frontmatter, routing descriptions, execution-contract declarations, adjacent metadata,
and related structural requirements. It does not measure instruction comprehension or
agreement between model executions.

If the binary is missing, see [Native Validator Materialization](references/validator-build.md) for the source
build procedure. The bundled binary does not establish support for other operating systems
or CPU architectures.

### Repository layout

```text
.
├── SKILL.md
├── agents/
│   └── openai.yaml
├── local-capability-manifest.json
├── references/
│   ├── instruction-clarity.md
│   ├── description-comparison.md
│   ├── catalog-budget.md
│   └── validator-build.md
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
