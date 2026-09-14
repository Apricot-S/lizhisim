# ADR-0017: coreが独立したrulesを直接利用する

- Status: Accepted
- Date: 2026-09-15
- Deciders: Project owner
- Supersedes: [ADR-0015](0015-rule-and-domain-tile-ownership.md)
- Relates to: [ADR-0014](0014-facade-and-core-crates.md)

## Context

ADR-0015ではrulesがcore所有の`TileKind`と`TileSet`へ依存していた。
Project ownerはrulesをcoreから独立させ、coreがrulesへ依存して直接利用する構成を指定した。
`TileKind`と`TileSet`はcoreに残すことも指定された。rulesがこの2型を返すAPIは維持できない。

## Decision

- 依存方向を`lizhisim-core -> lizhisim-rules`とし、rulesからcoreへの依存を禁止する。
- coreは`TileKind`、`TileSet`、`TileSetError`の実装と局所unit testを所有する。
  `tile.rs`と`tile_set.rs`は独立moduleを維持する。
- raw設定の検証はrulesに残し、`RuleSpec::hong_baopai`が検証済み設定への共有参照を返す。
- `RuleSpec::resolve_tile_set`をcoreの`TileSet::try_from(&RuleSpec)`へ移し、
  coreが設定を直接参照して実行時牌構成を生成する。変換失敗は`TileSetError`を返す。
- rulesの検証エラーは`TileKind`の代わりに、設定fieldを識別する閉じたenum
  `HongBaopaiConfigField`を持つ。対象field、実際の値、上限を保ち、coreのエラーは含めない。
- `Bingpai`、`Bipai`、`Round`等の状態と遷移はcoreが所有する。
  局内状態に対するruleの適用はcoreの責務とし、rulesやorchestrationへ遷移を移さない。
- 牌型の既存の`lizhisim_core`とfacadeの公開pathを維持する。
- raw入力を遷移へ直接渡さず、検証済み型だけを使う境界を維持する。
  出典管理やserializationの副作用をcoreへ持ち込まない。
- 未実装のruleやpolicyは今回追加せず、選択したtest list項目で順に実装する。

```text
lizhisim -> lizhisim-core -> lizhisim-rules
lizhisim -> lizhisim-rules
lizhisim-rules -X-> lizhisim-core
```

## Consequences

### Positive

- rulesをcoreなしでbuild・検証できる。
- coreはrulesの検証済み設定を直接利用し、rulesにcoreの値型を持ち込まない。
- `TileKind`のindex、牌数検証、公開pathを維持できる。

### Negative

- rulesの公開型の変更がcoreの再コンパイルへ波及する。
- 牌構成の解決APIとrulesの検証エラーpayloadの呼出側に移行が必要になる。

## Alternatives considered

- 牌型をrulesへ移す案は、coreに残すというユーザー指定に従い採用しない。
- rules側へ牌型を複製する案は、変換と対応表の重複が生じるため採用しない。
- 共通値型crateの追加は、現在の境界変更に必要でないため採用しない。

## Follow-up / verification

- [依存方向test list](../test-lists/rules-core-dependency.md)で直接利用と依存グラフを検証する。
- 既存の牌種・牌構成・遷移testを維持し、workspace標準検証を実行する。
