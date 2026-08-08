# 要求仕様

## 1. 文書の読み方

要求 ID は設計、test list、テスト名、変更履歴から参照する。優先度は Must / Should / Could で表す。Must は最初の安定版に必要、Should は設計で阻害してはならない、Could は拡張点を確保する。

## 2. 卓内シミュレーション

| ID | 優先度 | 要求 |
|---|---|---|
| CORE-001 | Must | 四人麻雀と三人麻雀を、それぞれ不変条件が検証された型として実行できる。 |
| CORE-002 | Must | 配牌、ツモ、打牌、チー、ポン、明槓、暗槓、加槓、リーチ、ロン、ツモ、流局、途中流局を、ルールで有効な場合に処理できる。 |
| CORE-003 | Must | 三麻の使用牌、チー禁止、北抜き、ツモ損または補正、本場、順位点を個別設定できる。 |
| CORE-004 | Must | 行為の優先順位と複数ロンを、応答到着順に依存せずルールどおり決定する。 |
| CORE-005 | Must | フリテン、リーチ後の制約、槓成立時点、嶺上・海底・河底、ドラ更新の境界を明示的に表現する。 |
| CORE-006 | Must | 外部から不正 action を受けても状態を部分更新せず、型付きエラーまたは設定済みの失格方針へ遷移する。 |
| CORE-007 | Must | 局の遷移は古い状態を消費し、次状態、完了、または型付き中断点のいずれかを返す。 |
| CORE-008 | Must | 通常のドメイン遷移は panic せず、入力領域に対して全域関数として扱える。 |

## 3. 手牌評価と点数計算

| ID | 優先度 | 要求 |
|---|---|---|
| SCORE-001 | Must | シャンテン数計算を `xiangting` crate のアダプター越しに利用する。 |
| SCORE-002 | Must | 点数計算を `hule` crate のアダプター越しに利用する。未公開期間は同じ port の test double で core 開発を分離できる。 |
| SCORE-003 | Must | 外部 crate の型、エラー、版をドメイン公開 API へ漏らさない。 |
| SCORE-004 | Must | crate の対応範囲外またはルール差分をアダプターの後処理で黙って補わず、契約上の capability として検証する。 |
| SCORE-005 | Must | 既知牌姿、境界点、役満複合、符計算、三麻支払について契約テスト corpus を持つ。 |
| SCORE-006 | Should | 同一 port に対する参照実装または differential test を追加できる。 |

## 4. ルール設定とプリセット

| ID | 優先度 | 要求 |
|---|---|---|
| RULE-001 | Must | 使用牌、赤牌、行為、和了、役、符・翻、支払、流局、連荘、半荘終了、精算を細分化して設定できる。 |
| RULE-002 | Must | Raw 設定を検証し、実行系には `Validated` な完全設定だけを渡す。 |
| RULE-003 | Must | 雀魂・天鳳・麻雀一番街・龍龍の段位戦について、四人/三人のプリセット family を用意する。 |
| RULE-004 | Must | M リーグ、WRC、最高位戦、日本プロ麻雀連盟、日本プロ麻雀協会、麻将連合、RMU のプリセット family を用意する。 |
| RULE-005 | Must | プリセットに不変 ID、schema version、対象期間、状態、出典、確認日、内容 hash を持たせる。 |
| RULE-006 | Must | 公式値を確認できないプリセットは実行可能な verified として公開しない。 |
| RULE-007 | Must | 旧プリセットを破壊的更新せず、新しい版を追加する。 |
| RULE-008 | Must | カスタム設定は、どの verified preset から何を変更したかに依存せず、解決済み完全設定として保存できる。 |
| RULE-009 | Should | 2 つのルール設定を意味のある項目名で比較し、差分 report を生成できる。 |

## 5. 半荘・対局

| ID | 優先度 | 要求 |
|---|---|---|
| MATCH-001 | Must | 東風、東南、その他の局構成、連荘条件、延長戦、終了目標、アガリ止め、聴牌止め、飛び終了を設定できる。 |
| MATCH-002 | Must | 座順と起家、開始点、供託、本場を含む半荘初期条件を明示する。 |
| MATCH-003 | Must | 同点順位、順位点、オカ、返し点、残供託を設定可能な精算方針で決める。 |
| MATCH-004 | Must | 半荘終了結果に、得点、順位、精算内訳、使用ルール ID、event range を含める。 |
| MATCH-005 | Should | 時間打切りを wall clock そのものではなく、外部から注入される管理イベントとして表現できる。 |

## 6. AI 意思決定と観測

| ID | 優先度 | 要求 |
|---|---|---|
| AI-001 | Must | 各 seat に見える情報だけから観測を生成し、完全情報の debug view と型で区別する。 |
| AI-002 | Must | 要求に request ID、table ID、actor、decision kind、観測 schema、合法 action、model key、continuation token を含める。 |
| AI-003 | Must | 応答を request ID、schema、actor、合法 action 集合、解決済み状態と照合する。 |
| AI-004 | Must | 打牌後の鳴き・ロン候補を同一 call window に属する要求群として発行し、必要な応答が揃ってから解決する。 |
| AI-005 | Must | 遅延、重複、未知、キャンセル済みの応答を区別し、結果を監査可能にする。 |
| AI-006 | Must | action ID と観測 schema を版管理し、モデル metadata と照合する。 |
| AI-007 | Should | 方策 logits、選択 action、value、補助 head の出力を拡張可能な応答 envelope で扱う。 |
| AI-008 | Should | trajectory に情報漏洩がないことを自動検査できる。 |

