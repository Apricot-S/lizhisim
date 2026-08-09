# Test lists

振る舞い単位のtest listをこのdirectoryに置く。Phase 0では実装対象を選択せず、planned listの`Selected`は`None`に保つ。ユーザーが実装開始を明示した後、一項目だけをactiveにする。

## Planned

- [雀魂段位戦・四人 walking skeleton](mahjong-soul-ranked-four-player.md) — 最初の基準presetに向けたactive list。
- [TileSetと牌構成rule](tile-set-and-rule-tile-config.md) — core実行値、rules解決、`Bingpai`/`Bipai`連携。

新規作成時は [template](../templates/test-list.md) を複製し、[開発手順書](../development-guide.md) の `test list -> one -> red -> green -> refactor` に従う。

完了した test list は削除せず、要求・ADR・公式ルール版への追跡記録として残す。後継 topic へ移した項目にはリンクと理由を記載する。
