# ロードマップ

ロードマップは期限ではなく、依存関係と品質 gate を示す。各 milestone は小さい vertical slice の test list に分解し、t-wada TDD で進める。Phase 0 中は実装しない。

## Phase 0 — 構想と設計（現在）

成果物:

- [x] 旧 Rust 実装と旧依存 manifest の撤去
- [x] ビジョン、要求、用語
- [x] event-driven typed continuation architecture
- [x] rule/preset versioning design
- [x] competition/ranked/league/tournament design
- [x] inference queue/batching design
- [x] t-wada TDD 開発手順と template
- [x] 公式ルール出典台帳の初版
- [x] 初期 ADR

Exit gate:

- 文書 review で blocking contradiction がない。
- 下記の実装前決定が完了する。
- ユーザーが実装開始を明示する。

## Implementation Gate A — Toolchain と最初の slice

実装開始前に決定する。

- [x] MSRV 1.97 / Edition 2024（manifest で固定済み。toolchain profile は未決）
- [x] 最小 workspace/crate boundary（root workspace + `lizhisim` library scaffolding）
- [ ] format、lint、test、dependency audit の CI command
- [ ] canonical serialization と stable hash の候補
- [ ] async runtime を必要とする時点と選定基準
- [ ] 最初の基準プリセット（雀魂段位戦・四人）
- [ ] 最初の active test list

Decision output:

- toolchain ADR
- workspace ADR
- initial schema draft
- first test list

## Phase 1 — 決定的な一局 walking skeleton

Goal: GPU、network、実 scoring crate なしで、固定牌山の小さな一局断片を要求・応答・event・replay まで通す。

Vertical slices:

1. `PlayerSet`、`Seat`、`Points`、IDs と、glossary で確定した牌関連の検証済み値型
2. fixed `FourPlayer` rule fixture と固定牌山
3. 配牌から最初の action request を発行して suspension
4. typed response validation から action event を確定
5. call 候補なし -> next draw
6. canonical event stream -> replay state hash

Exit gate:

- 不正 seat/tile/state の type/validation tests
- delivery order から独立した replay
- core に async/RNG/I/O 依存なし
- property test で牌 conservation

## Phase 2 — 四人一局の完結

Goal: 四人麻雀の通常局を和了/流局まで正しく完結する。

Slices:

- 合法手生成、フリテン、リーチ
- call window、チー/ポン/明槓、複数ロン
- 暗槓/加槓、槍槓、嶺上、ドラ
- exhaustive/abortive draw
- `xiangting` adapter contract
- `hule` port/fake、利用可能なら adapter contract
- 支払 ledger と `RoundResult`

Exit gate:

- 既知 scenario corpus
- call response permutation property
- points/tile conservation
- adapter capability mismatch が開始前に拒否される

## Phase 3 — 四人対局と最初の verified preset

Goal: 東風/東南など一つの対局を終え、雀魂段位戦・四人 preset を最初の `verified` にする。

Slices:

- 連荘、本場、供託、対局進行
- all-last、延長、飛び、同点
- オカ/ウマ/順位精算
- rule schema/canonical hash/registry
- source mapping と golden scenario
- complete match replay

Exit gate:

- verified preset 1 件
- match result と event stream の安定 schema
- source audit review 完了

## Phase 4 — 三人麻雀

Goal: 三人麻雀固有差分を first-class type と rule で扱い、雀魂段位戦・三人 preset を `verified` にする。

Slices:

- tile set と seat type
- 北抜きと replacement/槍槓
- チー禁止と call priority
- 三麻ツモ支払 variation、本場、ノーテン
- three-player match progression/settlement
- 雀魂段位戦・三人 preset

Exit gate:

- four-player path への条件分岐汚染がない
- 三麻 property/contract/golden tests
- verified three-player preset 1 件

## Phase 5 — 非同期多数卓と推論 batching

Goal: 多数卓の suspension を安全に所有し、fake backend から GPU backend へ段階的に batch する。

Slices:

- table scheduler、drive budget、lifecycle
- request envelope、pending registry、idempotency
- compatibility buckets と flush policy
- call-window barrier across async responses
- timeout/retry/cancel/backpressure
- fake/CPU backend contract
- GPU backend と deterministic sampling key
- metrics/benchmark harness

Exit gate:

- duplicate/late response で二重再開しない
- batch partition/delivery permutation で結果一致
- graceful/checkpoint/immediate shutdown
- baseline throughput/memory report と次期目標 ADR

## Phase 6 — Competition

Goal: 複数卓結果を stage/standing/advancement へ統合する。

Slices:

- fixed schedule + standing
- balanced multi-party assignment
- cut/advancement/final
- ranking policy + ranked queue
- teams/lineup/team standing
- promotion/relegation/hybrid stage graph
- failure/ruling/adjustment events

Exit gate:

- league と tournament の end-to-end replay
- assignment/tie-break determinism
- table rule と ranking policy の独立合成

## Phase 7 — Preset coverage

Goal: [必須 catalog](design/rules-and-presets.md#7-必須プリセット-catalog) を verified へ進める。

推奨順:

1. 雀魂段位戦 四人/三人
2. 天鳳段位戦 四人/三人
3. 麻雀一番街段位戦 四人/三人
4. 龍龍 四人/三人
5. WRC、M リーグ
6. 最高位戦、日本プロ麻雀連盟、日本プロ麻雀協会、麻将連合、RMU variations

各 family は個別 test list と review を持つ。数を優先して `draft` を `verified` にしない。

## Phase 8 — Dataset と分散実行

Goal: 実験データを安全に出力し、意味論を変えず scale-out する。

- trajectory projector と schema evolution
- durable event/snapshot store
- experiment manifest/data lineage
- multi-process/remote inference
- worker loss/recovery
- dataset validation、information leak check
- profiling と performance tuning

## Release gates

### Alpha

- 四人/三人各 1 verified preset
- single-process async batching
- complete replay/integrity
- fixed competition 1 形式

### Beta

- 必須競技 preset の主要 family
- online 四人/三人の verified preset（証跡が得られたもの）
- ranked/league/tournament
- stable event/trajectory schema policy
- hule integration/license gate 解消

### 1.0

- 必須 catalog の対象範囲を明文化し、未対応条項ゼロまたは明示的除外
- correctness corpus、property/model tests、replay suite
- benchmark と運用限界
- schema/preset compatibility policy
- public documentation

## 現在の blocking/open decisions

| Decision | Needed by | 状態 |
|---|---|---|
| `hule` の API/license/capability | Phase 2/Release | Blocked: 未公開 |
| Rust MSRV/edition | Gate A | Decided: Rust 1.97 / Edition 2024 |
| `rust-toolchain.toml` profile/components | Gate A | Open |
| canonical serialization/hash | Phase 1 | Open |
| observation/action schema | Phase 1/5 | Open |
| async runtime | Phase 5 | Open; Phase 1 では不要 |
| event/trajectory storage | Phase 5/8 | Open |
| 雀魂牌譜 evidence の保存・匿名化・hash 方法 | Gate A/Phase 3 | Open |
| 最初の verified preset | Gate A | Decided: 雀魂段位戦・四人（ADR-0005） |
| performance target hardware | Phase 5 | Open |

open は作業停止を意味しない。必要になる直前まで test で学び、不可逆な選択だけを ADR で先に固定する。
