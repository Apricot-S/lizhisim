# 公式ルール出典台帳

## 1. 目的

プリセットの値を確定するための一次資料と、まだ足りない証跡を管理する。これはルール値そのものの一覧ではない。公式資料は改定され得るため、実装時には URL だけでなく取得 snapshot、版、対象期間、内容 hash を preset metadata に固定する。

- 初回調査日: **2026-08-08 (Asia/Tokyo)**
- 最終更新日: **2026-08-09 (Asia/Tokyo)**

## 2. 証跡 grade

| Grade | 意味 |
|---|---|
| A | 公式かつ版・日付が明確な規則本文。snapshot/hash と条項 mapping 後に verified の根拠にできる。 |
| B | 公式の規則本文だが版または effective period が明確でない。取得 snapshot と公式改定確認が必要。 |
| C | 公式の概要・guide・入口だけで完全な設定値がない。game 内 help 等の追加一次証跡が必要。 |
| Blocked | 公式一次資料へ到達できない、または相互矛盾を解消できない。値を確定しない。 |

Grade は資料品質であり、preset の実装・検証状態ではない。Grade A でも条項 mapping と golden test が終わるまで preset は `verified` ではない。

## 3. オンライン段位戦

| 対象 | 公式一次資料 | Grade | 確認できた範囲 | 次の監査 |
|---|---|---:|---|---|
| 雀魂 四人/三人段位戦 | [雀魂 段位戦詳細ルール](https://mahjongsoul.com/news/46) | B | 公式の詳細ルール本文。明記されない corner case がある | 下記の Kanachan 検証記録から牌譜 ID を抽出し、元牌譜を再取得して回帰 test にする |
| 天鳳 四人/三人段位戦 | [オンライン対戦麻雀 天鳳 / マニュアル](https://tenhou.net/man/index.html) | B | 段位/Rate、卓、対局・大会機能の公式 manual | 段位戦 4/3 人の mode code、卓内ルール、pt 表を個別条項へ mapping |
| 麻雀一番街 四人/三人段位戦 | [麻雀一番街 公式サイト](https://www.mahjong-jp.com/) | C | 公式 service と game 内規約への入口 | game 内 help の卓内ルール、段位 point、room eligibility を app version/region 付きで取得 |
| 龍龍 四人/三人段位戦 | [龍龍 ルール](https://ron2.jp/rule/) | B | 四人東風/東南、三麻差分、赤、飛び、アガリ止め、点数等の公開本文 | 段位制度、room/mode 別 point、改定履歴を追加し、WRC 差分の対象版を照合 |

### オンライン game 内証跡

game 内表示しかない場合、最低限次を記録する。

- service、platform、app build/version、server/region、language
- 取得日時と navigation path
- 画面の識別名と複数 page の順序
- screenshot/file hash
- account 固有情報を除去したこと
- 表の全行が scroll/pagination を含め取得できたこと
- 同じ内容を別 reviewer が確認した記録

非公式 Wiki、攻略 site、SNS 投稿は一次資料の欠落発見に使えるが、値確定の唯一の根拠にしない。

### 雀魂の補助検証資料

| 資料 | 位置付け | 使用方法 |
|---|---|---|
| [Cryolite/kanachan `src/simulation`](https://github.com/Cryolite/kanachan/tree/main/src/simulation) | 雀魂牌譜との照合実績がある先行実装 | corner case の候補、状態分割、必要な test 観点を発見する。コードを一次資料扱いせず、license を確認せずコピーしない |
| [Cryolite の雀魂シミュレーション検証記録](https://gist.github.com/Cryolite/a026f41713f6a7ca88713737f5c2cfb6) | 牌譜 ID 付きの検証記録 | 記載された牌譜 ID から元牌譜を改めて取得し、該当 `Round` を最小 regression/golden test にする |

推奨する evidence chain:

```text
雀魂公式詳細ルール
  -> 記載外の corner case を Kanachan/検証記録から発見
  -> 記載された牌譜 ID の元牌譜を取得
  -> raw 牌譜を保存・hash 化
  -> 最小 test list を作成
  -> red -> green -> refactor
  -> LizhiSim の event と元牌譜を照合
```

Kanachan の挙動だけを期待値にしない。最終的な根拠を取得済み牌譜と公式ルールに戻し、取得不能な牌譜に依存する項目は `verified` にしない。

## 4. 競技団体・リーグ

| 対象 | 公式一次資料 | Grade | 公開版・更新情報 | 次の監査 |
|---|---|---:|---|---|
| M リーグ | [Mリーグとは — 公式戦ルール](https://m-league.jp/about/) | B | 規則本文は公開、page 上で明示版を確認できない | 取得 snapshot、対象 season、season competition 規定を分離して mapping |
| WRC | [WRC Rules](https://www.worldriichi.org/wrc-rules) | A | WRC Rules 2025、Penalties、Clarifications 2025、Optional Rules への公式導線 | rules/clarifications/optional の適用優先順位を mapping |
| 最高位戦日本プロ麻雀協会 | [ルール](https://saikouisen.com/about/rules/) | A/B | 最高位戦競技規定 PDF と対局種別特例への公式導線、更新日の表示あり | 取得時点の PDF 版を hash 化し、本戦/Classic/特例を別 family に分ける |
| 日本プロ麻雀連盟 | [競技ルール](https://www.ma-jan.or.jp/activity/game_rule.html) | B | 公式 page、本文は埋込み資料を含む | 公式/A、WRC、WRC-R と対象大会を分離し、最新本文を snapshot |
| 日本プロ麻雀協会 | [日本プロ麻雀協会 競技規定](https://npm2001.com/about/regulations/) | B | 公式規定本文と PDF への導線 | 対象期、title 戦特例、改定履歴を確認 |
| 麻将連合 (μ) | [競技規定](https://mu-mahjong.jp/tournament/%E7%AB%B6%E6%8A%80%E8%A6%8F%E5%AE%9A/)、[麻将連合公式ルール PDF (2025-01 path)](https://mu-mahjong.jp/wp-content/uploads/2025/01/murule_202501.pdf) | A/B | 公式本文、μカップ/将王・league 等の聴牌料・罰則差を公開 | PDF 内版を確認し、卓内意味論と実卓罰則を分離 |
| RMU | [RMU ルール](https://rmu.jp/rule) | B | A/B/M の差分と基本規則を公開 | page snapshot と改定履歴、各 title が採用する rule version を確認 |

## 5. 資料ごとの注意

### 5.1 M リーグ

公式 page には卓内規則と審判・所作の規則が同居する。LizhiSim は牌、行為、和了、点数、対局終了を `TableRules/MatchRules` へ mapping し、発声品質、見せ牌、審判裁定などを `physical/administrative clause` として別追跡する。season の team 数、stage、進出、持越しは `CompetitionPolicy` であり卓内 preset に含めない。

### 5.2 WRC と JPML

WRC 公式 2025 文書、JPML が大会で用いる WRC 系、龍龍の「WRCルール」表記を同一内容と仮定しない。それぞれの一次資料と適用日を比較し、content hash が一致しない限り別版とする。

### 5.3 最高位戦・麻将連合・RMU

一団体に複数 variation がある。団体名だけを preset ID にせず、main/Classic、μカップ/将王等、A/B/M を family または別版として明示する。

### 5.4 オンライン段位戦

卓内ルール、対局 mode、room eligibility、段位 point は別資料・別更新周期の場合がある。一つの画面だけで「段位戦 preset」を verified にしない。

## 6. Source review record

公式資料の記録は[ADR-0008](../adr/0008-source-review-without-copying.md)に従う。原資料をprivate storageを含むproject管理下へ原則複製せず、GitにはURL、取得日時、確認者、locator、必要最小限のclaimだけを保存する。

```text
SourceReview
  source_id
  organization_or_service
  title
  canonical_url
  final_url
  document_version
  effective_from / effective_to
  retrieved_at_utc
  reviewed_at_utc / reviewed_by
  locale / region / app_version
  source_locator
  evidence_grade
  availability
  supersedes
  notes
```

最低限、URLと取得日時があればrecordを作成できる。不明な版、対象期間、locatorは推測せずnotesへ記録する。project ownerの確認記録をrule value reviewの承認とし、本文、PDF、DOM、screenshotの保存やcontent hashをverifiedの必須条件にしない。

## 7. Clause mapping record

```text
RuleClaim
  claim_id
  preset_family
  config_path
  normalized_value
  source_id
  source_retrieved_at_utc
  source_locator (section/article/page/screen)
  reviewed_by / reviewed_at_utc
  applicability
  simulator_scope: computational | administrative | physical | unsupported
  reviewer
  status
```

一つの条項が複数設定へ影響する場合は claim を分ける。一つの設定値が複数資料に依存する場合はすべての source を持つ。曖昧な文章を boolean 一つに落とす前に、解釈 note と scenario を用意する。

## 8. 更新監視

release 前と preset alias 更新時に公式 source を再確認する。自動監視を将来導入しても、差分を自動的に verified 設定へ反映しない。変更検知 -> draft version -> human review -> TDD -> verified の順を守る。

## 9. 著作権と商標

- 公式文書の本文や画像を無断で大量転載しない。
- 設定値と必要最小限の要約、条項 locator、URL を保存する。
- service/団体名は互換性・出典表示のために使い、提携・公認を示唆しない。
- screenshot evidence の保存場所とアクセス範囲は別途決める。
- 旧実装の third-party notice は依存撤去とともに削除した。将来 dependency を追加したら license/notice をその時点で再生成・review する。
