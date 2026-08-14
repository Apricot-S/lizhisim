# Test lists

振る舞い単位のtest listをこのdirectoryに置く。Phase 0では実装対象を選択せず、planned listの`Selected`は`None`に保つ。ユーザーが実装開始を明示した後、一項目だけをactiveにする。

## Planned

- [雀魂段位戦・四人 walking skeleton](mahjong-soul-ranked-four-player.md) — 最初の基準presetに向けたactive list。
- [TileSetと牌構成rule](tile-set-and-rule-tile-config.md) — core実行値、rules解決、`Bingpai`/`Bipai`連携。
- [王牌・嶺上ツモ・宝牌表示](wangpai-replacement-draw-and-baopai.md) — 王牌index、嶺上ツモ権限、表裏ドラ表示、rule依存の開槓順序。
- [最小Player aggregateとRound初期遷移](round-player-initial-state.md) — 配牌結果のseat別所有と、局開始から最初のツモ後までのtypestate。
- [`Round`の最初の`Dapai`](round-first-dapai.md) — 打牌元、`Zimo` origin、第一巡状態、`Bingpai`・`He`の原子的更新。
- [`Round`の反応なし遷移](round-no-reaction-transition.md) — 打牌後に反応なしで解決し、次actorのツモ前typestateへ進む最小遷移。
- [`Round`の最小荒牌平局](round-exhaustive-draw.md) — 通常ツモ牌枯渇を、流局精算前の局終端へ変換する最小遷移。
- [`TableMatch`の局精算境界](table-match-round-settlement.md) — `RoundEnded`を局外ledgerへ適用し、次局または対局終端を選ぶ責務。

新規作成時は [template](../templates/test-list.md) を複製し、[開発手順書](../development-guide.md) の `test list -> one -> red -> green -> refactor` に従う。

完了した test list は削除せず、要求・ADR・公式ルール版への追跡記録として残す。後継 topic へ移した項目にはリンクと理由を記載する。
