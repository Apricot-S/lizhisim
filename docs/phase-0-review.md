# Phase 0 review

Phase 0から実装へ進む判断を、文書の存在ではなく確認可能なgateとして管理する。ユーザーが実装開始を明示するまでは、全項目を満たしてもPhase 1へ移行しない。

## 1. Product and architecture

- [x] 対象、非対象、成功指標が定義されている。
- [x] Gym/`step()`非対応とevent-driven typed continuationがAccepted ADRになっている。
- [x] functional core / imperative shellと依存方向が定義されている。
- [x] 卓内、対局、大会、段位方針が分離されている。
- [x] inference request、typed suspension、call-window barrierが設計されている。

## 2. Language and development baseline

- [x] 用語集の初期識別子がユーザー確認済みである。
- [x] `Round`と`Matchday`の使い分けが文書全体で一貫している。
- [x] Rust MSRV、Edition、開発toolchainが固定されている。
- [x] 最小workspaceと空library crateがある。
- [x] format、Clippy、build、test、docs-rsのCIがある。
- [x] `cargo-deny`によるdependency auditの設定と独立CI workflowがある。
- [x] Markdown lintとローカルリンク検査の独立CI workflowがある。

production dependency追加時は`deny.toml`のlicense、ban、source方針をreviewする。外部URLの定期検査はmerge gateとは分離し、必要になった時点で検討する。

## 3. Rules and evidence

- [x] 必須preset catalogと実装優先順位が定義されている。
- [x] 雀魂段位戦・四人が最初の基準presetに決定している。
- [x] 公式ルール、Kanachan、検証記録、元牌譜のevidence chainが定義されている。
- [x] 雀魂公式ページを複製せず、URL・取得日時・確認記録で追跡する方式を決める（[ADR-0008](adr/0008-source-review-without-copying.md)）。
- [x] 雀魂牌譜の取得、匿名化、最小CI test、1パスfull-record検証、保存・hash方針を決める（[ADR-0011](adr/0011-one-pass-majsoul-conformance.md)）。
- [x] [walking skeletonが必要とする最小`RuleClaim` mapping](references/mahjong-soul-walking-skeleton-rule-claims.md)を作る。

source未確定の値を推測してfixtureへ埋めない。pure engineの構造だけを試すfixtureは、公式presetと明確に区別する。

## 4. First TDD slice

- [x] test list templateがある。
- [x] [最初のplanned test list](test-lists/mahjong-soul-ranked-four-player.md)がある。
- [x] ユーザーの明示指示により、最初のtest itemとして四人用`Seat`の構築範囲testを一つだけ選択した。
- [ ] canonical serialization / `StateHash`方式を、最初に必要となる時点までに決める。
- [ ] observation/action/eventのinitial schema draftを、対応testを選ぶ前に作る。

## 5. Explicit implementation gate

実装開始には次の三条件をすべて必要とする。

1. blocking contradictionがないことを文書reviewで確認する。
2. 選択するtest itemに必要な未決事項とsource evidenceが解決済みである。
3. ユーザーがPhase 1または具体的なtest itemの実装開始を明示する。

開始時はplanned test listから一項目だけを選び、redを確認する。複数項目をまとめてactiveにしない。
