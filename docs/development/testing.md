# テスト戦略とtest double

対象の振る舞いに必要な検証方法を選ぶときに参照する。 [開発手順の入口](../development-guide.md)。節番号は移設前の参照との対応を保つ。

## 5. テスト戦略

選択したtest list項目に応じて以下を適用する。pure transitionの例示テストを最優先とし、ルール境界は表形式、広い入力空間はproperty test、状態機械はmodel-based testで検証する。同時ロン、鳴き競合、キャンセル、遅延応答、再送には決定的なスケジューラテストを用意する。replayはイベント列と終端状態の安定hashの両方を検証する。

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
