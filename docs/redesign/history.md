# Repository と Git history の調査

調査日: 2026-11-17。基準: `b0c2a5fa15a592615d42e3c2e7cc91a078515daf`。新設計への採否は[設計案](design.md)と[未解決の判断](questions.md)を参照する。

## 調査方法と限界

作業開始時の working tree は clean。`git log --all --reverse --oneline` で履歴の流れを確認し、旧実装末期、撤去・再出発、設計変更を表す ADR、関連差分、基準 HEAD の実装・test を重点的に読んだ。全 commit の全差分を監査したわけではない。

基準 HEAD の旧資料を調査してから撤去した。以下の path は明記した commit 内での歴史的な位置であり、現在の相対リンクではない。再確認の例:

```text
git show b0c2a5f:docs/adr/0001-event-driven-typed-continuations.md
git show b0c2a5f:docs/adr/0016-initial-deal-shouqie-action.md
git show b0c2a5f:docs/redesign/positions.md
git show 09c06fa -- crates/lizhisim-core/src/round.rs
git show 836fa35:lizhisim/src/bipai/core.rs
git show 836fa35:lizhisim/src/bipai/provider/random.rs
```

「コードで確認した事実」「旧資料が述べる理由」「新設計への提案」を分ける。旧資料に Accepted / user 指定とあっても、新設計で自動採用しない。公式ルールの現行値、実測性能、今回の test 成功はこの調査で確認していない。

## 現在の実装から分かること

| ID | 対象・確認した事実 | 証明していないこと |
|---|---|---|
| H-01 | [workspace](../../Cargo.toml) は facade/core/rules の3 crate。[rules](../../crates/lizhisim-rules/src/rule_set.rs) は赤牌枚数の各0〜4を検証し、親の開始方式を閉じた選択肢として保持する | ruleset 全体の意味・能力検証、完全な preset、最適な crate 境界 |
| H-02 | [局](../../crates/lizhisim-core/src/round.rs) は四人の配牌、取得、手切・摸切、反応なしの巡回、通常山枯渇時の荒牌平局を持つ。生成境界は crate-private。`no_reaction` は反応なしを仮定する | 全合法手の列挙、和了・副露・立直・応答窓・非同期 API・replay の完成 |
| H-02a | [対局台帳](../../crates/lizhisim-core/src/table_match.rs) は点数・場・局位置・本場・供託・親と、終了局を受け取る境界を持つ | 実際の点数移動、連荘、延長、対局終了、最終精算。`RoundSettlement` は入力を保持する段階 |
| H-02b | [牌山](../../crates/lizhisim-core/src/bipai.rs) は四人の固定配列と multiset 検証、配牌、通常取得、補充、表示牌参照を持つ。補充・表示の一部は局内 rule 判断への統合待ち | 三人での実行、槓の合法性・成立順序、表示の公式タイミング。低水準操作の test と一局全体の正しさは別 |

`ce39bd1` の subject は deterministic round walking skeleton の実装を示すが、実際の変更ファイルは文書である。`git show --stat ce39bd1` で確認でき、非同期実行や replay の実装根拠にはしない。

## これまでの設計と変更理由

以下の ADR path はすべて基準 `b0c2a5f` の `docs/adr/` 配下を指す。理由は該当資料に記載された理由であり、性能試験から実証された理由とは限らない。

