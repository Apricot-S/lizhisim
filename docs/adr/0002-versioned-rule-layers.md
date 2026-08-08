# ADR-0002: ルールを層別化し出典付き不変版として管理する

- Status: Accepted
- Date: 2026-08-08
- Deciders: Project owner, LizhiSim design

## Context

リーチ麻雀の「ルール」には、牌・役・点数、半荘終了、league 集計、段位 point、matchmaking が混在する。同じ団体にも複数 variation があり、オンライン service は卓内ルールと段位制度を別々に改定し得る。

service 名を engine の条件分岐にすると差分が追跡できず、preset 継承を可動なまま保存すると親更新によって過去実験の意味が変わる。公式資料は Web/PDF/game 内に分散し、完全に確認できない場合もある。

## Decision

- `TableRules`, `MatchRules`, `CompetitionPolicy`, `RankingPolicy` を別の versioned layer とする。
- Raw 設定を schema/semantic/capability validation し、engine は完全な `ValidatedRuleSet<P>` だけを受け取る。
- preset の canonical form は継承差分でなく解決済み完全設定とする。
- preset version は不変 ID、schema version、content hash、effective period、status、source mapping を持つ。
- `current` alias は開始時に不変版へ解決し、run manifest には保存しない。
- 未確認値を一般的 default で埋めず、`draft`/`blocked` のまま実行対象から外す。
- 物理/審判条項も source mapping し、computational scope 外として明示する。
- 公式改定は旧版を編集せず、新しい版と差分 test を追加する。

## Consequences

Positive:

- 過去実験を同じ意味で再生できる。
- 卓内ルールと段位/大会方式を自由に組み合わせられる。
- 公式差分と test の対応を説明できる。
- 未対応条項を silent ignore せず release gate にできる。
- rule comparison と sensitivity experiment が可能になる。

Negative:

- source audit と preset review の作業量が大きい。
- schema evolution と canonical hash の方針が必要。
- game 内にしかない仕様は証跡取得まで blocked になる。
- variation の family/版設計を誤ると catalog が増えすぎる。

## Alternatives considered

### Service ごとの hand-written rule class

Rejected. 差分の重複と更新漏れが増え、組合せと比較が難しい。

### 一つの巨大な JSON 設定

Rejected. 卓内、大会、段位の不変条件と更新周期が混ざる。authoring 表現として JSON/YAML を使うこと自体は可能だが、検証後の domain type は分ける。

### Preset inheritance を runtime で解決

Rejected. 親版の移動で過去 run の意味が変わる。authoring helper としてのみ許し、保存前に完全解決する。

### 非公式 Wiki を実用上の正解とする

Rejected. 更新履歴と権威が不足する。一次資料の欠落検出には利用できる。

## Follow-up / verification

- Gate A で canonical serialization/hash を ADR 化する。
- Phase 3 で一次資料が明確な四人 preset 一つを最初に verified にする。
- 各 config path に source claim を結ぶ仕組みを設計する。
- schema migration と old preset replay を test する。
