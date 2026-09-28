# ADR-0020: ゼロからの再設計案と既存設計の適用範囲を分ける

- Status: Proposed
- Date: 2026-09-28
- Deciders: Project owner（技術案の採用判断待ち）
- Relates to: [ADR一覧](README.md)、[再設計レビュー案](../redesign/README.md)
- Supersedes: 未確定。採用時に対象と範囲を記録する

## Context

ユーザーは、既存の型・crate・API・実装方針の継承を前提とせず、repositoryとGit historyを調査して再設計のレビュー資料を作るよう指示した。今回はRust実装を開始しない。既存Accepted ADRの具体的構造を新設計の必須条件にすると、この依頼と衝突する。一方で、過去の判断理由、製品要件、独立したinfrastructureを失う必要はない。

## Decision proposal

- [再設計資料](../redesign/README.md)をreview用の独立領域とし、confirmed requirement、candidate design、unresolved questionを明示する。
- 今回の最新指示に従い、既存実装・具体設計を参考資料として扱う。旧文書は現コードと履歴を読むために保持する。
- イベント駆動、決定性、Rust、評価adapter等の変更を指示されていない製品方針は維持する。具体型、継続表現、crate境界、内部正規化は新設計への採否を別途判断する。
- 新設計の採用時に、このADRまたは個別の後継ADRで置換範囲を明示する。旧Accepted本文の意味を上書きしない。
- CI、metadata、toolchain、license、既存コードとtest list実績を保存し、今は新実装を作らない。

再設計資料の作成自体はユーザーの依頼により実行済みの作業である。本ADRのProposedは、技術案の採用や旧ADR全体の廃止が未承認であることを表す。

### 2026-09-28の追加要件

ユーザーは、特定局面の期待値評価とAI学習用の局面生成のため、外部から局面を指定して続行できることを追加した。残り牌山を事前に設定するか・どう生成するかの判断は局面生成側に置く。この要件は[CORE-009〜010・DATA-010](../requirements.md)へ記録した。

[外部指定局面の設計案](../redesign/positions.md)で、通常の配牌以外の開始経路、入力検証、履歴に依存する資格、独立試行、開始局面以後のreplay、生成側との責務境界を扱う。入力schema、受入phase、到達性保証、牌資源の受渡し時点はQ-13で未決とし、既存の非公開constructorやsnapshot形式をそのまま公開する決定はしない。

## Consequences

- Positive: 過去の制約を検討材料にしつつ、実装継承を強制せずレビューできる。
- Negative: 旧設計と新案が併存するため、入口と各設計文書で適用範囲を明示する必要がある。

## Alternatives considered

- 旧実装・文書を即時削除する: 調査根拠とinfrastructureを失い、今回の文書案作成に不要。
- 既存型のrename/refactorとして進める: ゼロから再検討する指示を満たさない。
- 旧ADRを一括Supersededにする: 未解決の製品・技術判断まで新しい合意があるように見える。

## Follow-up / verification

- [判断待ち一覧](../redesign/questions.md)の優先項目を人間がレビューする。
- 採用時には要求ID、対象preset、用語、旧ADRとの置換関係を確認する。
- 今回は文書検証のみを行い、実装・test list選択・Rust検証を行わない。