| ID | 根拠 | 検討・変更された設計 | 理由と再設計での扱い |
|---|---|---|---|
| H-03 | `0001-event-driven-typed-continuations.md`、`0002-versioned-rule-layers.md`、`0003-separate-competition-domain.md` | 同期 step 中心案からイベント駆動・型付き継続へ。卓内・対局・大会・段位の意味を分離し、不変版のルールを使用 | 多数卓の非同期性、batch 推論、応答競合、再生、異なる改定周期のため。目的は有用だが closed enum の継続、Gym 禁止、4層の具体型まで引き継ぐ必然性は未確認 |
| H-04 | `0007-content-addressed-source-evidence.md` → `0008-source-review-without-copying.md`、`9996e91` | 公式原資料の content-addressed 保存から、URL・版・確認記録中心へ変更 | 原資料の複製と運用負担を避ける意図。再現入力の保存と公式文書本文の保存を混同しない |
| H-04a | `0009-minimized-game-record-evidence.md` → `0010-mahjong-soul-record-conformance.md` → `0011-one-pass-majsoul-conformance.md`、`0410f87` | 最小 fixture 中心、変換後の牌譜による検証を経て、local 生牌譜を decode/project して1パス照合する案へ | 中間出力の stale 化、converter bug の oracle 化、二重保存を避ける意図。最小の手書き scenario と CI 外 full-record corpus を分ける旧方針は Q-11 へ |
| H-05 | `0012-normalize-dealer-first-draw.md` → `0016-initial-deal-shouqie-action.md`、`ae32e9e`、`bffc635` | 親の14枚配牌を13枚＋分離1枚に正規化。最初の摸切 action を受理して結果だけ補正する案から、手切だけを受理する案へ変更 | action と公開結果の意味がずれ、存在しない選択肢を提示する問題。合法手・観測・再生の意味を内部正規化で変えない要件を抽出。13＋1という表現は再検討 |
| H-05a | `0019-first-zimo-origin-in-rules.md`、`e755189` | 親の開始方式を core 固有入力から rules の検証済み設定経由へ移動 | 旧資料には owner による所有先・enum 共有・名称の指定と記録。開始方式が合法手へ影響することは重要だが、所有 crate や型名は再選定 |
| H-06 | `0006-rust-toolchain-workspace-and-ci.md` → `0014-facade-and-core-crates.md`、`86e1db5` | 初期単一 crate から re-export facade と core を分離 | owner 指定と公開入口を実装から分ける意図。測定から最適性を決めた記録ではなく、Q-13 で比較する |
| H-06a | `0015-rule-and-domain-tile-ownership.md` → `0017-core-depends-on-rules.md`、`b4fa23e` | rules → core の依存を core → rules へ反転し、牌構成の解決を core へ移動 | 旧資料には rules の独立と牌型の core 所有を owner が指定したとある。実行用ルール、入力 schema、共有概念、出典の所有を分けて再検討 |
| H-06b | `0018-rule-set-validation-names.md`、`bed389b` | `RawRuleSpec` / `RuleSpec` から `RuleSet` / `ValidatedRuleSet` へ変更 | 無印の入力と Validated の対比を owner が採用。名称から検証能力の完成を推論しない |
| H-07 | `836fa35` の `lizhisim/src/bipai/core.rs`、`provider/random.rs` | RNG による構築、mutable な取得・槓処理、固定/乱数 provider。RNG clone で同じ牌山になる test | コードと test から確認。worker 数や用途別 stream の独立まで保証した記録ではない |
| H-07a | `fcb7eb6`、`4746362`、`0a8cd0f` | hand-tools を撤去し、旧牌山等を撤去、プロジェクトを再出発 | 削除対象・manifest の変更は確認。撤去の詳細な動機は commit 本文から確認できず、性能や正しさが原因だったとは推測しない |
| H-08 | `0013-tile-kind-without-copy-identity.md`、`8fac102`、牌・手牌 test | 赤を分けた37種と multiset を採用し、同種牌の個体 ID を core へ持たない | source 固有の情報を domain/観測へ混ぜず、赤を行為で区別する意図。37要素配列・index は再検討し、赤の意味の保存を重視 |
| H-09 | `d40c816`、`33939f9`、[player](../../crates/lizhisim-core/src/player.rs) | 第一打前の flag を第一巡資格として扱い、外部の行為で解除できる形へ変更 | 河が空であることと第一巡資格を同一視できない制約。現 flag が全資格を表現できる証明ではない |
| H-10 | `e27702c`、`100caba`、[終了局](../../crates/lizhisim-core/src/round.rs) | 終局理由だけの返却から、終了状態に局データを保持する形へ変更 | 終局時の牌・座席等を精算や診断へ渡せる。全局状態を保持するか十分な結果へ射影するかは再検討 |
| H-10a | `807c109`、`e66f83f`、`f91fed8` | 局結果を対局台帳とともに精算境界へ渡す。局内の終局判断と次局条件を分離 | 不採用の流局候補を確定結果にせず、得点・本場・連荘を別に決める意図。局中の供託まで含む原子性は未解決 |
| H-11 | `09c06fa`、`99cb437`、`7b66f3c`、`ba2eb6e`、`b444f58` | 親からの配牌先を修正、初期ドラ公開を配牌から分離、補充取得で通常残量も減らす | 以下の制約表へ抽出。配列 rotate、表示 count の所有、末尾逆順 index は再利用を前提にしない |
| H-12 | `b0c2a5f:docs/requirements.md`、`docs/glossary.md`、`docs/redesign/positions.md`、`docs/adr/0020-redesign-review-scope.md` | 広い preset catalog、大会・段位、外部指定局面、ピンイン等の用語、旧資料の保持を記録 | 今回の「旧資料をすべて削除」は旧保持方針を置き換える。他の旧要件は Q-01・Q-02・Q-11・Q-12・Q-14 で明示的に採否を確認する |

## 実装で具体化した制約

