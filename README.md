# LizhiSim

深層学習向けの AI 対戦用リーチ麻雀シミュレーターを、Rust で新規設計するプロジェクトです。

現在は **Phase 1: 決定的な一局walking skeleton** です。t-wada TDDに従い、test listから一項目ずつ`red -> green -> refactor`で実装します。公開APIはwalking skeletonに必要な最小範囲だけです。

## 目標

- 四人麻雀と三人麻雀を同じ設計原則で扱う。
- 雀魂、天鳳、麻雀一番街、龍龍の段位戦と、主要な競技団体・リーグのルールを、出典付きの版管理されたプリセットとして提供する。
- 1 卓の半荘だけでなく、段位戦、リーグ戦、チーム戦、予選・決勝を含むトーナメントをシミュレートする。
- 多数の卓から生じる意思決定要求をキューに集約し、GPU バッチ推論へ効率よく渡す。
- `step()` ではなく、型付きの状態と継続で合法な状態遷移だけを表現する。
- 同一の設定、牌山、応答列から同一結果を再生できるようにする。

英字の麻雀用語は原則としてピンインを採用し、`Round` は局だけに使います。最初に実装・検証するルールは雀魂段位戦（四人/三人）、次に天鳳、麻雀一番街です。

## 非目標

- Gym API およびその互換層
- GUI やオンライン対戦サービス
- 実卓の所作、発声の聞き取り、審判裁定そのものの完全再現
- 公式サービスとの通信や互換プロトコル

## 文書

1. [ビジョンとスコープ](docs/vision.md)
2. [要求仕様](docs/requirements.md)
3. [アーキテクチャ](docs/design/architecture.md)
4. [ドメインモデル](docs/design/domain-model.md)
5. [ルールとプリセット](docs/design/rules-and-presets.md)
6. [大会・段位戦・リーグ戦](docs/design/competitions.md)
7. [推論キューと継続プロトコル](docs/design/inference-protocol.md)
8. [開発手順書](docs/development-guide.md)
9. [ロードマップ](docs/roadmap.md)
10. [公式ルール出典台帳](docs/references/rule-sources.md)
11. [用語集](docs/glossary.md)
12. [設計判断記録](docs/adr/README.md)
13. [Phase 0 review](docs/phase-0-review.md)

開発者・AI エージェント向けの作業規約は [AGENTS.md](AGENTS.md) を参照してください。

## 設計上の中心原則

```text
純粋なドメイン遷移
    -> 意思決定要求を発行して中断
    -> 複数卓の要求を推論キューへ集約
    -> 互換な要求を GPU バッチ推論
    -> 応答を検証して型付き継続を再開
    -> ドメインイベントを追記
```

ルールは「卓内」「半荘」「大会」「段位・レーティング」に分割します。たとえば M リーグの卓内ルールを別のリーグ形式で使うことや、同じ卓内ルールに異なる段位ポイント制度を組み合わせることが可能な構造を目指します。

## 開発方法

t-wada の TDD を採用します。実装を許可された後も、必ず `test list -> 1つ選択 -> red -> green -> refactor` の順で、小さな縦切りを進めます。詳細は [開発手順書](docs/development-guide.md) に記載しています。

## License

Copyright (c) Apricot S. All rights reserved.

Licensed under the [MIT license](LICENSE).
