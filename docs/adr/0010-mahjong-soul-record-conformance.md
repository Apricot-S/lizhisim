# ADR-0010: `majsoul-record`を外部入力として二段階検証する

- Status: Accepted
- Date: 2026-08-09
- Deciders: Project owner
- Supersedes: [ADR-0009](0009-minimized-game-record-evidence.md)
- Amends: [ADR-0005](0005-mahjong-soul-first.md)の牌譜evidence手順

## Context

`majsoul-record`はprotobufで表現されたevent列であり、LizhiSimの検証資料として利用できる。`majsoul-record`の取得はproject ownerが別途行い、取得toolはLizhiSimの対象外とする。牌譜IDとIDを付加したreplay URLにはaccess tokenが含まれず、game内の共有機能等で得られる半ば公開のlocatorとしてprojectで利用できる。

完全な`majsoul-record`は容量が増えるためGit管理しない。変換用app crateで`game_log`へ変換し、匿名化・抜粋したtest dataだけをGit管理する。変換済みの完全な`game_log`もGit管理せず、一定量が集まった時点でproject ownerがlocal corpusとして検証する。

ただし、変換済みdataだけを正解としてsimulatorと比較すると、変換toolのbugが期待値を誤らせる危険がある。protobufから何らかのdomain表現への解釈は不可避だが、変換結果を無検証のoracleにしてはならない。

## Decision

### Scope and acquisition

- `majsoul-record`の取得はuserの責任とし、LizhiSim repositoryにdownloader、scraper、network client、認証処理を実装しない。
- 変換用app crateは明示的に指定されたlocal `majsoul-record` fileだけを入力にし、networkから取得しない。
- 完全な`majsoul-record`と変換済みの完全な`game_log`はGit管理しない。保存場所、backup、retentionはuser管理とし、repositoryのCIやrelease artifactへ含めない。
- 牌譜IDとreplay URLはsource locatorとしてGitへ記録できる。access tokenとして扱う必要はない。ただしplayer名等の不要な情報をURL周辺のmetadataとして併記しない。
- player名とservice内player identifierは変換時に除去する。fixture内participantは、そのfixture内だけで有効な連番等へ変換する。

### Converter architecture

変換用app crateの内部を次の三層に分ける。

```text
majsoul-record bytes
  -> wire decoder
  -> Mahjong Soul event projector
  -> LizhiSim record writer / conformance verifier
```

1. **Wire decoder**はprotobuf messageをsource順序付きのservice eventへdecodeする。未知message、未知field、decode不能dataを黙って捨てず、source event indexとともにerrorまたは明示的unknownとして返す。
2. **Mahjong Soul event projector**はservice eventを、比較に必要な観測事実へ変換する。tile/action、actor、points、`Round`境界等を解釈するが、LizhiSimの遷移結果を期待値として生成しない。
3. **Record writer / conformance verifier**は、匿名化・最小化した`game_log`の出力、またはLizhiSim eventとの逐次比較を行う。decodeとrule解釈を同じ巨大functionへまとめない。

変換結果を一度生成して固定oracleにするだけの構成は禁止する。full-record検証では原則としてlocal `majsoul-record`を毎回decodeし、source event indexごとにLizhiSimへ観測actionを与え、双方の公開状態・points・局面結果をcheck pointで比較する。

### Converter verification

converter自体を次で検証する。

- projectが生成したsynthetic protobuf messageによるwire decode test
- encode/decode可能な範囲のround-trip test
- event順序、actor、tile conservation、point conservation、`Round`境界のinvariant test
- unknown message/field、schema version不一致、切断dataを黙って受理しないnegative test
- source event indexから変換eventへのtrace mapping
- userによるgame内replay表示と、少数の変換結果のspot review
- schema/projector変更時の既知local corpus再実行

synthetic testはconverterのmechanicsを検証するが、service意味論の正しさを単独では保証しない。実牌譜の逐次照合とuser reviewを併用する。

### Tier 1: constructed corner-case tests

