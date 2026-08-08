# 雀魂四人 walking skeleton RuleClaim mapping

## 1. 目的と範囲

雀魂段位戦・四人を基準にする最初の walking skeleton が、どの公式ルール値へ依存するかを最小限に固定する。これは完全な雀魂presetではなく、最初のtest listのsource dependencyを明示するためのmappingである。

公式資料に書かれていない値は推測しない。`Bipai`の並び、初期actor、request IDなど、testが決定的であるために固定する値はfixture metadataであり、雀魂の`RuleClaim`ではない。

## 2. SourceReview

| Field | Value |
|---|---|
| `source_id` | `mahjongsoul-ranked-rules` |
| organization / service | 雀魂 |
| title | 段位戦ルール説明 |
| canonical / final URL | <https://mahjongsoul.com/news/46> |
| document version / effective period | 記載なし |
| retrieved at | `2026-08-08T19:27:19Z` |
| locale / region | 日本語 / 日本向けWeb site |
| source locator | `四人麻雀`以下の`基本ルール`。modeの適用範囲は冒頭注記 |
| evidence grade | B |
| availability | 公開Web page。本文のproject内複製は行わない |
| reviewed by / at | project owner / `2026-08-09` |
| notes | 取得時点でpage上に版、発効日、更新日を確認できない |

追加のgame挙動は、project ownerが`2026-08-09`に確認した記録を`mahjongsoul-game-behavior-2026-08-09`として参照する。これはWeb本文にない赤牌の種類別内訳と親の初期牌数を補うreview evidenceであり、原牌譜やscreenshotをprojectへ保存したことを意味しない。

## 3. 最小RuleClaim mapping

`config_path`はschema確定前の概念pathであり、実装識別子を確定するものではない。`review`は公式本文との対応を記録済みだがproject ownerの確認前、`reviewed`は値をproject ownerが確認済みだが対応testは未実装であることを表す。個別claimの`reviewed`はpreset全体の`verified`を意味しない。

| Claim ID | `config_path` | Normalized value | Source / locator | Walking skeletonでの用途 | Status |
|---|---|---|---|---|---|
| `MS4-WS-001` | `table_rules.player_set` | `four_player` | `mahjongsoul-ranked-rules` / `四人麻雀` | `Seat`の個数と範囲を決める | `reviewed` |
| `MS4-WS-002` | `match_rules.initial.starting_points_per_seat` | `25000` | `mahjongsoul-ranked-rules` / `四人麻雀 > 基本ルール > 配原` | 初期状態と観測に点数を含める場合の値を決める | `review` |
| `MS4-WS-003` | `table_rules.tiles.red_tile_copy_count.total` | `3` | `mahjongsoul-ranked-rules` / `四人麻雀 > 基本ルール > 赤ドラ` | 雀魂基準の全`Bipai`でtile conservationを検査する | `reviewed` |
| `MS4-WS-004` | `table_rules.tiles.red_count_by_kind` | `5m = 1, 5p = 1, 5s = 1` | `mahjongsoul-game-behavior-2026-08-09` / project owner review | 赤牌を含む完全な`TileKind` multisetを作る | `reviewed` |
| `MS4-WS-005` | `table_rules.dealing.dealer_initial_tile_count` | `14` | `mahjongsoul-game-behavior-2026-08-09` / project owner review | 雀魂の開始状態を正規化する | `reviewed` |
| `MS4-WS-006` | `table_rules.tiles.count_by_kind` | 通常の5は各3枚、赤の5は各1枚、その他は各4枚 | `mahjongsoul-game-behavior-2026-08-09` / project owner review | 37種類の完全なmultisetとtile conservationを検査する | `reviewed` |

全claimの`preset_family`は`mahjongsoul.ranked.four_player`、`applicability`は段位戦・四人、`simulator_scope`は`computational`とする。公式Web pageに基づくclaimの`source_id`は`mahjongsoul-ranked-rules`、`source_retrieved_at_utc`は`2026-08-08T19:27:19Z`とする。project ownerによるgame挙動確認を根拠にするclaimは、review記録と確認日をsource locatorとして保持する。

赤牌を通常牌と同じものとして数えた34種類は各4枚である。37種類の`TileKind`へ解決すると、通常の`5m`、`5p`、`5s`が各3枚、対応する赤牌が各1枚、その他が各4枚となる。同じ`TileKind`の複数枚を個別には識別しない。

## 4. RuleClaimではないwalking skeleton条件

次はsourceから導くルール値ではなく、最小の決定的scenarioを作るための条件である。

| 条件 | 管理場所 | 理由 |
|---|---|---|
| `Bipai`の具体的な順序 | fixture | replay可能な入力を固定するため |
| 最初の`Zimo`と`Dapai`対象 | fixtureから導出 | 固定`Bipai`とseat状態の結果であるため |
| 選択する合法`Dapai` | test case | continuation再開を一度だけ検査するため |
| request / table識別子 | fixture | queue境界によらないevent比較のため |
| event配送時刻とbatch境界 | test parameter | ルールではなく実行環境の非決定性を検査するため |

純粋なengine構造を先に試すfixtureは、`preset_family`を名乗らず`simulator_fixture`と明記する。未確定の雀魂値を便宜的なdefaultで補って、雀魂presetのfixtureとして扱ってはならない。

## 5. Test listとの対応

| Test list item | 必要なclaim | 選択条件 |
|---|---|---|
| 四人用の全`Seat`を構築でき、範囲外seatを構築できない | `MS4-WS-001` | project owner review後に選択可能 |
| 赤牌を区別した37種類の`TileKind`を構築できる | なし | domain型のtestとして選択可能。牌の個別identityは作らない |
| 雀魂四人基準fixtureの牌構成を検証済み値へ変換できる | `MS4-WS-001`, `MS4-WS-003`, `MS4-WS-004`, `MS4-WS-006` | 対応claimのreview後 |
| 固定`Bipai`から最初のrequest、応答、次の状態を得る | `MS4-WS-005`と使用するfixture | [ADR-0012](../adr/0012-normalize-dealer-first-draw.md)の正規化を適用する |

残りの公式条項は、和了、`fulu`、`huangpai_pingju`、連荘、終了などを扱う後続test listで必要になった時点で追加する。walking skeletonで未使用の値を先回りしてmappingしない。
