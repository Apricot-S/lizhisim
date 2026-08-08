# ADR-0013: 37種類の`TileKind`を使い牌の個別identityを持たない

- Status: Accepted
- Date: 2026-08-09
- Deciders: Project owner

## Context

通常の34種類に対して、赤`5m`、赤`5p`、赤`5s`は行為、宝牌、観測featureで区別する必要がある。一方、同じ種類の通常牌4枚を何枚目かまで区別する必要は麻雀のrule上存在しない。

天鳳のsource dataは牌を0〜135のIDで表すが、雀魂は文字列で表し、競技麻雀でも同種牌の個体識別はできない。天鳳固有IDをcoreへ持ち込むと、他sourceに存在しない情報がcanonical stateやagent観測へ混入する。

## Decision

- `TileKind`は通常の34種類に赤`5m`、赤`5p`、赤`5s`を加えた37種類とする。
- core domainに`TileCopy`または同種牌の個別identityを表す型を置かない。
- 同じ`TileKind`の複数枚は個数またはmultisetとして表す。
- `Bipai`は重複を許す`TileKind`の順序として記録する。
- `bingpai`、`zimopai`、`fulu`、`he`、宝牌表示、canonical eventも`TileKind`を使う。
- canonical serialization、`StateHash`、agentの`Observation`にsource固有の牌IDを含めない。
- 天鳳の0〜135 ID等はservice decoder/projector内でsource event traceに利用できる。coreへprojectするときに`TileKind`へ変換する。
- 学習featureが同種牌を複数tokenへ展開する場合、実物の個別identityではなく、そのfeature内の決定的な出現順でtokenを作る。
- `xiangting`や`hule`向けの34要素count表現はadapter境界で作り、赤牌を対応する通常の5へ射影する。
- 34種類の射影だけを表す新しいdomain識別子は、必要性が確認されるまで作らない。

## Consequences

### Positive

- 雀魂、天鳳、競技ruleに共通する観測可能な意味だけをcoreへ持てる。
- 赤牌を行為とfeatureで直接区別できる。
- 牌山、状態、event、hashがsource固有IDに依存しない。
- 同種牌の個体識別による不正なagent情報を防げる。

### Negative

- 天鳳牌譜との差分reportで物理IDを示す場合、projector側のsource traceを併記する必要がある。
- 外部libraryの34種類indexへ毎回明示的に射影する必要がある。
- multisetから同種牌を移動するとき、個体ではなく個数の増減として検証する必要がある。

## Alternatives considered

### 34種類の`TileKind`と赤flagを分ける

Rejected. 赤牌を選ぶ行為、牌山、event、featureの各所で組を持つ必要があり、無効な組合せを作りやすい。

### `TileKind`と`TileCopy`を分ける

Rejected. 同種牌の個体差はcoreのrule意味論に不要で、source間に存在しない能力をagentへ漏らす危険がある。

### 天鳳の0〜135 IDを共通identityにする

Rejected. 雀魂や競技麻雀へ適用できず、天鳳source encodingをdomain modelへ結合する。

## Follow-up / verification

- walking skeletonのtest listを37種類の`TileKind`と個数validationへ変更する。
- 37種類の牌構成から34要素count表現へのadapter contract testを追加する。
- 天鳳projector実装時にsource IDとprojected `TileKind`のtrace testを作る。
- canonical schemaに個別牌IDが混入していないことをreviewする。
