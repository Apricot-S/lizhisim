# lizhisim-rules

ルール設定の検証と、検証済み`ValidatedRuleSet`の参照APIを提供します。
`lizhisim-core`へ依存せず、coreがこのcrateの公開型を直接利用します。
`TileKind`と`TileSet`、設定から牌構成への解決はcoreが所有します。

`RuleSet`はユーザーが構築する未検証の設定です。`ValidatedRuleSet::try_from`で検証し、失敗時は`RuleSetValidationError`を返します。
現在の検証範囲はM/P/Sの赤牌枚数が各0〜4であることです。人数やengineの対応能力の検証は未実装です。

`RuleSet.first_zimo_origin`には`FirstZimoOrigin::Qipai`（親に14枚配牌）または`FirstZimoOrigin::Bipai`（親に13枚配牌後、第一ツモ）を明示指定します。
検証前後で同じenumを使い、`ValidatedRuleSet::first_zimo_origin()`で取得できます。
