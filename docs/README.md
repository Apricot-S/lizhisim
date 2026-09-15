# 文書案内

作業に必要な文書と節を選ぶための入口。現在は **Phase 1: 決定的な一局walking skeleton**。設計上の将来像と、実装済みの振る舞いを区別する。

## 作業から探す

| 知りたいこと・作業 | 最初に読む文書 | 必要に応じて読む文書 |
|---|---|---|
| 目的と対象範囲 | [ビジョン](vision.md) | [要求仕様](requirements.md) |
| 実装する項目・作業再開 | [test list一覧](test-lists/README.md)の対象listのCurrent | [TDD](development/tdd.md)、対象の設計 |
| 依存方向・責務境界 | [アーキテクチャ](design/architecture.md) | [ADR一覧](adr/README.md) |
| 牌・Player・Round・対局精算 | [ドメインモデル](design/domain-model.md) | [ルール設計](design/rules-and-presets.md) |
| 要求・応答・キュー | [推論プロトコル](design/inference-protocol.md) | [テスト戦略](development/testing.md) |
| 大会・段位・リーグ | [大会設計](design/competitions.md) | [要求仕様](requirements.md) |
| プリセット・公式情報の確認 | [ルール設計](design/rules-and-presets.md) | [出典台帳](references/rule-sources.md)、[開発手順](development/adapters-and-presets.md) |
| 雀魂walking skeletonの根拠 | [RuleClaim mapping](references/mahjong-soul-walking-skeleton-rule-claims.md) | [1パスconformance ADR](adr/0011-one-pass-majsoul-conformance.md) |
| 識別子を追加する | [用語集](glossary.md) | [用語のADR](adr/0004-pinyin-terminology-and-round.md) |
| 検証・レビュー・完了判定 | [検証と完了条件](development/verification.md) | [レビュー観点](development/review.md) |
| 今後の段階と未決事項 | [ロードマップ](roadmap.md) | 対象の要求・設計・test list |
| AIへ作業を依頼する | [タスクプロンプト](task-prompts.md) | [開発手順の入口](development-guide.md) |

## 情報の置き場所

| 情報 | 更新先 |
|---|---|
| リポジトリ全体の作業規約 | [AGENTS.md](../AGENTS.md) |
| 製品の目的・非目標 | [ビジョン](vision.md) |
| 実現すべき契約と要求ID | [要求仕様](requirements.md) |
| 現在採用している責務・状態・schemaの設計 | `design/`の対象文書。概念例は実装済みAPIを意味しない |
| 設計判断の理由と置換関係 | [ADR一覧](adr/README.md)。Accepted本文の意味を変えるときは後継ADR |
| 識別子の確定・未決 | [用語集](glossary.md) |
| 作業手順・検証方法 | [開発手順の入口](development-guide.md)から用途別文書 |
| 実装する項目、red/greenの証拠、移送先 | 対象test listのCurrent・項目・Cycle log |
| 公式資料の確認記録と条項の解釈 | 出典台帳とRuleClaim mapping。原資料の保存方針は[ADR-0008](adr/0008-source-review-without-copying.md) |
| 段階ごとの到達目標と未決事項 | [ロードマップ](roadmap.md) |

同じ規範の全文を別文書へ複製せず、更新先へリンクする。要求ID、条項ID、既存のtest list記録は追跡に使うため維持する。

## 履歴とテンプレート

- [Phase 0 review](phase-0-review.md)は初回実装開始の判断記録。現在の作業再開に新たな承認を要求するものではない。
- Superseded ADRは経緯を調べるときに読む。現在の規範はADR一覧から後継を確認する。
- 完了したtest listのCycle logは検証履歴。再開時はまずCurrentと未完了項目を確認し、判断の根拠が必要な場合に該当cycleへ進む。
- 新規test listには[テンプレート](templates/test-list.md)を使う。
- 文書構成の変更理由は[再構成の記録](maintenance/documentation-restructure.md)にある。
