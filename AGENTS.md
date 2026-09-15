# AGENTS.md

リポジトリ全体の作業規約。ここには常時必要な制約を置き、詳細手順は対象文書を参照する。

## 1. 作業範囲と進め方

現在は **Phase 1: 決定的な一局walking skeleton** である。

- 依頼された範囲を、必要な修正・検証・文書更新まで完了する。通常の調査、ローカル編集、検証、今回の変更に起因する失敗の修正は、各段階で再承認を求めず進める。
- 実装は選択中のtest list項目だけを対象に、一項目ずつ `red -> green -> refactor` で進める。未選択の振る舞い、新しいcrate/dependency、生成コード、実行可能なプロトタイプを先回りして追加しない。
- 設計中の例は擬似コードに留め、production実装の代わりとなるコンパイル可能な例を文書へ埋め込まない。
- 既存設計を変更するときは、影響する文書とADRを同じ変更で更新する。
- 判断が必要なのは、製品方針の変更、未決の麻雀用語、未確認の公式値など、既存の合意や資料で解決できない事項である。該当規約と必要な判断を具体的に示し、独立して進められる作業は続ける。
- 最終報告には変更内容、検証結果、残る制約を簡潔に記す。未実行の検証を成功と扱わない。

## 2. 必要な文書を読む

変更対象に応じて次の文書を読む。既に確認済みで変更のない文書の再読や、無関係な文書の一括読込は不要である。

| 作業 | 参照先 |
|---|---|
| プロジェクトの目的・スコープの把握 | [README](README.md)、[ビジョン](docs/vision.md)、[要求仕様](docs/requirements.md) |
| 実装・テストの変更 | [開発手順書](docs/development-guide.md)、対象の[test list](docs/test-lists/README.md)、対象領域の設計文書 |
| 責務・依存方向・状態遷移 | [アーキテクチャ](docs/design/architecture.md)、[ドメインモデル](docs/design/domain-model.md) |
| ルール・プリセット・牌譜検証 | [ルール設計](docs/design/rules-and-presets.md)、[出典台帳](docs/references/rule-sources.md)、[プリセット開発手順](docs/development/adapters-and-presets.md) |
| 推論要求・応答・スケジューリング | [推論プロトコル](docs/design/inference-protocol.md) |
| 半荘・大会・段位 | [大会設計](docs/design/competitions.md) |
| 識別子の追加・変更 | [用語集](docs/glossary.md) |
| 設計・CI・toolchainの変更 | [ADR一覧](docs/adr/README.md)から対象のAccepted ADR、関連設計文書 |

対象文書が参照するADRも確認する。文書の誤字・リンク修正は、対象箇所と参照元の確認でよい。

プロジェクト内の規範の優先順位は、ユーザーの最新指示、`AGENTS.md`、Accepted ADR、設計文書、READMEとする。矛盾は文書を修正して解消するか、判断が必要な点をユーザーへ示す。

## 3. 不変の製品方針

- 言語は Rust とする。
- Gym API、Gym 互換層、単一環境を同期的に進める公開 `step()` API は作らない。
- 多数卓の意思決定要求をキューに積むイベント駆動方式とする。
- 状態遷移は、状態を消費して新しい状態または型付きの中断点を返す継続渡しで表現する。
- 不正な状態を可能な限り型で表現不能にし、外部入力は必ず検証済み型へ変換する。
- functional core / imperative shell とし、ドメイン計算は決定的かつ副作用なしに保つ。
- シャンテン数計算は `xiangting` crate、点数計算は未公開の `hule` crate を境界アダプター越しに利用する。ドメイン層から crate 固有型を参照しない。
- ルール、牌山、AI 応答が同じなら再生結果が一致するよう、乱数とイベントを記録する。

これらを変更するには、ユーザーの明示的な合意と ADR の更新が必要である。

## 4. 用語と識別子

- 日本語文書の説明文は日本語でよいが、非英語の麻雀用語をコード識別子、schema field、protocol 名、英字表記にする場合は、原則として中国語のピンインを使う。
- 日本語のローマ字表記や英訳語を、対応する確立済みピンインがある麻雀概念へ採用しない。たとえば `bingpai`、`fulu` を使い、`ConcealedHand`、`Furo`、`Fuuro` は使わない。
- 麻雀固有の対応語が存在しない技術概念は英語とする。`Seat`、`Actor`、`Request` などが該当する。
- 中国語の語やピンインを推測・造語しない。新しい麻雀用語の識別子が必要になったら、先に `docs/glossary.md` の「ユーザー決定待ち」表へ概念と空欄の識別子を追加し、ユーザーへ記入を依頼する。決定前に production 識別子を作らない。
- `Round` は麻雀の「局」だけに使う。競技日程上の「節」は `Matchday` とし、`Round` を流用しない。ただし `round-robin` や scheduler の `weighted round-robin` など、一般アルゴリズムの固有表現は除く。
- 確定済み表記、禁止表記、大小文字規則は `docs/glossary.md` を規範とする。

## 5. ルールとプリセット

