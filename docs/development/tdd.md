# TDDとバグ修正

実装・バグ修正時の必須手順。対象test listのCurrentから始める。 [開発手順の入口](../development-guide.md)。節番号は移設前の参照との対応を保つ。

## 3. t-wada TDD の基本サイクル

すべての production behavior は次の順で作る。

```text
test list
  -> one test selected
  -> red
  -> green
  -> refactor
  -> update test list
  -> next one
```

### 3.1 Test list を作る

作業単位ごとに `docs/test-lists/<topic>.md` を作り、[template](../templates/test-list.md) を使う。要求 ID、根拠、例、境界、エラー、property 候補を思いつく限り列挙する。最初から実装順に固定する必要はなく、途中で気づいた項目を追加する。

test list はテストコードの一覧ではなく、振る舞いの仮説と不安の一覧である。項目は観測可能な結果で書く。

良い例:

- 「頭ハネ設定では、下家のロン応答が先着しても上家のロンが採用される」
- 「三麻で除外した二萬を赤牌に指定した設定は validation error になる」

悪い例:

- 「`resolve()` をテストする」
- 「coverage を上げる」

### 3.2 1 つだけ選ぶ

最小で、設計上の学びが大きく、短く green にできる項目を一つ `Current` にする。関連する複数例を一度に選ばない。新しい疑問が出たら test list へ追加し、現在の red を広げない。

選択した項目に独立して失敗し得る複数の観点が含まれていたら、testを書く前に項目を分割する。正常系と異常系、個数と変換、受理と拒否を一つの項目またはtestへまとめない。

### 3.3 Red

失敗する最小テストを書く。

- compile failure を狙う type test か、runtime assertion failure かを明確にする。
- 意図した理由で失敗していることを読む。
- fixture/setup の失敗や typo を red と数えない。
- 既存テストが別理由で壊れている場合は先に切り分ける。
- red の command と要点を cycle log に記録する。

一つのtest functionは一つの観点だけを検証し、One assertion per testを原則とする。

- assertion macroは原則としてtest functionごとに一回だけ使う。
- 同じ性質に対する複数入力は、結果を配列や値へ集約して一回のassertionで比較できる。
- loop内で繰り返しassertionせず、入力ごとに別観点ならtestを分ける。
- 複数assertionが不可分な例外は、分割できない理由をcycle logへ記録しreviewする。
- fixtureやsetupの妥当性確認を追加assertionで混在させない。必要ならfixture自身のtestへ分ける。

型で禁止すべき不正状態は compile-fail test を検討する。ただし compile-fail snapshot の保守コストと価値を比べる。

### 3.4 Green

選んだ一項目だけを通す最小実装を行う。

- 将来の一般化を先回りしない。
- まだ test list にある別のケースを同時実装しない。
- ハードコードが現在例だけを偽装する場合は、次の代表例を test list からすぐ選ぶ。
- green 後に関連 suite と全 suite を実行する。

### 3.5 三角測量

一つの具体例だけでは実装の一般性を判断できず、現在例へのハードコードでもgreenになる場合は、三角測量を使う。最初の項目をgreenにした後、値や条件が異なる代表例をtest listの次項目として一つだけ選び、新しい`red -> green`サイクルで一般化を促す。同じredへ複数例を追加したり、将来必要になりそうな組合せを網羅したりしない。

三角測量に使う例は、現在の仮実装と意図する規則を区別できる最小のものにする。境界値、異常系、rule variation、異なる状態や処理分岐が仕様上重要なら、それらは一般化の足場ではなく独立した契約としてtest listへ残す。

実装を一般化した後のrefactorでは、三角測量で追加したtestも見直す。次をすべて満たす場合に限り削除できる。

- 追加目的が、現在例だけに通用する仮実装から一般化を導くことだった。
- 残るtestが同じ仕様を十分に検証し、仕様の意図も明確に表現している。
- 削除しても境界値、異常系、過去のbugの再発防止、重要な条件分岐の検出能力を失わない。
- 必要に応じて最小のmutantを使い、残るtestが一般化前の誤実装を検出できると確認した。

削除する場合は、三角測量のために必要だった理由、一般化後に不要となった理由、契約を引き継ぐtestをcycle logへ記録する。単に入力値が似ている、または同じproduction経路を通るという理由だけでは削除しない。

### 3.6 Refactor

全テスト green のまま改善する。

- 名前を domain 用語へ揃える。
- 重複を除く。
- primitive を value type へ寄せる。
- 不正状態を型へ押し出す。
- effect を shell 側へ移す。
- function/enum の責務と exhaustive match を見直す。

refactor で新しい振る舞いを追加しない。必要なら test list へ戻り、新しい red から始める。

test codeもrefactor対象であり、testを追加し続けるだけの履歴置き場にしない。Kent Beck『Test-Driven Development: By Example』第11章の「不要になったら消す」という考え方を、次の具体的な規則として適用する。

- 後から追加した強いtestが既存testの契約を完全に包含するなら、既存testを削除する。
- 全要素の固定期待値は同じcollectionの`len`だけの検証を、具体的な`Err`値は`is_err()`だけの検証を、複数回の固定列は同じ列の先頭だけの検証を通常包含する。
- 同じfixtureを正常に構築して後続の振る舞いを検証するtestがあれば、同じfixtureに対する`is_ok()`だけのsmoke testは通常不要である。
- testを削除しても、test listの完了済み項目は戻さない。どの強いtestが契約を引き継いだかをcycle logへ記録する。

ただし、見た目が似ていることだけを理由に削除しない。入力境界、状態、rule variation、処理分岐、error payloadが独立して壊れ得るなら別のtestとして維持する。包含関係が不明な場合は、候補testを一時的に削除するだけで判断せず、production codeへ最小のmutantを入れて残るtestが意図した不具合を検出するか確認する。

### 3.7 記録して繰り返す

対象項目を完了し、気づいた項目を追加する。未完了一覧がゼロになっても、要求・property・integration の観点で漏れを review してから topic を完了する。

## 4. Vertical slice の選び方

最初に牌型 library 全体、次に queue 全体という horizontal 実装を避ける。一つの小さな scenario を end-to-end に通す。

推奨する最初の slice 例:

1. 固定済み四人ルールと固定牌山を検証して一つの `Round` を開始する。
2. 一人へ捨牌要求を発行して typed continuation で中断する。
3. 合法な捨牌応答を返す。
4. 誰にも鳴き候補がなければ次のツモ状態へ進む。
5. event log から同じ終端 hash を再生する。

この一つを test list でさらに小さく分ける。GPU、network、実 `hule` がなくても fake backend/port で core の形を検証できる。

## 9. Bug 修正

1. 再現条件を test list へ追加する。
2. 最も小さい層で再現テストを書く。
3. 意図した失敗を確認する。
4. 最小修正で green にする。
5. 同種の入力を property/table test へ広げる。
6. replay/preset/source への影響を確認する。

イベントや公式ルールが誤っていた場合、過去データの無言修正はしない。affected version/run を記録し、新版または migration 方針を用意する。
