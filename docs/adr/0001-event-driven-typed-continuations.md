# ADR-0001: イベント駆動・型付き継続を採用する

- Status: Accepted
- Date: 2026-08-08
- Deciders: Project owner, LizhiSim design

## Context

深層学習用自己対戦では多数の麻雀卓を同時に進め、意思決定要求を GPU batch にまとめる必要がある。麻雀卓は局進行、鳴き、連荘により非同期で、打牌後には複数 actor の応答を待つ場合がある。

Gym 型の同期 `step(action) -> observation` は、一つの環境と一つの action を中心にする。vector 化しても同じ時点に揃えるか padding が必要で、call window、cancel、retry、late response の意味が interface 外へ漏れる。また任意 state に任意 action を渡せる API は、合法性を runtime check に偏らせる。

## Decision

- Gym API と Gym 互換層を提供しない。
- 公開 `step()` を中心 API にしない。
- pure domain transition は現在の typestate を消費し、次状態、完了、または `Request<R>` と `Continuation<R>` を結ぶ型付き中断点を返す。
- 継続は closure ではなく閉じた data enum として defunctionalize する。
- 異種 request は transport 境界だけで versioned envelope に型消去し、応答検証後に型を復元する。
- call window は複数 response slot を持つ一つの suspension とし、応答到着順ではなくルールで解決する。
- imperative shell が queue、async runtime、timeout、GPU、I/O を担当する。

## Consequences

Positive:

- 卓ごとの非同期性を自然に表現できる。
- model/schema ごとに要求を横断 batch 化できる。
- 状態と期待 response の対応を型で表現できる。
- 同時ロン、鳴き優先、late/duplicate response を protocol として検証できる。
- core tests は async runtime/GPU なしで決定的に実行できる。

Negative:

- typed internal state と heterogeneous queue の間に型消去・復元層が必要。
- continuation の durable snapshot には closed enum と schema evolution が必要。
- 一般的な RL library の Gym wrapper をそのまま使えない。
- scheduler、backpressure、idempotency を初期から設計する必要がある。

## Alternatives considered

### Gym/Gymnasium + vector env

Rejected. 卓を同期させる歪みと call window の表現不足があり、GPU batching の主導権も environment 外へ移る。

### Actor ごとの async callback/closure

Rejected. closure の永続化、clone、schema evolution、監査が難しく、任意 captured state が再現性を損なう。

### 一つの巨大な runtime state enum と `step(Event)`

Rejected as public/core model. transport 最外層の enum は必要でも、domain の状態別合法性が弱くなる。内部は typestate と typed continuation を保つ。

## Follow-up / verification

- Phase 1 で discard suspension の最小 walking skeleton を TDD する。
- request delivery order と batch partition を変えて終端 hash が一致する property test を作る。
- duplicate/late/cancel response で continuation が二重再開しない model test を作る。
- durable continuation の schema は storage 選定時に別 ADR で決める。