## 7. キュー、バッチ、スケジューリング

| ID | 優先度 | 要求 |
|---|---|---|
| QUEUE-001 | Must | 論理卓を互いに非同期に進行させ、一卓の待機が他卓を止めない。 |
| QUEUE-002 | Must | model key、schema、device、推論 shape が互換な要求だけを同じ batch にする。 |
| QUEUE-003 | Must | 最大 batch、最大待機、優先度、backpressure、卓ごとの outstanding 上限を設定できる。 |
| QUEUE-004 | Must | queue の順序や thread scheduling が麻雀ルール上の優先順位を変えない。 |
| QUEUE-005 | Must | 推論失敗、timeout、キャンセル時の方針を明示し、pure core へ結果イベントとして注入する。 |
| QUEUE-006 | Should | CPU、単一 GPU、複数 GPU、remote inference を同じ port の実装として差し替えられる。 |
| QUEUE-007 | Should | live table の公平性と GPU 効率のトレードオフを計測できる。 |

## 8. 大会・段位・リーグ

| ID | 優先度 | 要求 |
|---|---|---|
| COMP-001 | Must | Competition、Stage、Round、Table Assignment、Table Match、Standing を卓内状態から分離する。 |
| COMP-002 | Must | 個人戦とチーム戦、固定 roster と stage ごとの lineup を扱える。 |
| COMP-003 | Must | 段位戦の参加資格、マッチング、順位結果からのポイント更新、昇段・降段を版管理できる。 |
| COMP-004 | Must | 総当たり、指定 schedule、スイス式相当、予選足切り、ノックアウト、グループから決勝を構成できる。 |
| COMP-005 | Must | 麻雀の一卓が 3 または 4 者であることを前提に、対戦履歴、同卓回数、座順を考慮した assignment policy を差し替えられる。 |
| COMP-006 | Must | 順位点、素点、チーム合計、持越し、ペナルティ、同点処理、勝ち上がり条件を集計方針として構成できる。 |
| COMP-007 | Must | schedule の乱数、assignment、lineup、裁定、進出決定を event log に残す。 |
| COMP-008 | Should | 昇降級、複数 division、season、playoff、入替規定を表現できる。 |
| COMP-009 | Could | population-based training の opponent pool 更新を competition event として統合する。 |

## 9. 記録・再生・データ出力

| ID | 優先度 | 要求 |
|---|---|---|
| DATA-001 | Must | 各実験に設定 hash、コード版、依存版、乱数方式、モデル版を記録する。 |
| DATA-002 | Must | 牌山または牌山を一意に再構成できる情報と、すべての外部応答を記録する。 |
| DATA-003 | Must | event log から終端状態を再構築し、安定 hash を照合できる。 |
| DATA-004 | Must | 公開 event と trajectory schema に version を持たせる。 |
| DATA-005 | Must | 不完全な run、失敗、キャンセルを成功 run と区別して保存する。 |
| DATA-006 | Should | policy 学習用 trajectory、評価集計、デバッグ用完全イベントを別 view として生成する。 |
| DATA-007 | Should | 古い schema を読み込む migration を core から分離できる。 |

## 10. 非機能要求

| ID | 優先度 | 要求 |
|---|---|---|
| NFR-001 | Must | 同一の解決済み設定、牌山、応答列、互換コード版で同じ event log と終端 hash を得る。 |
| NFR-002 | Must | ルール意味論は非同期 runtime、CPU 数、GPU batch 分割に依存しない。 |
| NFR-003 | Must | public boundary は型付きエラーを返し、外部入力で process 全体を panic させない。 |
| NFR-004 | Must | throughput、latency、batch、memory、queue depth、失敗を計測可能にする。 |
| NFR-005 | Must | secret、個人情報、公式サービスの認証情報を必要としない。 |
| NFR-006 | Must | 公式文書の著作権を尊重し、プリセットには値と出典のみを保持する。 |
| NFR-007 | Should | core の correctness test は GPU や network なしで実行できる。 |
| NFR-008 | Should | 単一 process から開始し、意味論を変えず複数 process へ拡張できる。 |
| NFR-009 | Should | Rust stable を使い、MSRV は実装開始時に依存関係を確認して固定する。 |

## 11. 制約

- 実装言語は Rust。
- Gym API とその互換層は採用しない。
- 公開 `step()` を中心 API にしない。
- TDD は `test list -> one -> red -> green -> refactor` の順を守る。
- `hule` は未公開であるため、取得方法、API、ライセンス、対応ルールが確定するまで release gate とする。
- 公式ルールは変更され得るため、確認日だけでなく対象版または対象期間を持つ。

## 12. 未決事項

以下は実装開始前または該当 milestone 前に ADR で決める。

- 最初に固定する Rust toolchain と MSRV
- event/trajectory の永続化形式
- in-process queue の async runtime
- 安定 hash と canonical serialization
- action/observation schema の最初の形状
- `hule` の配布方法、capability、ライセンス
- throughput 基準値とベンチマーク環境
- 最初に verified とする基準プリセット
