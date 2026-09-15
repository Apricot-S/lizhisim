# ADR-0018: ユーザー設定をRuleSet、検証済み設定をValidatedRuleSetとする

- Status: Accepted
- Date: 2026-09-15
- Deciders: Project owner
- Relates to: [ADR-0002](0002-versioned-rule-layers.md)、[ADR-0017](0017-core-depends-on-rules.md)

## Context

ユーザーは`RawRuleSpec`と`RuleSpec`の`Spec`を省き、ユーザー設定を無印、
検証済み設定を`Validated`付きとする提案を採用した。
本ADRは名称の追加判断であり、既存ADRの責務・依存方向・検証境界は変更しない。

## Decision

- `RawRuleSpec`を`RuleSet`、`RuleSpec`を`ValidatedRuleSet`、
  `RuleSpecError`を`RuleSetValidationError`へ改名する。旧名のaliasは残さない。
- moduleは`rule_set`とする。ユーザー設定は公開fieldで構築でき、
  検証済み設定は非公開fieldを持ち、`TryFrom<RuleSet>`で検証して構築する。
- 現在の`ValidatedRuleSet`が保証するのは、M/P/Sの赤牌枚数が各0〜4であることだけである。
  人数やengineの対応能力の検証、serde/TOML adapterは今回追加しない。
- 設計上の`ValidatedRuleSet<P>`は人数・semantic・capability validationを含む将来像である。
  現在の非genericな型と保証範囲を区別する。
- `TableRules`、`MatchRules`等の層分離を維持する。`RuleSet`は全層を巨大な万能設定型にまとめる指示ではない。
- 過去のADR本文・cycle logにある旧名は当時の記録として残し、現在の設計文書とコードは新名に統一する。

## Consequences

- ユーザー設定の名前が短くなり、coreの引数型から検証済みであることが分かる。
- 公開型名が変わるため、呼出側は新名へ移行する必要がある。
- 検証済み型名は長くなる。型名だけで将来の検証まで実装済みと解釈しないよう保証範囲を記す。

## Alternatives considered

- `RawRuleSet`から`RuleSet`への変換は、ユーザー設定を無印にする採用方針と一致しない。
- `Rules`も候補だが、既存の`ValidatedRuleSet<P>`・`RuleSetId`との整合を優先する。

## Follow-up / verification

- [rulesとcoreのtest list](../test-lists/rules-core-dependency.md)で新名による直接利用を検証する。
- 既存の正常系・境界・エラーpayloadのtestを維持し、workspace標準検証を行う。
