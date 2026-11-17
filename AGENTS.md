# 作業指針

## 現在の作業

LizhiSim はゼロからの再設計レビュー中である。[レビュー資料](docs/redesign/README.md)を入口とする。

- Rust の実装は、ユーザーが開始を指示するまで行わない。コンパイル可能な試作や依存追加も行わない。
- 既存コードと Git history は調査資料である。既存の型、crate、API、実装方針の継承を前提にしない。
- 旧資料は作業ツリーから撤去済みである。履歴上の Accepted ADR や旧要求を、新設計で自動的に採用済みと扱わない。
- 文書は必要になったものだけ作る。旧資料に対応する全文書、ADR、test list を先回りして再作成しない。
- confirmed requirement、candidate design、unresolved question を区別する。確定要件の根拠は最新の依頼に置き、履歴の事実と新しい判断を混同しない。
- 判断を要する点は選択肢・影響・必要な証拠とともに残す。未確認の公式ルール値を推測して埋めない。
- 日本語の概念説明を基本とし、具体的な Rust trait、struct、公開 schema、識別子をまだ確定しない。

## 変更と検証

- CI、repository metadata、toolchain、ライセンスなどの既存 infrastructure を可能な限り維持する。
- 無関係なユーザー変更を削除・整形・stage しない。コミットする場合は Conventional Commits の type を付ける。
- 文書変更では相対リンク、用語、要求・質問 ID の対応、確定と未決の整合性、`git diff --check` を確認する。
- Markdown lint と offline link check は[既存 workflow](.github/workflows/documentation.yml)を基準にする。実行できなければ代替検証と未実行項目を明記する。
- 文書だけの変更では Rust の build・test は不要。未実行の検証を成功と報告しない。
- 最終報告には変更内容、検証結果、残る制約を簡潔に記す。
