# ADR-0003: 大会ドメインを卓内エンジンから分離する

- Status: Accepted
- Date: 2026-08-08
- Deciders: Project owner, LizhiSim design

## Context

LizhiSim は一卓だけでなく、段位戦、league、team 戦、tournament を扱う。サッカー league の season/fixture/standing model は参考になるが、麻雀は一卓 3/4 者の multi-party competition であり、座順、同卓回数、素点、順位点がある。

卓内 engine に season や bracket を入れると状態が巨大化し、局の correctness test と大会 schedule test が結合する。一方、単純な後処理 script だけでは結果依存の次卓割り、失敗、裁定、replay を一貫して扱えない。

## Decision

- Competition を独立 domain とし、`Competition -> Stage -> Round -> TableAssignment` の aggregate を持つ。
- 卓内 engine との interface は `TableMatchSpec` と `TableMatchResult` に限定する。
- schedule/assignment、aggregation、completion、advancement、ranking を差し替え可能な pure policy とする。
- assignment、seat、tie-break、carry-over、penalty、ruling を event 化する。
- standings は raw result と adjustment から再計算可能な view とする。
- 二者 fixture でなく 3/4 者の assignment を中心にし、同卓・座順 balance を policy input にする。
- ranking/matchmaking は competition layer に置き、卓上点数から分離する。

## Consequences

Positive:

- 同じ卓内ルールを異なる league/tournament/ranked policy で再利用できる。
- fixed schedule と動的 matching を共通の event model で扱える。
- 大会全体を決定的に replay できる。
- team roster/lineup、promotion/relegation を局 engine へ持ち込まずに拡張できる。

Negative:

- 卓結果を待つ orchestration と stage lifecycle が必要。
- multi-party balanced scheduling は二者 league より複雑。
- 大会固有の例外を generic policy に過剰一般化する危険がある。
- 長期 ranked simulation では event/standing storage 量が増える。

## Alternatives considered

### 外部 script で半荘を繰り返す

Rejected as the only model. 初期 experiment helper には使えても、動的進出、裁定、再生、failure policy が追跡できない。

### Table engine に tournament state を含める

Rejected. 責務と test が結合し、単一卓 library としての再利用性を失う。

### サッカーの home/away fixture をそのまま使う

Rejected. 一卓 3/4 者、seat、multi-party tie/standing を自然に表現できない。season/stage/standing の概念だけを参考にする。

## Follow-up / verification

- Phase 6 の最初は fixed 8 人/2 卓 schedule と standing の縦切りにする。
- balanced assignment の目的関数と限界を別 ADR で決める。
- ranked queue と finite tournament の共通/相違部分を test で学んでから抽象化する。
- raw results から standing を再構築する property/replay test を作る。
