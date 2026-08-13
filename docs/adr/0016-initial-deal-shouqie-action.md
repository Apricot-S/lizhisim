# ADR-0016: 14枚配牌の親第一打は`Shouqie`だけを提示する

- Status: Accepted
- Date: 2026-08-13
- Deciders: Project owner
- Supersedes: [ADR-0012](0012-normalize-dealer-first-draw.md)

## Context

[ADR-0012](0012-normalize-dealer-first-draw.md)は、親の14枚配牌を13枚の`bingpai`と
`origin = initial_deal`の`zimopai`へ正規化した。その際、分離した14枚目を捨てるactionを
`Moqie`として受け付け、公開結果だけ`moqie = false`へ補正する設計としていた。

しかし14枚配牌ルールの親第一打では、14枚すべてが配牌であり、分離した`zimopai`も手切の
対象である。外部へ提示するaction候補に`Moqie`は存在せず、`Shouqie`が`bingpai`の13枚と
`zimopai`の1枚を合わせた14枚を対象にする。

actionの`Moqie`と、結果の`Sipai::moqie`を異なる意味にすると、合法action、replay、観測、
学習featureで同じvariantを補正して解釈する必要が生じる。

## Decision

- canonical進行は`RoundStarted -> Zimo(origin = initial_deal | live_wall) -> first decision`へ統一する。
- `bingpai`は最大13枚とし、行為対象になった1枚を`zimopai`として分離する。
- 親へ14枚配るruleでは14枚目を`origin = initial_deal`の最初の`Zimo`へ正規化し、親へ13枚配る
  ruleではlive wallからの第一`Zimo`を`origin = live_wall`とする。
- `Dapai::Moqie(tile_kind)`は、live wall等から得た`zimopai`を摸切するactionとし、成功した
  `Sipai`は常に`moqie = true`とする。
- `Dapai::Shouqie(tile_kind)`は手切するactionとし、成功した`Sipai`は常に`moqie = false`とする。
- `origin = initial_deal`の親第一打では、合法action候補に`Moqie`を含めない。
- 同じ条件では、`Shouqie`の候補を`bingpai`と`zimopai`を合わせた牌種から生成する。
- `Shouqie`で`zimopai`と同じ`TileKind`を選んだ場合、分離された`zimopai`を捨て、
  `bingpai`のcountsは変更しない。
- `origin = live_wall`または通常巡目では、`Shouqie`は`bingpai`から指定牌を1枚減らし、
  元の`zimopai`を`bingpai`へ1枚加える。
- `FirstZimoOrigin`は、`moqie`を補正するためではなく、合法action候補と`Shouqie`の牌移動元を
  決める文脈として使う。
- action候補を生成する境界とactionを適用する境界は、同じ条件を共有し、不正な`Moqie`を
  候補から除外するだけでなくcore遷移でも拒否できるようにする。
- `RoundStarted`時点のsource表現が14枚でも、canonical event streamとcore stateには正規化後の
  最初の`Zimo`を含める。source record上の14枚目との対応はservice projectorで検証する。
- rule設定はdealerの開始方式を表すenumとし、`moqie`を独立したrule booleanとして持たない。

## Consequences

### Positive

- action variantと`Sipai::moqie`の意味が一対一になる。
- 14枚配牌ルールの親第一打を、外部へ実際の選択肢どおり提示できる。
- model action、canonical event、replayでorigin依存の`moqie`補正が不要になる。
- 同じ`TileKind`の個別identityを導入せず、配牌14枚からの手切を表現できる。

### Negative

- `Shouqie`の牌移動は、`initial_deal`由来の親第一打と通常巡目で異なる。
- 合法action生成とcore遷移の双方が、最初の`Zimo` originと第一打前の文脈を参照する。
- ADR-0012に基づいて追加した、`initial_deal`由来の`Moqie`を`moqie = false`に補正する実装と
  testは置き換える必要がある。

## Alternatives considered

### `Moqie`を受け付けて結果だけ`moqie = false`にする

Rejected. actionの意味と公開結果が一致せず、外部へ実際には存在しない候補を提示する。

### 14枚配牌だけ`bingpai`を14枚にする

Rejected. 最初のdecisionだけ状態形状が変わり、ADR-0012の正規化による利点を失う。

## Follow-up / verification

- `round-first-dapai` test listで、initial deal由来の親第一打における`Shouqie(zimopai)`を追加する。
- 同条件で`Moqie`を合法action候補に含めず、core遷移でも拒否するtestを追加する。
- live wall由来の`Moqie`が`moqie = true`になる対照testを維持する。
- action候補生成、canonical event、observation、replayへの射影を後続test listで検証する。
