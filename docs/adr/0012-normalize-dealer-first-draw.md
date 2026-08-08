# ADR-0012: 親の14枚配牌を最初の`Zimo`へ正規化する

- Status: Accepted
- Date: 2026-08-09
- Deciders: Project owner

## Context

雀魂や最高位戦では、親は14枚の配牌から最初の行為を選ぶ。一方、天鳳やMリーグのように全自動卓の進行を基準にするruleでは、親は13枚の配牌後に第一`Zimo`を行う。

実際の配牌状態をそのまま二種類のtypestateにすると、最初のdecisionだけ`bingpai`が14枚になる。通常の`Zimo`後に使う`zimopai`との表現差が、合法手生成、観測schema、学習feature、replayへ波及する。

反対に14枚配牌を通常のlive wallからの`Zimo`と同一視すると、親の第一`Dapai`が`moqie`かという公開情報を誤る。

## Decision

内部のcanonical進行を次に統一する。

```text
RoundStarted
  -> Zimo(origin = initial_deal | live_wall)
  -> first decision
```

- `bingpai`は最大13枚とし、行為対象になった1枚を`zimopai`として分離する。
- 親へ14枚配るruleでは、14枚目を`origin = initial_deal`の最初の`Zimo`へ正規化する。
- 親へ13枚配るruleでは、live wallからの第一`Zimo`を`origin = live_wall`とする。
- `origin = initial_deal`の牌を直後に`Dapai`した場合、公開eventと学習featureでは`moqie = false`とする。
- `origin = live_wall`の牌を直後に`Dapai`した場合、`moqie = true`とする。
- rule設定はdealerの開始方式を表すenumとし、`moqie`を独立したrule booleanとして重複設定しない。`moqie`は`Zimo`のoriginと選択した`Dapai`から導く。
- `RoundStarted`時点のsource表現が14枚でも、canonical event streamとcore stateには正規化後の最初の`Zimo`を含める。
- source recordが14枚目のidentityをどう表すかはservice projectorの契約testで確定し、根拠なく並び順を仮定しない。

概念上の設定値は次の二値である。schema上の識別子はinitial schema設計時に確定する。

```text
dealer_first_draw_origin
  = initial_deal
  | live_wall
```

雀魂段位戦は`initial_deal`とする。天鳳、Mリーグ、最高位戦の値は、それぞれのsource mappingで確認してからpresetへ設定する。

## Consequences

### Positive

- 最初から通常巡目まで`bingpai`と`zimopai`の形を統一できる。
- 最初のdecisionを常に`Zimo`後として扱える。
- 観測schemaとmodel featureに14枚配牌専用分岐を持ち込まずに済む。
- source上の`moqie`情報を保持できる。

### Negative

- 14枚配牌のsource recordとcanonical event列が一対一にはならず、syntheticな最初の`Zimo`をtraceする必要がある。
- 利用者は`origin = initial_deal`の`zimopai`が配牌の一部であることを理解する必要がある。
- service projectorは14枚目のidentityとsource locatorを正しく対応付ける必要がある。

## Alternatives considered

### 実際の14枚配牌状態をそのまま表す

Rejected. 実物の状態に近いが、最初だけ`bingpai`の上限と合法遷移が異なり、型、観測、学習featureに専用経路が必要になる。

### 14枚目を通常のlive wall由来として扱う

Rejected. 状態形状は統一できるが、第一`Dapai`の`moqie`情報を誤る。

## Follow-up / verification

- initial rule schemaで開始方式のenumとvalidationを定義する。
- 雀魂walking skeletonに、initial deal由来の牌を最初に打つと`moqie = false`となるtestを追加する。
- live wall由来の第一`Zimo`では同じ操作が`moqie = true`となる対照testを追加する。
- `majsoul-record` projectorで14枚目のidentityとsynthetic event traceを検証する。
