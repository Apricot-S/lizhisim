# LizhiSim 再設計レビュー

- 作成日: 2026-11-17
- 状態: **人間によるレビュー用の案。技術案の採用・Rust 実装開始を意味しない。**
- 調査基準: `b0c2a5fa15a592615d42e3c2e7cc91a078515daf`。作業開始時の working tree は clean。

## 資料の位置付け

今回は既存設計の改修ではなく、AI / reinforcement learning 用の日本式リーチ麻雀 simulator をゼロから設計する。既存実装は調査用に残すが、型・crate 構成・API・実装方針を継承する前提を置かない。

AGENTS.md を含む旧設計書、旧要求一覧、ADR、開発手順、test list、前回の再設計案は作業ツリーから撤去し、必要な資料だけを新しく作成した。過去資料の内容は [Git history の調査](history.md)に基準 commit と path を記し、履歴から参照する。旧 Accepted ADR を新設計の規範として扱わない。

ライセンス・第三者通知、CI、repository metadata、toolchain と既存 Rust ソースは保持する。現在の workspace が旧 crate 構成のままであることは、新設計の採用を意味しない。

## 区分

| 区分 | 意味 |
|---|---|
| **confirmed requirement** | 今回の依頼から直接確認できる目的・制約。下表の G-ID で参照する |
| **candidate design** | 目的を満たすための提案・比較対象。採用前であり、実装方針を拘束しない |
| **unresolved question** | 人間の判断、公式仕様の確認、実測等が必要な点。[Q-ID](questions.md)で追跡する |

履歴調査の「確認した事実」は当時のコード・文書についての事実であり、新しい confirmed requirement ではない。旧方針を維持する提案も candidate design として示す。旧資料に「ユーザー決定」とある事項は再確認対象として追跡し、暗黙に消したり、新しい合意へ読み替えたりしない。

## Confirmed requirement

| ID | 今回確認できる要件 |
|---|---|
| G-01 | AI / reinforcement learning 用途を主目的とする日本式リーチ麻雀 simulator |
| G-02 | deterministic / reproducible simulation を重視する |
| G-03 | high-throughput execution と parallel execution を重視する |
| G-04 | asynchronous agent interaction を重視する |
| G-05 | multiple rulesets を扱い、rule-dependent behavior を game mechanics から明確に分離する |
| G-06 | replayability と testability を重視する |
| G-07 | 既存型・crate・API・実装方針の継承を前提にしない。旧資料を削除・再作成し、必要な資料だけを作る |
| G-08 | Rust の実装は開始せず、設計上の判断・具体的な trait・struct を未承認で確定しない。後続の仕様化で合意した事項は[回答の記録](questions.md)に従う |
| G-09 | CI、repository metadata、ライセンスなど実装と独立した infrastructure を可能な限り維持する |
| G-10 | repository と Git history を調査し、指定の10項目を人間がレビューできる資料にする |

## レビューの入口

| 文書 | 内容 |
|---|---|
| [公開 API の契約・利用シナリオ](api-contract.md) | 第1〜136問の合意の横断整理、正常系・失敗時の契約、公開操作・戻り値の候補と最初の TDD 対象 |
| [設計案](design.md) | 1. goals / non-goals、2. domain model、3. lifecycle、4. transitions、5. actions、6. rule decisions、7. invariants、8. randomness、9. replay / events、10. crate boundaries |
| [履歴調査](history.md) | 検討済み設計、変更理由、実装で具体化した制約、維持を提案する要件と再検討事項、保全対象 |
| [未解決の判断](questions.md) | 選択肢、影響、判断に必要な根拠、レビューの順序 |
| [対象ルールの差分・初回調査](rule-differences.md) | 第4問で指定された比較対象、公式資料の取得状況、7領域への具体的な影響、未確認事項 |

第136問までの合意は公開 API のレビュー案から横断して確認できる。設計案の1節と回答の記録でスコープ・根拠を読み、対象ルールの差分調査、6節の7領域マトリクス、4節の遷移表を照合する。実際のルール差を踏まえ、API 契約に影響する判断を一つずつ検討する。文書の作成完了と、候補設計の採用・未解決事項の解消・実装許可は別である。

説明上の用語はコード識別子の決定ではない。第116問で Rust API の命名は中国語ピンイン中心とすることに合意したが、個別の名称は未確定であり、旧用語集の回収は必要になるまで保留する。本資料では hand termination は「局終了」、round continuation は「連荘・次局への進行」、game termination は「対局終了」を意味する。「手牌」と「局」、「一局」と「一対局」を区別する。

初回の再設計資料作成では外部公式ルールの再監査を行っていない。その後、第4問の回答を受けて公式公開資料の初回差分調査を行い、確認した条項と未取得・旧版参考を[別紙](rule-differences.md)に分けた。完全なルール監査・公式プリセットの認定ではない。ゲームへのログイン・大会作成・元牌譜取得・性能測定・Rust test の再実行は行っておらず、公式プリセット・新依存も追加していない。
