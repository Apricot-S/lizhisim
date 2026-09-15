# ADR-0019: 親の開始方式をrulesのFirstZimoOriginで設定する

- Status: Accepted
- Date: 2026-09-15
- Deciders: Project owner
- Relates to: [ADR-0016](0016-initial-deal-shouqie-action.md)、[ADR-0017](0017-core-depends-on-rules.md)、[ADR-0018](0018-rule-set-validation-names.md)

## Context

coreの`FirstZimoOrigin`は局の開始方式を表していたが、ルール設定を経ずに`Round`へ渡していた。
ユーザーはrulesへの移動、公開・検証後APIでのenum共有を採用し、variantを配牌・壁牌のピンインにするよう指定した。
本ADRは所有先と設定経路を追加し、既存variantの名称を更新する。ADR-0016の打牌意味論は維持する。

## Decision

- `FirstZimoOrigin`を`lizhisim-rules`が所有し、`RuleSet.first_zimo_origin`で必ず指定する。
- variantは`Qipai`（配牌）と`Bipai`（壁牌）とする。旧`InitialDeal`・`LiveWall`のaliasは設けない。
- `Qipai`は親へ14枚配牌する方式である。内部で分離した14枚目も配牌に属する。
  `Bipai`は親へ13枚配牌した後、壁牌から第一ツモを得る方式である。
- 公開APIと検証後APIは同じenumを使う。二つの選択肢は型で制限し、boolや検証後専用enumへ変換しない。
- `ValidatedRuleSet::try_from`は赤牌枚数を検証し、開始方式をそのまま保持する。
  この二方式の間に追加の組合せ制約やdefaultは導入しない。
- `Round::new`は`&ValidatedRuleSet`を受け取り、`first_zimo_origin()`から値をコピーして局中保持する。
  設定への参照を局中保持するlifetimeや、rules全体のcloneは追加しない。
- coreはrulesのenumを利用する。公開pathは`lizhisim_rules::FirstZimoOrigin`とfacadeの
  `lizhisim::rules::FirstZimoOrigin`とし、coreからの旧re-exportは削除する。
- `Qipai`での親第一打の摸切拒否と、分離した牌の手切を維持する。`Bipai`では通常の打牌意味論を使う。
  配牌・ツモの正規化、牌山cursor、牌移動、第一巡資格の更新方法は変更しない。
- この設定は局開始方式であり、暗槓・北抜き後などにおける現在のツモ牌の由来を表す可変状態ではない。
- 過去のADRとcycle logの旧variant名は履歴として保持する。新しいevent/schemaは今回実装しない。

## Consequences

- ユーザー設定から検証済み設定を経てcoreへ開始方式が渡り、core専用のルール型が不要になる。
- 既存の`RuleSet`構築側には明示的な開始方式の追加が必要になる。
- `Bipai`はこのenum内では開始方式の選択肢、coreの型名では牌山を意味する。
- 現在の検証範囲を拡張するが、人数・capability検証や入力adapterは追加しない。

## Alternatives considered

- 親第一打を手出しとするboolは、開始方式から導く打牌意味論を独立設定にしてしまうため採用しない。
- 配牌枚数の数値設定は時点が曖昧になり、13・14以外の値の検証も必要になるため採用しない。
- 検証後だけ別enumとする案は、同じ選択肢の複製と変換が増えるため採用しない。

## Follow-up / verification

- [rulesとcoreのtest list](../test-lists/rules-core-dependency.md)で設定からRoundへの保持を検証する。
- 親第一打の手切・摸切、通常巡目、局終端の既存testを新しい設定経路へ移行し維持する。
- 未実装の合法action列挙・event・暗槓・北抜きは各test listの対象項目を選択してから実装する。
