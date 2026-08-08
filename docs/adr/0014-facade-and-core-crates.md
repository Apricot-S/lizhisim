# ADR-0014: facadeとcore crateを分離する

- Status: Accepted
- Date: 2026-08-09
- Deciders: Project owner
- Amends: [ADR-0006](0006-rust-toolchain-workspace-and-ci.md)の初期workspace境界

## Context

ADR-0006はPhase 0のscaffoldingを単一の`lizhisim` crateとし、具体的な分割をwalking skeletonまで延期した。最初の`Seat` testを実装する時点で、利用者向けcrateをBurnのようなre-export専用facadeにし、実装を別crateへ置く方針がproject ownerから指定された。

facadeがdomain実装を直接所有すると、将来crateを分離した際に公開pathが変わる。最初から公開入口と実装所有権を分ければ、内部境界を変更しても利用者向けpathを維持できる。

## Decision

- package `lizhisim`は利用者向けfacadeとし、domain型や振る舞いを実装しない。
- package `lizhisim-core`が`Seat`を含むwalking skeletonのdomain実装を所有する。
- `lizhisim`は`lizhisim-core`の公開APIをre-exportする。
- 局所的なdomain不変条件のunit testは、実装所有crateの対象source moduleへ同居させる。公開facadeの互換性や複数moduleをまたぐ契約を検証する場合だけ`tests/`へintegration testを追加する。
- 依存方向は`lizhisim -> lizhisim-core`とし、逆依存を禁止する。
- facade固有のruntime、global state、I/O、rule分岐を追加しない。
- `lizhisim-core`以外のcrate分割はwalking skeletonから必要性を確認して決める。

## Consequences

### Positive

- 利用者は安定した`lizhisim`pathからAPIを利用できる。
- 内部crate境界を公開pathから分離できる。
- domain testと実装の所有場所が一致する。
- facadeに振る舞いが混入していないことをreviewしやすい。

### Negative

- 最小実装でもworkspace packageと依存辺が一つ増える。
- coreの公開APIを無条件にglob re-exportすると、意図しないAPIまでfacadeから公開され得る。
- facadeの公開面を制御するため、APIが増えた時点で明示的re-exportへ切り替える検討が必要になる。

## Alternatives considered

### `lizhisim`が実装を所有する

Rejected. project ownerが指定したfacade構成と一致せず、後の分割で公開pathが変わる。

### 最初から責務ごとに多数のcrateへ分割する

Rejected. `lizhisim-core`より細かい境界はwalking skeletonからまだ確認できない。

## Follow-up / verification

- `Seat`のred testを`lizhisim-core/src/seat.rs`へ置く。
- facade crateにdomain実装が追加されていないことをreviewする。
- 公開APIが増えた時点でglobと明示的re-exportのどちらを使うか再評価する。
