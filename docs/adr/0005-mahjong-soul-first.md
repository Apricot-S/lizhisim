# ADR-0005: 雀魂段位戦を最優先のルール実装・検証対象にする

- Status: Accepted
- Date: 2026-08-09
- Deciders: Project owner, LizhiSim design

## Context

初期ロードマップでは、公開一次資料が明確な競技ルールを最初の verified preset 候補としていた。しかし深層学習用 simulator の正しさを高めるには、文書だけでなく大量の実牌譜で corner case を検証できることが重要である。

雀魂には膨大な牌譜があり、Cryolite/kanachan の `src/simulation` には雀魂牌譜との照合を通じて発見・対応された先行知見がある。検証記録には corner case の牌譜 ID も掲載されている。天鳳は雀魂との差分が比較的小さく牌譜が公開され、麻雀一番街も差分が比較的小さい。

## Decision

ルール family の実装・検証順を次に固定する。

1. 雀魂段位戦（四人/三人）
2. 天鳳段位戦（四人/三人）
3. 麻雀一番街段位戦（四人/三人）

雀魂の evidence chain は次とする。

1. [雀魂公式詳細ルール](https://mahjongsoul.com/news/46)を一次資料にする。
2. [Cryolite/kanachan `src/simulation`](https://github.com/Cryolite/kanachan/tree/main/src/simulation) と [検証記録](https://gist.github.com/Cryolite/a026f41713f6a7ca88713737f5c2cfb6)から記載外 corner case と牌譜 ID を発見する。
3. 牌譜 ID から元牌譜を取得し、raw data の hash と対象 `Round` を固定する。
4. 元牌譜から regression/golden test を作り、LizhiSim の event と照合する。

牌譜の取得・保存・匿名化手順はADR-0009で一度改定し、その後[ADR-0010](0010-mahjong-soul-record-conformance.md)で再改定する。現在はuserがproject外で取得した`majsoul-record`を、変換用app crateによるCI用最小`game_log` fixtureとCI外full-record testの二段階で利用する。

先行実装そのものを一次資料または期待値の唯一の根拠にはしない。元牌譜を取得できない項目は `blocked` とする。

## Consequences

Positive:

- 初期から実データに基づく高い corner-case coverage を得られる。
- 先行実装の知見を調査 index として活用できる。
- 天鳳・麻雀一番街へ差分駆動で展開できる。

Negative:

- 公式文書だけでは完結せず、牌譜取得・保存・匿名化・hash 管理が必要になる。
- 雀魂固有仕様へ core を過適合させる危険がある。
- 三人麻雀固有機能を早い段階で設計する必要がある。

## Alternatives considered

### WRC/Mリーグ等の競技ルールを最初にする

Rejected as first priority. 文書監査はしやすいが、雀魂ほど大規模な機械検証用牌譜を得にくい。

### Kanachan の挙動をそのまま正解とする

Rejected. source code は有力な参考資料だが、公式ルールと元牌譜へ根拠を戻せなければ独立検証にならない。

## Follow-up / verification

- Implementation Gate A の最初の preset を雀魂段位戦・四人へ変更する。
- 三人麻雀 milestone の最初の preset を雀魂段位戦・三人にする。
- 牌譜 evidence の保存形式、個人情報除去、content hash を実装前 ADR で決める。
