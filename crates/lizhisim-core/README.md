# lizhisim-core

手牌・座席・局進行などの決定的なドメイン型と純粋遷移を提供します。
`lizhisim-rules`に依存し、検証済みルールと牌構成を直接利用します。
`TileKind`、`TileSet`、`TileSetError`を所有し、`TileSet::try_from(&ValidatedRuleSet)`で牌構成を解決します。