- 特定corner caseを最小のdomain scenarioへ編集し、Git管理する。
- たとえば`rong`優先順位だけを検証する場合、不要な巡目を除き1巡目に同じ条件を作るなど、testの意図が明確な最小状態へ変形できる。
- player名、service player ID、不要な時刻・profile metadataを含めない。
- source牌譜ID、対象`Round`、source event範囲、どの部分を編集したかをprovenanceとして記録する。
- 編集済みtestは実牌譜そのものの完全再現ではなく、牌譜から発見した`constructed regression`と明記する。
- 通常CIに含め、すべてのpush/PRで実行する。

### Tier 2: full-record conformance tests

- user管理のlocal `majsoul-record` corpusを入力にし、変換用app crateとLizhiSimで対局全体を逐次検証する。
- 完全な`majsoul-record`、変換済みの完全な`game_log`、player名mappingはGit管理しない。
- corner caseの元牌譜を含め、corpusが一定量増えるごと、converter変更時、rule engine変更時、release前にuserが実行する。
- 通常CIの対象外とする。CI環境はcorpusを取得せず、secret storageも要求しない。
- 実行reportにはcorpus件数、牌譜ID、converter/schema version、成功・失敗event index、未対応event、集計値を残せる。reportをGitへ入れる場合はplayer名等を含まないことを確認する。

### Hash and provenance

- Git管理するTier 1 fixtureはfile bytesのSHA-256を持てる。
- local Tier 2では`majsoul-record` file hashと`game_log` hashをrun reportへ記録できるが、完全なfile自体はGitへ入れない。
- 牌譜IDはsource locatorとして記録し、hashの代替ではなく併記する。
- converter/schema versionを必ず記録し、同じraw inputでもconverter変更による差分を追跡する。

## Why raw input still needs a converter

`majsoul-record` eventとLizhiSim eventはschemaも抽象度も異なるため、直接比較にもdecodeとsemantic mappingは必要である。重要なのは変換をなくすことではなく、変換済み`game_log`を唯一の真実にしないことである。

full-record testでrawを毎回読み、decoder/projectorを独立testし、source event indexまでtraceすることで、converter bugを局所化できる。さらにprojectorが期待するLizhiSim結果を作るのではなく、観測actionと観測後状態だけを抽出することで、simulatorとoracleの実装共有を避ける。

## Consequences

### Positive

- 完全な`majsoul-record`をGitへ置かず、大規模corpusをlocalに増やせる。
- 小さいcorner-case testは高速でCI実行できる。
- full-record testは変換済みsnapshotではなくrawから毎回検証できる。
- decoder、projector、simulatorのbugをsource event indexで切り分けやすい。
- 牌譜IDを使ってtestの出所を追跡できる。

### Negative

- converter自体に独立したtest suiteとversion管理が必要になる。
- Tier 2はCIで常時実行されず、user運用に依存する。
- raw schema変更時にdecoder/projectorとlocal corpusを再検証する必要がある。
- 最小testへの編集が元牌譜と異なる条件を作らないようreviewが必要になる。

## Alternatives considered

### 変換済み完全牌譜だけをoracleにする

Rejected. converter bugを固定化し、simulatorが正しくても誤判定し得る。

### `majsoul-record`をdomain coreで直接読む

Rejected. service固有schemaとdecode errorをpure domainへ漏らし、取得元変更やschema evolutionをcoreへ結合する。

### 完全な`majsoul-record`をGitへcommitしてCIで全件実行する

Rejected. repository size、実行時間、player情報、service dataの管理負担が大きい。

### Corner-case testだけを持つ

Rejected. 編集時に落としたcontextや長い対局進行の差異を検出できない。

## Follow-up / verification

- 実装開始時に変換用app crateの責務と依存方向をworkspace ADRへ追加する。
- converter、Tier 1、Tier 2をそれぞれ別test listに分ける。
- Tier 2の実行command、corpus discovery、匿名化reportを実装時に文書化する。
- 最初のcorner caseでsource eventからLizhiSim check pointまでのtraceをuser reviewする。
