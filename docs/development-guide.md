# 開発手順書

## 1. 現在の作業範囲

現在は Phase 0 であり、許可される成果物は要求、設計、ADR、調査台帳、test list の雛形である。Cargo workspace と空の library crate scaffolding は存在するが、ユーザーが実装開始を明示するまで振る舞いを持つ Rust コード、crate、dependency を追加しない。

文書だけを変更する場合も、変更理由、要求 ID、文書間リンク、未決事項を確認する。設計が変わる場合は ADR を追加または supersede する。

識別子を追加する前に [用語集](glossary.md) を確認する。対応する行がなければ「ユーザー決定待ち」表へ空欄で追加し、ユーザーがピンインまたは英語識別子を決めるまで production 名を作らない。日本語ローマ字やその場限りの英訳で仮置きしない。

## 2. 実装開始 gate

次を満たしてから最初の Rust 変更へ進む。

- Phase 0 文書が review 済み。
- 最初の vertical slice と対象プリセットが決まっている。
- manifest に記載済みの Edition 2024 / Rust 1.97 と workspace 境界を ADR で追認している。
- `xiangting` の採用版と license を確認している。
- `hule` の取得方法、license、API、capability が確認できるか、明示的な test double 期間が承認されている。
- 最初の test list が作成されている。
- CI の最小 command が決まっている。

## 3. t-wada TDD の基本サイクル

すべての production behavior は次の順で作る。

```text
test list
  -> one test selected
  -> red
  -> green
  -> refactor
  -> update test list
  -> next one
```

### 3.1 Test list を作る

作業単位ごとに `docs/test-lists/<topic>.md` を作り、[template](templates/test-list.md) を使う。要求 ID、根拠、例、境界、エラー、property 候補を思いつく限り列挙する。最初から実装順に固定する必要はなく、途中で気づいた項目を追加する。

test list はテストコードの一覧ではなく、振る舞いの仮説と不安の一覧である。項目は観測可能な結果で書く。

良い例:

- 「頭ハネ設定では、下家のロン応答が先着しても上家のロンが採用される」
- 「三麻で除外した二萬を赤牌に指定した設定は validation error になる」

悪い例:

- 「`resolve()` をテストする」
- 「coverage を上げる」

### 3.2 1 つだけ選ぶ

最小で、設計上の学びが大きく、短く green にできる項目を一つ `Current` にする。関連する複数例を一度に選ばない。新しい疑問が出たら test list へ追加し、現在の red を広げない。

### 3.3 Red

失敗する最小テストを書く。

- compile failure を狙う type test か、runtime assertion failure かを明確にする。
- 意図した理由で失敗していることを読む。
- fixture/setup の失敗や typo を red と数えない。
- 既存テストが別理由で壊れている場合は先に切り分ける。
- red の command と要点を cycle log に記録する。

型で禁止すべき不正状態は compile-fail test を検討する。ただし compile-fail snapshot の保守コストと価値を比べる。

### 3.4 Green

選んだ一項目だけを通す最小実装を行う。

- 将来の一般化を先回りしない。
- まだ test list にある別のケースを同時実装しない。
- ハードコードが現在例だけを偽装する場合は、次の代表例を test list からすぐ選ぶ。
- green 後に関連 suite と全 suite を実行する。

### 3.5 Refactor

全テスト green のまま改善する。

- 名前を domain 用語へ揃える。
- 重複を除く。
- primitive を value type へ寄せる。
- 不正状態を型へ押し出す。
- effect を shell 側へ移す。
- function/enum の責務と exhaustive match を見直す。

refactor で新しい振る舞いを追加しない。必要なら test list へ戻り、新しい red から始める。

### 3.6 記録して繰り返す

対象項目を完了し、気づいた項目を追加する。未完了一覧がゼロになっても、要求・property・integration の観点で漏れを review してから topic を完了する。

## 4. Vertical slice の選び方

最初に牌型 library 全体、次に queue 全体という horizontal 実装を避ける。一つの小さな scenario を end-to-end に通す。

推奨する最初の slice 例:

1. 固定済み四人ルールと固定牌山を検証して一つの `Round` を開始する。
2. 一人へ捨牌要求を発行して typed continuation で中断する。
3. 合法な捨牌応答を返す。
4. 誰にも鳴き候補がなければ次のツモ状態へ進む。
5. event log から同じ終端 hash を再生する。

