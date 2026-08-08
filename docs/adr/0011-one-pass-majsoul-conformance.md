# ADR-0011: `majsoul-record`を1パスで逐次conformance検証する

- Status: Accepted
- Date: 2026-08-09
- Deciders: Project owner
- Supersedes: [ADR-0010](0010-mahjong-soul-record-conformance.md)
- Amends: [ADR-0005](0005-mahjong-soul-first.md)の牌譜evidence手順

## Context

ADR-0010は、`majsoul-record`を`game_log`へ変換するapp crateと、CI用最小fixture・CI外full-record testの二段階検証を定めた。しかし完全牌譜検証で変換済み`game_log`を一度書き出してから読み直すと、中間fileが古いconverterの解釈を固定し、converter bugをoracleとして扱う危険がある。中間fileの保存、version、migration、二重実行も必要になる。

raw protobuf eventとLizhiSim eventのschemaは異なるため、decodeとsemantic mapping自体は避けられない。重要なのは変換をなくすことではなく、変換済みfileを通常のfull-record検証経路から外し、source eventからLizhiSimとの比較までを一回の実行でtrace可能にすることである。

CI用corner-case testは、実牌譜を機械変換したfixtureより、検証対象の遷移を明示した手書きの最小scenarioが適する。

## Decision

### Conformance app

- 独立した変換製品を作らず、service固有依存をcoreから隔離するconformance app crateを作る。
- appはuserがlocalに置いた`majsoul-record`だけを入力にし、牌譜取得、network access、認証処理を持たない。
- 通常のfull-record検証では、完全な`game_log`を中間fileとして生成・読込しない。
- `game_log`出力は、projectorの調査、他toolとの連携、bug report等で必要な場合だけ使う任意の診断機能とする。出力をconformance oracleにしない。

### One-pass pipeline

```text
majsoul-record bytes
  -> wire decoder
  -> Mahjong Soul event projector
  -> conformance driver
       -> observed actionをLizhiSimへ入力
       -> checkpointごとに期待/実際を比較
  -> diff reporter
```

1. **Wire decoder**はprotobufをsource順序付きservice eventへdecodeする。
2. **Event projector**はservice eventから、観測actionと比較可能な公開事実を抽出する。
3. **Conformance driver**はactionをLizhiSimへ逐次入力し、source event indexごとに状態とresultを比較する。
4. **Diff reporter**は最初の不一致を人間がgame画面まで追跡できる形で報告する。

各層は別component・別test対象とする。1パスを一つの巨大functionとして実装しない。未知message、未知event、decode不能data、未対応projectionを黙ってskipしない。

projectorはLizhiSimが生成すべき内部eventを組み立てない。sourceから観測できるaction、points、actor、`Round`境界、公開状態だけを抽出し、同じrule実装をsimulatorとoracleで共有しない。

### Tier 1: minimal transition tests

- 必要な遷移を明示した最小scenarioを原則として手書きする。
- corner caseの発見元として牌譜ID、対象`Round`、source event範囲を記録できる。
- 実牌譜の不要な巡目を機械的に残さず、たとえば`rong`競合を1巡目に構成するなど、test意図に必要な状態だけを作る。
- fixtureは`game_log`またはdomain test builderで表現できる。どちらを標準にするかはinitial schema設計で決める。
- `majsoul-record` importerを通さないため、converter bugとdomain testを分離できる。
- Git管理し、通常CIへ含める。

### Tier 2: full-record conformance tests

- user管理のGit外`majsoul-record` corpusを入力に、1パスpipelineで完全対局を検証する。
- 完全な`majsoul-record`と任意出力した完全な`game_log`はGit管理しない。
- corpusが一定量増えたとき、decoder/projector変更時、rule engine変更時、release前にuserが実行する。
- 通常CIには含めず、CIがcorpus取得やsecret storageを要求しない。
- player名はreportへ出さず、fixture内だけで有効なparticipant IDへ匿名化する。通常のLizhiSim生成`game_log`ではagent名を匿名化しない。

### Diff report

defaultでは最初の不一致で停止し、少なくとも次を出力する。

```text
ConformanceDifference
  record_id
  round
  source_event_index
  source_event_kind
  projected_action
  checkpoint_kind
  expected
  actual
  differing_fields
  preceding_event_context
  state_summary
  converter_version
  schema_version
  rule_preset_id
```

- `state_summary`は調査に必要なpoints、`bingpai`、`fulu`、`he`、`chang`、`ben`、`lizhibang`等を、公開範囲とprivacy方針に従って要約する。
- debug optionで前後event contextを広げ、複数差分を収集できる。ただし後続差分が最初の状態ずれの連鎖であることを明示する。
- 牌譜IDとsource event indexからgame画面で該当箇所を確認できるようにする。
- machine-readable reportと人間向けsummaryを同じ比較結果から生成できる設計にする。

### Decoder and projector verification

- project生成のsynthetic protobuf messageによるdecode test
- encode/decode可能範囲のround-trip test
- event順序、actor、tile conservation、point conservation、`Round`境界のinvariant test
- unknown message/field、schema不一致、切断dataのnegative test
- source event indexからprojected observationへのtrace test
- 少数例についてgame内replayとprojected factsをuserがspot review
- decoder/projector変更後の既知local corpus再実行

synthetic testだけでservice意味論が正しいとはみなさず、Tier 2とspot reviewを併用する。

### Hash and provenance

- Tier 1 fixtureはsource牌譜ID、編集意図、fixture file hashを記録できる。
- Tier 2 reportは`majsoul-record` file hash、牌譜ID、decoder/projector/schema versionを記録できる。
- 任意の`game_log`診断出力を保存する場合は、入力hashとconverter versionを併記する。
- 牌譜IDとreplay URLはaccess tokenを含まないsource locatorとして利用できる。

## Consequences

### Positive

- 変換済みfileを古いoracleとして固定しない。
- 最初の不一致をsource event indexまで追跡できる。
- Tier 1のdomain testがconverter実装から独立する。
- 完全`game_log`の保存・migration・二重実行が通常経路では不要になる。
- service固有protobuf依存をdomain coreへ漏らさない。

### Negative

- full-record testのたびにdecode/projectが必要になる。
- converterだけの出力を目視するには診断optionを明示的に使う必要がある。
- diff reporterとtrace情報を丁寧に設計しないと不一致調査が難しくなる。
- Tier 2がCI外であるため、実行頻度はuser運用に依存する。

## Alternatives considered

### 常に`majsoul-record`を`game_log`へ変換してから検証する

Rejected as the primary path. converter bugとstale outputをoracleとして固定し、中間file管理も増える。

### Domain coreがprotobufを直接読む

Rejected. service固有schema、decode error、schema evolutionをpure coreへ結合する。

### Converterを完全に廃止する

Rejected. raw schemaとdomain schemaの差を解釈するcomponentは不可避である。独立製品にはしないが、decoder/projectorとして明確に分離する。

### Tier 1も実牌譜から自動生成する

Rejected as default. test意図が不要な進行に埋もれ、converter bugの影響も受ける。手書きscenarioを基本とし、自動生成は補助に限定する。

## Follow-up / verification

- 実装開始時にconformance app crateの責務と依存方向をworkspace ADRへ追加する。
- decoder/projector、Tier 1、Tier 2、diff reporterを別test listに分ける。
- initial schema設計でTier 1を`game_log`とdomain builderのどちらで表現するか決める。
- Tier 2の実行command、corpus discovery、report schemaを実装時に文書化する。
- 最初のcorner caseでsource eventからLizhiSim checkpointまでのtraceとdiff表示をuser reviewする。
