# rulesとcoreの依存方向

## Scope

- 要求: RULE-002、CORE-008。
- ユーザー指定に従い、rulesをcoreから独立させ、coreがrulesの検証済み型を直接利用する。
- `TileKind`と`TileSet`はcoreに残し、牌構成の解決をcore側へ移す。
- 既存の牌構成、検証エラーの情報、牌型の公開pathを維持する。

## Test list

- [x] rulesの`Qipai`設定を検証後にRoundへ渡すと、最初のツモ後も開始方式が保持される。

- [x] ユーザー設定RuleSetから検証したValidatedRuleSetをcoreで利用できる。

- [x] coreが赤牌0枚の`ValidatedRuleSet`を`TileSet::try_from`で直接利用し、`Bingpai`への赤牌追加を拒否する。
- [x] Cargoの依存グラフが`core -> rules`であり、rulesからcoreへの依存がない。
- [x] 既存の型・牌構成・遷移testとfacadeのbuildが成功する。

Current: なし（親の開始方式の設定移動まで完了）。

## Cycle log

- 2026-09-15: coreからrulesを直接利用する境界testを選択する。牌の意味論は変更しない。
- 2026-09-15: 初回redはcoreからrulesを参照できないE0433。型移動によるgreen後、ユーザー指定で牌型をcore所有に維持する方針へ変更した。同じ境界testを`TileSet::try_from(&RuleSpec)`へ変更して再度redを確認する。
- 2026-09-15: `cargo test -p lizhisim-core rules_with_zero_red_tiles_reject_red_tile_in_core`で`TryFrom<&RuleSpec>`未実装のE0277を確認した。牌型をcoreへ戻し、rulesの共有参照APIとcoreの変換実装でgreenにした。
- 2026-09-15: refactorで牌構成解決の既存5 testをcoreへ移し、赤3枚の共通fixtureを`RuleSpec`経由へ変更した。0枚・各1枚・各4枚・総数・通常牌の既存契約を維持し、coreのraw countによる除外牌testは異なる入力境界なので残した。
- 2026-09-15: `cargo metadata --no-deps --format-version 1`でrulesは`thiserror`だけに依存し、coreがrulesへ通常依存することを確認した。Cargoは循環依存も検査する。
- 2026-09-15: 標準のformat、Clippy、build、testが成功。core 103 test、rules 6 testがgreen。牌種・index・ルール値・schema・再現性の意味は変更していない。

- 2026-09-15: [ADR-0018](../adr/0018-rule-set-validation-names.md)に従い、ユーザー設定を`RuleSet`、検証済み設定を`ValidatedRuleSet`、検証エラーを`RuleSetValidationError`へ改名する項目を選択した。
- 2026-09-15: 既存の`rules_with_zero_red_tiles_reject_red_tile_in_core`を新名へ移し、`cargo test -p lizhisim-core rules_with_zero_red_tiles_reject_red_tile_in_core`で新公開型の未定義によるE0432をredとして確認した。
- 2026-09-15: 型・re-export・呼出側を改名し、`cargo test --verbose`でcore 103 test、rules 6 testとdoc-testがgreen。refactorでmoduleを`rule_set`、test名・変数も新名へ統一した。既存testの観点・assertion・エラーpayloadは維持し、重複testは追加していない。過去のcycle logとADRの旧名は当時の記録として残す。
- 2026-09-15: `cargo fmt -- --check`、`cargo clippy -- -D warnings`、`cargo build --verbose`が成功。現在の検証保証は各色の赤牌枚数0〜4であり、人数・capability・schemaの振る舞いは追加していない。

- 2026-09-15: [ADR-0019](../adr/0019-first-zimo-origin-in-rules.md)に従い、Qipai設定からRoundへの保持を選択。既存の`first_zimo_preserves_configured_origin`を設定経由へ移し、`cargo test -p lizhisim-core first_zimo_preserves_configured_origin`でrulesの型未定義E0433、field未定義E0560、constructor引数不一致E0308のredを確認した。
- 2026-09-15: rulesへ`FirstZimoOrigin::{Qipai, Bipai}`と設定field・accessorを追加し、Round生成時に検証済み設定から値を取得してgreen。enum移動・初期化の編集漏れを修正後、`cargo test --verbose`でcore 103 test、rules 6 testとdoc-testが成功した。
- 2026-09-15: refactorでは赤3枚の検証済み設定fixtureを`rules_with_origin`へ集約した。既存のBipai設定の摸切受理とQipai設定の摸切拒否が両方式の固定化を検出するため、同じ観点のtestや三角測量testを追加しない。独立した牌移動・通常巡目・局終端・エラーpayloadのtestとassertionは維持した。
- 2026-09-15: refactor後のformat、Clippy、build、全109 testとdoc-testが成功。Markdown lint・lycheeは未導入のため未実行とし、変更文書の相対リンク先を別途確認した。