- 卓内ルール、半荘進行、大会形式、段位・レーティングを別の設定型にする。巨大な万能設定型にまとめない。
- プリセットは上書き差分の連鎖ではなく、検証後の完全なスナップショットとして識別・ハッシュ化できるようにする。
- 再現可能な実験では `current` のような可動 alias を保存せず、解決済みの不変 ID を保存する。
- 公式ルールの値を推測しない。出典 URL、文書版、確認日、対象シーズンまたはアプリ版を記録する。
- Web に完全な公式仕様がない場合は、ゲーム内表示の証跡が得られるまで `draft` または `blocked` とする。
- 公式文書の物理的な作法や審判規定を非対応とする場合も、無視せず「シミュレーション対象外の条項」として追跡する。
- ルール本文を大量転載しない。事実を構造化して要約し、出典へリンクする。
- 実装・検証の優先順位は、雀魂段位戦（四人/三人）、天鳳段位戦（四人/三人）、麻雀一番街段位戦（四人/三人）の順とする。
- 雀魂は公式詳細ルールを一次資料とし、記載のない corner case は Cryolite/kanachan の `src/simulation` と検証記録を手掛かりにする。最終的な test evidence は、記載された牌譜 ID から元牌譜を取得して作ることを推奨する。
- `majsoul-record`の取得はuserがproject外で行い、取得toolを作らない。conformance app crateはlocal `majsoul-record`を逐次decode・projectし、中間`game_log`を必須にせずLizhiSimと1パスで比較する。牌譜IDとreplay URLはsource locatorとして利用できる。
- Gitには手書きの最小corner-case fixtureだけを置いてCIで検証する。完全な`majsoul-record`と診断用に出力した完全な`game_log`はGitへ置かず、user管理corpusによるfull-record検証をCI外で定期実行する。

## 6. 実装開始後の Rust 規約

- `unsafe` は原則禁止。必要性、健全性条件、代替案、テストを ADR に記録し、明示的レビューを受ける。
- ドメイン層での `unwrap`、`expect`、`panic!`、到達不能を仮定したワイルドカード分岐を禁止する。
- ID、点数、局番号、座席、牌、ルール版を primitive のまま混用せず newtype または enum にする。
- 状態別のデータは typestate で分ける。任意フィールドの集合で状態差を表現しない。
- 遷移は古い状態を消費し、`(new_state, events)`、完了、または `Suspension<Response, Continuation>` を返す。
- 非同期 runtime、時刻、乱数、永続化、推論クライアントは shell に閉じ込める。
- 外部応答を合法手集合と照合し、重複、期限切れ、未知 ID、不正 action を区別したエラーにする。
- 公開スキーマとイベントには明示的な schema version を持たせる。

## 7. TDD

実装変更には[TDD手順](docs/development/tdd.md#3-t-wada-tdd-の基本サイクル)を必須手順として適用する。

- test listの一項目を `Current` にし、意図したred、最小green、全テストgreenでのrefactor、cycle log更新の順で進める。バグ修正も再現テストのredから始め、テストを実装へ合わせて弱めない。
- 一つのtestは一観点・原則一assertionとする。不可分な例外はcycle logへ理由を記録してreviewする。fixture/setup確認を例外にしない。
- 一例へのハードコードでもgreenになる場合は、次の一項目で三角測量する。同じredに複数例を混ぜない。
- refactorではtest suiteも整理する。契約を包含する強いtestがある場合だけ不要なtestを削除し、独立した境界・error・分岐・過去のbugの検証を維持する。三角測量testの削除理由と契約の引継先はcycle logへ残す。

## 8. 検証

検証対象ごとの方針は[テスト戦略](docs/development/testing.md)、コマンドとCI運用は[検証と完了条件](docs/development/verification.md)を規範とする。

- 実装変更の標準検証は `cargo fmt -- --check`、`cargo clippy -- -D warnings`、`cargo build --verbose`、`cargo test --verbose` とする。
- 文書変更は用語・要求ID・ADR参照・対象プリセット一覧・相対リンクの整合性と `git diff --check` を確認する。Markdown lintとoffline link checkはdocumentation workflowに従う。
- 必須検証が成功した後、変更や失敗などの新たな根拠がなければ同じ検証を繰り返さない。文書だけの変更でRustの検証は不要である。

## 9. 文書と変更管理

- 文書は日本語を基本とし、コード識別子、プロトコルフィールド、固有名詞は必要に応じ英語を使う。
- RFC 2119 風の「必須」「禁止」「推奨」は規範的意味で使う。
- 相対リンクを保ち、文書の移動時は参照元を同時に更新する。
- 重要な設計判断は `docs/adr/` に追加する。Accepted ADR の意味を変える場合は、本文を上書きせず後継 ADR で supersede する。
- コミットを作成する場合は Conventional Commits の type を付ける。
- 無関係なユーザー変更を削除、整形、stage しない。

## 10. 完了条件

文書変更は、リンク、用語、要求 ID、ADR 参照、対象プリセット一覧に矛盾がなく、`git diff --check` が成功した時点で完了とする。実装変更の完了条件は、test list の対象項目が green、関連する全テスト・静的検査が成功、再現性と出典が必要に応じ更新済みであることとする。
