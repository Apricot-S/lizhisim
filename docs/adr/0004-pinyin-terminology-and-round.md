# ADR-0004: 麻雀用語はピンインを基本とし、Roundを局専用にする

- Status: Accepted
- Date: 2026-08-09
- Deciders: Project owner, LizhiSim design

## Context

日本語ローマ字には `furo` / `fuuro` のような表記揺れがある。英語も、手牌の門前部分に `closed hand` / `concealed hand` など複数の訳があり、同じ `hand` が局を指す場合もある。schema、protocol、Rust API で複数表記が混在すると、型名の境界と検索性が損なわれる。

また従来文書では `Hand` を局、`Round` を競技日程上の節に使っていたが、一般的な麻雀用法では局を `Round` と呼ぶ。

## Decision

- 非英語の麻雀用語の英字識別子は、原則として声調記号なしのピンインを使う。
- 手牌の門前部分は `bingpai`、副露は `fulu` とする。
- 既存方針に従い `xiangting`、`hule`、`lizhi` を使う。
- `Round` は局だけに使う。競技日程上の節は `Matchday` とする。
- 麻雀固有の対応語がない技術概念は英語とする。
- 中国語の語を造らない。新しい用語は glossary のユーザー決定待ち表へ追加し、ユーザーが識別子を決めるまで production 名を付けない。
- 日本語の説明文、外部 API・固有名の引用、一般アルゴリズムの `round-robin` はこの制約の対象外とする。

## Consequences

Positive:

- 日本語ローマ字と英訳の表記揺れを避けられる。
- `Round`、`bingpai`、`fulu` の境界が明確になる。
- `xiangting` / `hule` crate やプロジェクト名と用語体系が揃う。

Negative:

- 英語圏の開発者には用語集の参照が必要になる。
- 新しい用語を即興で命名できず、ユーザー決定を待つ工程が増える。
- 外部 library の英語 API との adapter で名称変換が必要になる。

## Alternatives considered

### 日本語ローマ字に統一

Rejected. 長音や促音、慣用表記に複数形が残る。

### 英語へ統一

Rejected. 麻雀用語の英訳が一意でなく、`hand` のような重大な多義性がある。

### プロジェクト内で中国語相当語を新しく作る

Rejected. 実在しない語を正規用語にしてしまう。対応語がなければ英語を使う。

## Follow-up / verification

- 禁止表記を `rg` で検査する。
- 新しい API/schema review では glossary 行の存在を確認する。
- ユーザー決定待ち表の識別子が埋まるまで、該当 production 型を実装しない。
