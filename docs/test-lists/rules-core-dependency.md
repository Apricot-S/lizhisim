# rulesとcoreの依存方向

## Scope

- 要求: RULE-002、CORE-008。
- ユーザー指定に従い、rulesをcoreから独立させ、coreがrulesの検証済み型を直接利用する。
- `TileKind`と`TileSet`はcoreに残し、牌構成の解決をcore側へ移す。
- 既存の牌構成、検証エラーの情報、牌型の公開pathを維持する。

## Test list

- [x] ユーザー設定RuleSetから検証したValidatedRuleSetをcoreで利用できる。

- [x] coreが赤牌0枚の`ValidatedRuleSet`を`TileSet::try_from`で直接利用し、`Bingpai`への赤牌追加を拒否する。
- [x] Cargoの依存グラフが`core -> rules`であり、rulesからcoreへの依存がない。
- [x] 既存の型・牌構成・遷移testとfacadeのbuildが成功する。

Current: なし（依存変更と名称変更の対象項目は完了）。

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
