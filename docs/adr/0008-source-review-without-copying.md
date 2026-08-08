# ADR-0008: 公式資料を複製せずreview記録で追跡する

- Status: Accepted
- Date: 2026-08-09
- Deciders: Project owner
- Supersedes: [ADR-0007](0007-content-addressed-source-evidence.md)

## Context

ADR-0007は、公式Webページ等のraw bytesをGit外のprivate artifact storageへ保存する方針を採用した。しかし非公開storageであることはaccess controlを意味するだけで、第三者資料を複製・保存する法的根拠にはならない。保存先の公開範囲だけを理由に保存可能と判断してはならない。

LizhiSimではproject ownerがルール本文を確認し、設定値とsourceの対応をreviewする。公式資料そのものを再配布・保管することより、どのURLをいつ誰が確認し、どの箇所をどう解釈したかを追跡することを優先する。

このADRは公式Webページ、PDF、game内help等のrule sourceを対象とする。牌譜の取得、保存、匿名化は別途判断する。

## Decision

### 原資料を保存しない

- 公式Webページ、PDF、rendered DOM、screenshot等の原資料は、Git、private artifact storage、local evidence cacheのいずれにも原則保存しない。
- 「privateだから保存可能」と判断しない。保存が必要な個別資料は、権利・利用条件と目的をproject ownerが別途確認し、明示的に許可した場合だけ例外とする。
- source content hashは必須にしない。raw bytesを保持しないhashだけでは内容を復元・reviewできず、保存の代替証跡にはならない。
- 通常のbrowser cache、OS cache、通信基盤の一時dataは本projectのevidence storageとして管理・依存しない。

### SourceReview record

Gitには原資料ではなく、次のreview recordを保存する。

```text
SourceReview
  source_id
  organization_or_service
  title
  canonical_url
  final_url
  retrieved_at_utc
  reviewed_at_utc
  reviewed_by
  document_version
  effective_from / effective_to
  locale / region / app_version
  source_locator
  evidence_grade
  availability: available | unavailable | access_restricted
  supersedes
  notes
```

- 最低限必要なのはsource ID、URL、title、取得日時、確認者、確認日時、対象範囲である。
- source locatorはsection、見出し、article、page、screen path等、再確認に使える位置情報とする。
- secret、cookie、account ID、tokenは記録しない。
- rule本文は転載せず、設定値、必要最小限のparaphrase、解釈noteだけを記録する。短い引用が必要な場合もproject ownerのreview対象とする。

### RuleClaim and verification

- `RuleClaim`は`source_id`、取得日時、source locator、確認者へ結び付ける。
- project ownerによる確認記録を、LizhiSimにおけるrule value reviewの承認とする。
- 同じURLを再確認して意味が変わっていた場合、既存recordを上書きせず、新しい`SourceReview`とpreset draft versionを追加する。
- sourceが後から消失・変更しても、過去runの設定hashとreview recordは保持する。ただし再確認不能であることをavailabilityへ記録し、`current` alias更新やrelease reviewでは再評価する。
- URLと取得日時だけしか残せない場合もsource recordとして受理できる。ただしlocator、版、対象期間が不明な点をnotesへ明示する。

## Consequences

### Positive

- 非公開storageを理由に第三者資料を無条件で複製する誤りを避けられる。
- repositoryに著作物やscreenshotを蓄積せず、rule valueとreview責任を追跡できる。
- sourceごとに保存可否を法的判断する運用負担を通常経路から外せる。
- project ownerの確認手順とpreset reviewを直接結び付けられる。

### Negative

- 公式sourceが変更・消失した場合、当時の本文を独立に再現できない。
- content hashによる機械的な改定検出や第三者による完全な再監査はできない。
- review品質がlocator、paraphrase、確認者の記録精度に依存する。
- 同じURLのsilent updateは定期的な人手確認が必要になる。

## Alternatives considered

### Private artifact storageへraw bytesを保存する

Rejected as default. 非公開であることだけでは複製・保存の適法性を保証しない。

### hashだけを保存する

Rejected as required evidence. 内容を保持しないhashは後からclaimをreviewする材料にならない。変更検知の補助として個別に使うことは禁止しないが、verifiedの必須条件にはしない。

### 公式資料をGitへcommitする

Rejected. 再配布、repository size、削除要求への対応に問題がある。

## Follow-up / verification

- 最初の対象として、雀魂公式詳細ルールの`SourceReview`と最小`RuleClaim` mappingを作る。
- release前とpreset alias更新時にsource URLを再確認し、確認日時を新しいrecordへ残す。
- sourceの個別保存が必要になった場合は、project ownerの明示的承認と保存範囲を別記録にする。
- 雀魂牌譜については、rule sourceとは異なる利用条件と個人情報を考慮した別ADRで決める。
