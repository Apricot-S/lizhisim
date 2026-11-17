# LizhiSim

AI / reinforcement learning 用途を主目的とする、日本式リーチ麻雀 simulator の Rust プロジェクトです。

現在はゼロからの再設計レビュー中です。Rust の新規実装は開始していません。既存コードは参考資料として残していますが、その型・crate 構成・API・実装方針を新設計へ引き継ぐ前提はありません。

重視するのは、決定的で再現可能な simulation、高スループット、並列実行、非同期 agent interaction、複数 ruleset、再生可能性、game mechanics と rule-dependent behavior の明確な分離、テスト容易性です。

[再設計レビュー資料](docs/redesign/README.md)に、10項目の設計案、履歴調査、未解決の判断をまとめています。旧設計書・ADR・test list などは撤去し、必要な資料だけを作り直しました。過去の資料は Git history から参照できます。

CI、repository metadata、toolchain、ライセンスは維持しています。作業上の指針は [AGENTS.md](AGENTS.md) を参照してください。

## License

Copyright (c) Apricot S. All rights reserved.

Licensed under the [MIT license](LICENSE).