この一つを test list でさらに小さく分ける。GPU、network、実 `hule` がなくても fake backend/port で core の形を検証できる。

## 5. テスト戦略

### 5.1 Example tests

pure transition の代表例。Arrange は完全な validated state builder を使い、無効状態を struct literal で捏造しない。

### 5.2 Table-driven tests

点数境界、順位精算、preset 差分、終了条件など、同じ規則の入力と期待値を並べる。

### 5.3 Property tests

候補:

- 牌 conservation
- 明示 source/sink を除く point transfer conservation
- seat rotation equivariance
- legal action の encode/decode round trip
- request delivery order/batch partition に対する終端結果不変
- canonical serialization/hash の安定性
- replay の状態一致

shrinking 後の反例を regression test へ残す。

### 5.4 Model-based state-machine tests

小さい参照 model と command 列を生成し、typestate engine の event/result を比較する。鳴き窓、槓、リーチ、流局は状態数が多いため、example test の次に導入する。

### 5.5 Contract tests

同じ suite を fake と実 adapter に適用する。

- `XiangtingPort` と `xiangting` adapter
- `HuleEvaluationPort` と `hule` adapter
- inference backend
- event store

adapter の capability declaration も contract の一部にする。

### 5.6 Golden scenarios

公式プリセットの出典差分を最小 scenario で固定する。大きな牌譜 JSON の snapshot だけに頼らず、「どの条項を守る test か」を要求 ID/source clause と結び付ける。

golden 更新は review 対象であり、無条件の snapshot accept を禁止する。

### 5.7 Differential/replay tests

信頼できる外部結果がある場合に比較する。ただし外部 service の非公開挙動を真実と決めず、対象版と出典を記録する。event replay は毎 milestone の Must とする。

### 5.8 Performance tests

correctness test と分ける。固定 workload と環境 metadata を持ち、次を計測する。

- pure transition throughput
- scheduler throughput
- batch fill/latency
- memory per live table
- replay speed

性能を理由に correctness assertion を削らない。profile で hot path を確認してから最適化し、前後の benchmark を保存する。

## 6. Test double

- **Fake**: domain port の単純で決定的な実装。core TDD に使う。
- **Stub**: 特定例だけの応答。test 内に限定する。
- **Spy**: request や event の内容・順序を記録する。
- **Mock**: interaction contract 自体が要求の場合だけ使う。

外部 crate の内部呼出順を mock して implementation detail に固定しない。入力と出力、capability、error mapping を contract test する。

## 7. `xiangting` と `hule` の導入

### 7.1 `xiangting`

1. version、MSRV、license、公開 API、三麻牌構成の扱いを確認する。
2. domain の `XiangtingPort` contract を fake で red/green にする。
3. adapter に同じ contract suite を適用する。
4. 既知牌姿と property/differential corpus を追加する。
5. crate 型を adapter 外へ export していないことを review する。

### 7.2 `hule`

未公開の間は production 相当の独自点数計算器を作って代替しない。port、context、capability、fake と test corpus を先に設計する。利用可能になった時点で次を確認する。

- source の取得・pin 方法
- license と配布可否
- MSRV と feature
- 役、符、上限、責任払い、三麻、local variation の対応範囲
- error semantics
- deterministic behavior

未対応 variation を LizhiSim adapter が補う場合は、その責務と根拠を ADR にし、独立 test list から実装する。

## 8. Rule preset の開発手順

1. source 台帳を更新する。
2. 対象 family/版と対応範囲を決める。
3. 全条項の mapping checklist を作る。
4. 未確認・矛盾・対象外を分類する。
5. test list を作り、差分一つずつ red/green にする。
6. canonical 完全設定を生成し hash を固定する。
7. review と golden suite 後に `verified` にする。

同時に多数 preset を埋めず、最初の一つで schema の不足を学び、次の preset を追加する。schema 変更で既存版の canonical content が変わる場合は migration/新 schema 版として扱う。

### 8.1 実装順

1. 雀魂段位戦（四人/三人）
2. 天鳳段位戦（四人/三人）
3. 麻雀一番街段位戦（四人/三人）

