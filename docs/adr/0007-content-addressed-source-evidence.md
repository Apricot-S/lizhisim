# ADR-0007: 公式資料をcontent-addressed evidenceとして保存する

- Status: Superseded
- Date: 2026-08-09
- Deciders: Project owner, LizhiSim design
- Superseded by: [ADR-0008](0008-source-review-without-copying.md)

## Context

公式Webページは同じURLのまま更新され得るため、URLと取得日時だけではpresetの根拠を再検証できない。一方、公式ルール本文、PDF、game内画面をGitへそのまま複製すると、repositoryの肥大化、著作権、公開範囲の問題が生じる。

雀魂段位戦の最初のsourceは版が明示されないWebページである。取得時点のbytesを固定しつつ、Gitにはreview可能なmetadataと必要最小限のrule claimだけを残す方式が必要である。

このADRは公式Webページ、PDF、game内help等のsource evidenceを対象とする。雀魂牌譜の取得、個人情報除去、保存は後続の判断に分離する。

## Decision

### Evidence object

- 取得物は変更不能なbinary objectとして保存し、object identityを`sha256:<lowercase-hex>`とする。
- SHA-256は保存対象の正確なbytesに対して計算し、media typeとbyte lengthを同時に記録する。
- HTTP sourceはtransfer/content encodingを展開したresponse bodyを`http_body` objectとして保存する。圧縮方式や転送分割の違いをsource内容の違いにしない。
- client-side renderingが根拠の表示に必要な場合は、`rendered_dom`と必要最小限の`screenshot`を別objectとして保存し、それぞれhashを持たせる。DOMや画像のhashをHTTP bodyのhashで代用しない。
- PDF、画像、添付fileは取得したfile bytesをそのままhashする。再保存、印刷、画像変換した派生物は別objectにする。

### Manifest

Gitにはraw objectではなく、review可能な`SourceSnapshot` manifestを保存する。manifestは少なくとも次を持つ。

```text
SourceSnapshot
  source_id
  organization_or_service
  title
  canonical_url
  final_url
  redirect_chain
  retrieved_at_utc
  document_version
  effective_from / effective_to
  locale / region / app_version
  request_profile
  response_status
  etag / last_modified
  objects[]
    role: http_body | rendered_dom | screenshot | pdf | attachment
    sha256
    byte_length
    media_type
    storage_uri
  evidence_grade
  supersedes
  reviewer
  notes
```

- `request_profile`には再取得へ必要なlanguage、region、user-agent class、認証有無を記録する。secret、cookie、tokenは記録しない。
- manifestのauthoring形式はPhase 0ではTOMLを第一候補とするが、canonical preset serializationの決定とは分離する。source identityはmanifest bytesではなくevidence objectのhashへ結び付ける。
- 同じURLを再取得してbytesが変わった場合、既存manifest/objectを上書きせず新しいsnapshotを追加する。

### Storage boundary

- raw evidence objectはGitへcommitしない。access control、immutable/versioning、backupを持つprivate artifact storageへ保存する。
- logical storage keyは`sha256/<first-two-hex>/<remaining-hex>`とする。物理backendのbucket名やdirectoryはmanifestへ埋め込まず、`storage_uri`は移送可能なlogical URIにする。
- repositoryにはmanifest、source locator、設定値のparaphrase、必要最小限の短い引用、review記録だけを置く。ページ全文やfull screenshotを置かない。
- local cacheは再取得可能なcacheとして扱い、唯一の保管場所にしない。cacheはGit管理対象外とする。
- artifact storage backend自体の製品選定とcredential管理は、最初のevidence取得作業までにimperative shell/運用設定として決める。domainやpreset schemaへbackend固有情報を漏らさない。

### Verification and status

- 取得直後と利用直前にbytes、byte length、SHA-256を照合する。
- `RuleClaim`は`source_id`に加え、object hashとsection/page/screen等のlocatorを参照する。
- raw objectへ到達できないsnapshotは`metadata_only`として扱い、そのsourceだけに依存するclaimを`verified`にしない。
- robots、利用規約、認証境界を迂回して取得しない。保存または再配布が許容できない場合も、無断転載せず`metadata_only`または`blocked`とする。
- evidence objectの削除が必要になった場合、manifestを消さずtombstone、理由、日時を記録し、依存するpresetを再評価する。

## Consequences

### Positive

- 同じURLが更新されても、presetが依存した取得物をhashで識別できる。
- Gitを肥大化させず、公開repositoryとraw evidenceのaccess policyを分離できる。
- HTTP body、rendered DOM、screenshotを混同せず、どの表現がclaimの根拠か追跡できる。
- storage backendを後から変更してもcontent identityを維持できる。

### Negative

- private artifact storageとbackupの運用が必要になる。
- repositoryだけではraw evidenceを完全reviewできず、reviewerにもstorage accessが必要になる。
- dynamic pageでは複数objectの取得とlocator管理が必要になる。
- exact bytesの保存可否をsourceごとに確認する作業が増える。

## Alternatives considered

### URLと取得日時だけを保存する

Rejected. 同じ内容を再取得できず、改定前後の差を証明できない。

### WebページやPDFをGitへ直接commitする

Rejected. 著作権、公開範囲、repository sizeの問題を一律に解決できない。

### 正規化した本文だけをhashする

Rejected. normalization ruleの変更でidentityが変わり、元の取得bytesを検証できない。検索用のnormalized textは派生objectとして追加できる。

### screenshotだけを保存する

Rejected. 人間には読めても機械的な条項mappingと差分検出が難しく、非表示metadataや全文を保持できない。

## Follow-up / verification

- 最初の対象として雀魂公式詳細ルールの`SourceSnapshot` manifestを作る。
- artifact storage backendとlogical `storage_uri` schemeを、実際のsnapshot取得前に決める。
- 雀魂牌譜には個人情報とservice固有dataが含まれるため、別ADRで保存・匿名化・hash境界を決める。
- canonical preset serializationと`StateHash`はevidence object hashとは別ADRで決める。
