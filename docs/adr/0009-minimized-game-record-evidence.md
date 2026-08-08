# ADR-0009: 牌譜を保存せず承認済み最小fixtureへ変換する

- Status: Superseded
- Date: 2026-08-09
- Deciders: Project owner
- Amends: [ADR-0005](0005-mahjong-soul-first.md)の牌譜evidence手順
- Superseded by: [ADR-0010](0010-mahjong-soul-record-conformance.md)

## Context

雀魂のcorner caseを検証するには、実際の牌譜が有力な観測資料になる。一方、牌譜にはserviceが提供するcontent、表示名やaccountに結び付く識別子、replay locator、時刻等が含まれ得る。private storageであることだけでは複製・保存の法的根拠にならず、匿名化してもserviceの利用条件や知的財産に関する問題が自動的に解消するわけではない。

2026-08-09に確認した[雀魂利用規約](https://dmg.mahjongsoul.com/terms_of_service)には、service関連情報の権利帰属、不正な方法によるcontent取得、reverse engineering、個人を特定可能な情報の掲載等に関する条項がある。LizhiSimはこれらの条項の法的解釈を自動化せず、project ownerの確認を取得・利用のgateにする。

このADRは法的助言ではなく、repositoryで採用する保守的なdata-handling policyである。

## Decision

### Acquisition gate

- 牌譜を取得するdownloader、非公開protocol client、traffic interception、認証回避、scraperをLizhiSimに実装しない。
- serviceの公式なuser-visible share/export/replay機能、またはproject ownerが適法・規約上許容されると確認した提供経路だけを候補にする。
- 取得経路、対象範囲、利用目的をproject ownerが事前に確認し、`approved`と記録するまで牌譜を取得・処理しない。不明な場合は`blocked`とする。
- 第三者から提供された牌譜も、提供者が共有可能であると推測しない。project ownerが利用条件を確認する。
- gist等に掲載された牌譜IDは調査locatorにすぎず、取得許可を意味しない。IDやreplay URLがaccess tokenとして機能する可能性も考慮する。

### Raw data boundary

- LizhiSim projectはraw牌譜をGit、private artifact storage、fixture directory、backupへ保存しない。
- userが手元で確認するraw牌譜をrepositoryへcopyしない。project toolが原本を自動削除・移動することもしない。
- raw牌譜全体のhashを必須にしない。原本を保持しないhashだけでは後日の内容reviewを可能にせず、取得・保存の許可を代替しない。
- 変換toolを将来作る場合も、明示的に指定されたlocal inputを読み、承認対象の派生dataだけを出力するoffline toolとする。network取得機能を持たせない。

### Data minimization and anonymization

test evidenceへ必要なのは、corner caseを再現する最小の局面と期待される結果である。原則として一つの`Round`からさらに必要な直前状態とaction列へ縮小する。

派生fixtureから次を除去する。

- account ID、user ID、display name、stable pseudonym
- avatar、title、rank表示等のprofile metadata
- chat、free-form text、social情報
- replay URL、access token、secretを含み得る完全な牌譜ID
- device、network、IP、region等の利用者metadata
- 検証に不要なexact timestamp、room ID、contest ID、matchmaking ID
- 対象corner caseと無関係な別`Round`と観戦metadata

participantはfixture内だけで有効な非連結IDへ割り当て、複数fixtureをまたいで同一人物を追跡できるpseudonymを作らない。`Seat`、tile/action sequence、points、`chang`、`ben`、`lizhibang`、rule判断に必要な公開状態は、再現に必要な範囲だけ残せる。

匿名化は利用許可の代替ではない。派生fixtureもproject ownerが内容と利用条件をreviewし、commitを明示的に承認した場合だけrepositoryへ置く。

### Evidence record and hash

Gitへ置けるrecordは次とする。

```text
GameRecordEvidence
  evidence_id
  service
  acquisition_method
  acquired_at_utc
  terms_url
  terms_reviewed_at_utc
  approved_by / approved_at_utc
  source_locator_status: withheld | public_approved | unavailable
  player_identifiers_removed: true
  minimized_scope
  derived_fixture_path
  derived_fixture_sha256
  related_claim_ids
  notes
```

- 原牌譜のID、URL、hash、本文は必須recordに含めない。
- public sourceに既に掲載されたlocatorをGitへ記録する場合も、secret性、個人識別性、利用条件をproject ownerが個別に承認する。
- hashはrepositoryへcommitする承認済み派生fixtureの正確なfile bytesに対してSHA-256を計算する。raw牌譜のhashではない。
- fixtureの意味が変わる編集はhashを更新し、reviewをやり直す。formatだけの変更もfile hashが変わるためreview対象にする。

### Test usage

- testはraw牌譜parserやservice互換性ではなく、承認済み最小fixtureが表すdomain scenarioを検証する。
- 期待値は「先行実装がそうした」だけで決めず、project ownerのrule review、公式source、観測されたservice結果を区別して記録する。
- 元牌譜を保存しないため完全な再監査ができないことをevidence limitationとして明示する。
- 承認済みfixtureを作れないcorner caseはtest listに`blocked`で残し、一般的な推測値で置き換えない。

## Consequences

### Positive

- raw牌譜とplayer識別情報をrepositoryやproject storageへ蓄積しない。
- private storageを保存許可の根拠にする誤りを避けられる。
- test corpusをdomain上必要な最小scenarioへ限定できる。
- service取得処理をsimulatorから分離し、reverse engineeringへの依存を避けられる。

### Negative

- 元牌譜全体を後から再生・再監査できない。
- project ownerによる取得経路と派生fixtureの個別reviewが必要になる。
- publicな牌譜IDがあっても、許可確認まで自動収集できない。
- 最小化の過程でcorner caseに必要なcontextを落とす可能性があり、review負担が増える。

## Alternatives considered

### Raw牌譜をprivate storageへ保存する

Rejected as default. 非公開であることだけでは取得・複製・保存の許可を保証しない。

### Raw牌譜をrepositoryへcommitする

Rejected. player情報、service content、access locator、repository sizeの問題がある。

### 自動取得してから匿名化する

Rejected. 匿名化は取得方法の許可を代替せず、自動取得自体がservice利用条件に抵触し得る。

### 牌譜IDとraw hashだけを保存する

Rejected as required evidence. locatorがsecretまたは識別子になり得て、rawを保持しないhashは後日の内容reviewを可能にしない。

## Follow-up / verification

- 最初の牌譜候補を処理する前に、project ownerが取得経路と利用条件を個別にreviewする。
- 最初の派生fixture案はcommit前に匿名化checklistとdiffをproject ownerへ提示する。
- fixture schemaはinitial observation/action/event schemaと整合させるが、raw service schemaを公開schemaにしない。
- 利用規約の改定日とreview日時を記録し、取得作業の前に再確認する。