雀魂では [公式詳細ルール](https://mahjongsoul.com/news/46)を一次資料にする。記載外の corner case は [Cryolite/kanachan `src/simulation`](https://github.com/Cryolite/kanachan/tree/main/src/simulation) と [牌譜 ID 付き検証記録](https://gist.github.com/Cryolite/a026f41713f6a7ca88713737f5c2cfb6)から候補を得る。ただし先行実装の出力だけを期待値にせず、牌譜 ID の元牌譜を取得し、raw data の hash、対象 `Round`、期待 event を test evidence として固定する。

元牌譜を取得できない corner case は test list に残し、理由付きで `blocked` とする。先行実装からコードをコピーせず、状態分割・規則差分・牌譜 locator を調査資料として利用する。

## 9. Bug 修正

1. 再現条件を test list へ追加する。
2. 最も小さい層で再現テストを書く。
3. 意図した失敗を確認する。
4. 最小修正で green にする。
5. 同種の入力を property/table test へ広げる。
6. replay/preset/source への影響を確認する。

イベントや公式ルールが誤っていた場合、過去データの無言修正はしない。affected version/run を記録し、新版または migration 方針を用意する。

## 10. Review checklist

### Domain

- 不正状態を型または validation で拒否しているか。
- 遷移が全域で、古い状態の部分更新を残さないか。
- async、clock、RNG、I/O が core に漏れていないか。
- ルールをサービス名で分岐していないか。

### Protocol

- schema/version/hash を照合しているか。
- duplicate/late/cancelled response が continuation を再開しないか。
- call-window の結果が応答順に依存しないか。
- observation に非公開情報が漏れていないか。

### Rules

- 一次資料、版、対象期間があるか。
- 未確認値を default で埋めていないか。
- unsupported/physical clause を追跡しているか。
- golden scenario が出典差分を説明するか。

### Competition

- raw result と adjustment を分けているか。
- tie-break と failure policy が total か。
- assignment の乱数と座順が event に残るか。

### Tests

- test list の一項目から始めたか。
- red を意図した理由で確認したか。
- green の最小性を保ったか。
- refactor 後に全 suite が green か。

## 11. Git と変更単位

- 一つの変更は一つの設計意図または一つの TDD slice を中心にする。
- commit を作る場合は `docs:`, `test:`, `feat:`, `fix:`, `refactor:`, `perf:`, `build:`, `ci:` など Conventional Commits の type を使う。
- red の途中状態を共有 branch の最終 commit にしない。red を commit する運用を採る場合も、その直後の green と組で review できるようにする。
- formatting だけの変更を意味変更と混ぜない。
- 自動生成物、大容量 trajectory、公式 PDF の複製、game 内 screenshot を無計画に Git へ入れない。証跡 storage 方針を決めてから扱う。

## 12. コマンド

Phase 0 の検証:

```powershell
git diff --check
git status --short
rg --files
```

Rust の標準検証 command は次のとおりである。

```powershell
cargo fmt -- --check
cargo clippy -- -D warnings
cargo build --verbose
cargo test --verbose
cargo deny check
markdownlint-cli2 "**/*.md" "#target/**"
lychee --offline --no-progress --root-dir . "**/*.md"
```

通常のCI jobもversionを別指定せず、repositoryの`rust-toolchain.toml`を使ってformat、Clippy、build、testを実行する。CIは加えてnightlyの`cargo docs-rs`を実行する。`cargo deny check`は`deny.toml`を使う独立workflowで実行する。cargo-deny Actionの`rust-version`はtoolchain fileから自動取得されないため、`rust-toolchain.toml`更新時に同じ値へ更新する。Markdown lintとrepository内リンク検査も独立workflowで実行し、外部URLはmerge gateに含めない。ローカルcommandは各toolを導入済みの場合に利用できる。空scaffoldingがbuildできることと、ドメインの振る舞いが検証済みであることを混同しない。

## 13. Definition of Done

### 文書

- 要求 ID、用語、リンク、ADR が整合する。
- 決定と未決事項が区別されている。
- 変更した公式情報に出典と確認日がある。
- `git diff --check` が成功する。

### 実装（開始後）

- 対象 test list 項目が完了し cycle log がある。
- red を確認し、最小 green、refactor を経ている。
- 関連 test、全 test、format、lint が成功する。
- error、event、metrics、replay への影響を検証している。
- public schema、rule preset、ADR、開発文書を必要に応じ更新している。
- benchmark-sensitive な変更は比較結果がある。
