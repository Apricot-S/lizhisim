# レビューと変更単位

差分のレビュー時に、変更した領域の観点を確認する。 [開発手順の入口](../development-guide.md)。節番号は移設前の参照との対応を保つ。

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
- 一例へのハードコードでもgreenになる場合、次の一項目で三角測量したか。
- より強いtestに包含され、独立した契約を持たなくなったtestを削除したか。
- 三角測量testを削除した場合、削除理由と契約を引き継ぐtestをcycle logへ記録したか。
- refactor 後に全 suite が green か。

## 11. Git と変更単位

- 一つの変更は一つの設計意図または一つの TDD slice を中心にする。
- commit を作る場合は `docs:`, `test:`, `feat:`, `fix:`, `refactor:`, `perf:`, `build:`, `ci:` など Conventional Commits の type を使う。
- red の途中状態を共有 branch の最終 commit にしない。red を commit する運用を採る場合も、その直後の green と組で review できるようにする。
- formatting だけの変更を意味変更と混ぜない。
- 自動生成物、大容量 trajectory、公式 PDF の複製、game 内 screenshot を無計画に Git へ入れない。証跡 storage 方針を決めてから扱う。