| 根拠 | 再設計でも維持を提案する意味上の契約 | 再検討する判断 |
|---|---|---|
| H-11 / 配牌修正 | 親の位置が変わっても正しい席へ配る。相対配牌順と固定 seat index は異なる | 配列の rotate、席型の generic parameter |
| H-11 / 表示分離 | 牌の位置、公開許可、公開情報の参照を区別する | 配牌時の自動公開、表示 count の所有者 |
| H-11 / 補充残量 | 通常・補充の牌資源は同じ全体資源に属し、補充後の通常残量も整合する | 固定配列、末尾逆順、具体的な cursor |
| H-05、H-08 | 内部で分離した配牌の1枚を、利用者に通常の取得牌と同じ選択意味で見せない。同じ牌種の手切/摸切も区別する | 13＋1正規化、独立した取得牌 field |
| H-09 | 履歴依存の資格を現在の牌姿・河だけから再推定しない | 共通の単一 flag、資格別状態の粒度 |
| H-10、H-10a | 局終了の事実を保持し、終局分類と支払い・連荘・対局終了を区別する | typestate の型数、局結果の所有範囲 |
| H-01、H-02 | 限定的な検証と低水準 test を、完全な ruleset や実行 API の完成と扱わない | 旧 test list の進捗や API を新設計の完成基準にすること |

これらは既存実装の流用を要求するものではなく、表現を変えても同じ問題を再発させないための候補契約である。[不変条件案](design.md)の I-01〜I-12 で検証方法を提案する。

## 旧要件の扱い

| 旧方針・要件 | 新設計での扱い |
|---|---|
| 決定性・非同期・並列性・複数ルール・再生・テスト容易性 | 今回も confirmed requirement（G-01〜G-06）。具体的な方式は再選定 |
| 情報秘匿、外部入力検証、到着順で優先を変えない、版固定 | 目的を満たすため維持を提案。[設計案](design.md)の候補契約としてレビュー |
| functional core / imperative shell、状態消費・typestate・data 継続、Gym 非対応、単卓公開 step 禁止 | 要求と方式を分離して Q-02。旧規約をそのまま復活させない |
| 四人/三人、雀魂→天鳳→麻雀一番街の優先、龍龍と競技団体/リーグ catalog、大会・段位・チーム戦 | 対応対象・初期範囲・優先は Q-01。旧 catalog を実装必須一覧にしない |
| `xiangting` と未公開 `hule` の adapter 利用 | 旧指定を Q-11 に記録。API・入手性・能力が未確認のまま依存を確定しない |
| local `majsoul-record` の1パス検証、取得は user が外部で実施、最小手書き fixture と CI 外 corpus | Q-11。外部取得機能は今回作らない |
| 外部指定局面からの期待値評価・学習、残り山の生成方針は生成側 | 履歴上は追加要求。Q-12 で採否・保証・範囲を確認し、候補経路を設計に残す |
| 麻雀識別子のピンイン、`Round` は局、`Matchday` は節、確定済み用語 | Q-14。新しい production 識別子を作らず、日本語概念でレビュー |
| TDD と test list、Rust の詳細規約、全実装の一 assertion 原則等 | 実装再開時に必要な範囲を Q-11 で確認。今回は旧開発手順を再作成しない |

## Infrastructure と変更範囲

| 保持する対象 | 理由・範囲 |
|---|---|
| [CI](../../.github/workflows/ci.yml)、[依存監査](../../.github/workflows/deps-audit.yml)、[文書 CI](../../.github/workflows/documentation.yml) | 既存の fmt・Clippy・build・test、依存監査、Markdown lint・offline links をそのまま保持 |
| [lint 設定](../../.markdownlint-cli2.jsonc)、[deny 設定](../../deny.toml)、[toolchain](../../rust-toolchain.toml) | 設計資料の再作成を理由に toolchain・監査方針を変更しない |
| [workspace metadata](../../Cargo.toml)、crate manifests、[devcontainer](../../.devcontainer/devcontainer.json)、[gitignore](../../.gitignore) | crate 候補を提案する段階で実 workspace を改変しない |
| [LICENSE](../../LICENSE)、[第三者通知](../../THIRD-PARTY-NOTICES.md) | 法的通知は旧設計資料の撤去対象に含めない |
| 既存 Rust ソース・埋込み test、Git history | 今回は参考資産。削除・移動・再実装・履歴書換えを行わない |

root と crate の README は旧設計の規範を残さないよう書き直し、manifest の readme 参照先を維持する。旧文書へのリンクを保持するための空文書や互換 redirect は作らない。

現 CI に docs-rs job はない。旧手順には記載が残っていたが、`5375f6f` で削除済みの workflow と一致していなかった。新 AGENTS では維持した workflow を検証の基準とし、旧手順の内容を転載しない。
