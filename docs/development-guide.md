# 開発手順書

## 1. 現在の作業範囲

現在はPhase 1であり、決定的な一局walking skeletonをtest listから一項目ずつ実装する。選択中でない振る舞いを先回りして実装せず、crateやdependencyの追加は選択項目に必要な場合だけ行う。

文書だけを変更する場合も、変更理由、要求 ID、文書間リンク、未決事項を確認する。設計が変わる場合は ADR を追加または supersede する。

AIへの依頼は[タスクプロンプト](task-prompts.md)を参考に、今回の成果物と完了範囲を指定する。通常の調査・編集・検証は依頼範囲内で続行し、TDDの各サイクルを承認待ちの区切りにはしない。

workspace のcrateは `{root}/crates/` 配下に置く。通常のcrateは独立した `README.md` を持ち、責務、依存方向、公開範囲を簡潔に記載する。プロジェクトのfacade crateだけは `readme.workspace = true` でworkspace rootのREADMEを使ってよい。workspace rootのREADMEはプロジェクト全体の説明に限定する。workspaceと全crateの初期versionは `0.0.1` とし、crate固有のversion変更が必要になるまでworkspace versionを基準にする。

識別子を追加する前に [用語集](glossary.md) を確認する。対応する行がなければ「ユーザー決定待ち」表へ空欄で追加し、ユーザーがピンインまたは英語識別子を決めるまで production 名を作らない。日本語ローマ字やその場限りの英訳で仮置きしない。

Rust の `use` 宣言は、rustfmt nightly の `group_imports = "StdExternalCrate"` に倣って並べる。標準ライブラリ（`std`、`core`、`alloc`）、外部crate、crate内（`self`、`super`、`crate`）の順に3グループへ分け、グループ間には1行の空行を置く。該当しないグループは省略する。

## 作業別の手順

| 作業 | 必須の参照先 |
|---|---|
| 実装・バグ修正 | [TDDとバグ修正](development/tdd.md)、対象の[test list](test-lists/README.md) |
| テストの種類・test doubleを選ぶ | [テスト戦略](development/testing.md) |
| `xiangting`・`hule`導入、プリセット追加・改定 | [外部crateとプリセット](development/adapters-and-presets.md) |
| 差分レビュー・commit作成 | [レビューと変更単位](development/review.md) |
| 検証・CI・完了判定 | [検証コマンドと完了条件](development/verification.md) |

対象の手順を必要なときに読む。設計と要求の入口は[文書案内](README.md)にある。

## 初回実装開始の記録

Phase 0からの開始判断は[Phase 0 review](phase-0-review.md)に記録している。現在はPhase 1であり、この初回gateを通常の作業再開時に再承認する必要はない。選択項目に必要な未決事項・出典・依存crateの条件は、その項目を実装する前に解決する。
