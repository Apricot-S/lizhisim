# AGENTS.md

このファイルはリポジトリ全体に適用する作業規約である。人間と AI エージェントは、変更前にこのファイルと関連文書を読むこと。

## 1. 現在のフェーズ

- 現在は **Phase 0: 設計** であり、Rust workspace は空の crate scaffolding だけで、ドメインの振る舞いは実装されていない。
- ユーザーから明示的に実装開始の指示があるまで、振る舞いを持つ Rust コード、新しい crate/dependency、生成コード、実行可能なプロトタイプを追加しない。既存 scaffolding は保持する。
- 設計中の例は擬似コードに留め、コンパイル可能な実装を文書へ埋め込まない。
- 既存の設計を変更するときは、影響する文書と ADR を同じ変更で更新する。

## 2. 読む順序

1. `README.md`
2. `docs/vision.md`
3. `docs/requirements.md`
4. 対象領域の `docs/design/*.md`
5. `docs/development-guide.md`
6. `docs/adr/*.md`
7. ルールを扱う場合は `docs/references/rule-sources.md`

矛盾がある場合は、ユーザーの最新指示、`AGENTS.md`、Accepted ADR、設計文書、README の順に優先する。矛盾を黙って解釈せず、文書を修正するか質問する。

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

## 7. TDD の必須手順

実装変更は必ず t-wada のサイクルで行う。

1. `docs/test-lists/` に test list を作るか更新する。
2. 未完了項目から 1 つだけ選び、現在の対象として印を付ける。
3. 失敗する最小のテストを書き、意図した理由で失敗することを確認する（red）。
4. そのテストだけを通す最小の実装を行う（green）。
5. 全テストが green の状態で重複、名前、責務、型を改善する（refactor）。
6. test list を更新し、次の 1 項目を選ぶ。

複数の振る舞いを一度に red にしない。バグ修正では、再現テストの red を先に確認する。テストを実装へ合わせて弱めない。

- 一つのtestは一つの観点だけを検証する。`正常系と異常系`、`個数と変換`のような独立して失敗し得る観点を同じtestへ混在させない。
- One assertion per testを原則とし、一つのtest functionではassertion macroを一回だけ使う。複数入力を同じ性質として検証する場合は結果を一つの値へ集約し、一回のassertionで比較する。
- 複数assertionが不可分だと判断した例外は、分割できない理由をtest listのcycle logへ記録してreviewする。fixture/setup確認をassertion数の例外にしない。

## 8. 検証方針

- pure transition の例示テストを最優先する。
- ルール境界値は表形式テスト、広い入力空間は property test、状態機械は model-based test を使う。
- `xiangting` と `hule` は契約テストと既知牌姿 corpus でアダプターを検証する。
- 同時ロン、鳴き競合、キャンセル、遅延応答、再送は決定的なスケジューラテストを持つ。
- replay にはイベント列だけでなく終端状態の安定 hash を検証する。
- プリセットには出典上の差分を示す golden test を用意する。
- 標準検証は `cargo fmt -- --check`、`cargo clippy -- -D warnings`、`cargo build --verbose`、`cargo test --verbose` とする。通常のCI jobはrepositoryの`rust-toolchain.toml`を使い、加えてnightlyの`cargo docs-rs`を実行する。
- dependency auditは`deny.toml`を規範として、独立したGitHub Actions workflowで`cargo deny check`を実行する。
- cargo-deny Actionの`rust-version`は`rust-toolchain.toml`の`channel`と一致させ、toolchain更新時に同じ変更で更新する。
- Markdownは`markdownlint-cli2`、repository内の相対リンクは`lychee --offline`を使う独立したGitHub Actions workflowで検査する。外部URLの疎通はこのgateへ含めない。

## 9. 文書と変更管理

- 文書は日本語を基本とし、コード識別子、プロトコルフィールド、固有名詞は必要に応じ英語を使う。
- RFC 2119 風の「必須」「禁止」「推奨」は規範的意味で使う。
- 相対リンクを保ち、文書の移動時は参照元を同時に更新する。
- 重要な設計判断は `docs/adr/` に追加する。Accepted ADR の意味を変える場合は、本文を上書きせず後継 ADR で supersede する。
- コミットを作成する場合は Conventional Commits の type を付ける。
- 無関係なユーザー変更を削除、整形、stage しない。

## 10. 完了条件

文書変更は、リンク、用語、要求 ID、ADR 参照、対象プリセット一覧に矛盾がなく、`git diff --check` が成功した時点で完了とする。実装変更の完了条件は、test list の対象項目が green、関連する全テスト・静的検査が成功、再現性と出典が必要に応じ更新済みであることとする。
