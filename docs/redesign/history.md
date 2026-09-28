# 既存資産とGit historyの調査

調査基準は`e755189`、調査日は2026-09-28。以下はrepository内の実装・文書・Git差分から確認した事実であり、外部公式仕様の現在の正しさを保証しない。新設計の区分は[入口](README.md)を参照する。

## 1. 調査方法と限界

全履歴のcommit一覧を確認し、旧実装の最終期`836fa35`、撤去・再出発、主要な設計変更、現HEADのコードを重点的に読む方法を取った。全commitの全差分を監査したものではない。参照した根拠は次の方法で再確認できる。

```text
git log --reverse --oneline
git show <commit> -- <path>
git show <commit>:<path>
git ls-tree -r --name-only <commit>
```

現コードを実行して性能・正しさを再測定した調査ではない。testの存在は当時の契約の手掛かりとして扱い、今回のテスト成功や完全なルール対応の証明とはしない。commit subjectだけから実装範囲・動機を判断しない。とくに`ce39bd1`はwalking skeleton実装というsubjectだが、変更ファイルは文書であり、非同期・replay完成の根拠にはできない。

## 2. 現在の実装で確認できる範囲

| 対象 | コードで確認したこと | 新設計で前提にできないこと |
|---|---|---|
| [workspace](../../Cargo.toml)・[core入口](../../crates/lizhisim-core/src/lib.rs) | facade、core、rulesの3 crate。coreはrulesに依存 | 3 crate構成の最適性、公開APIの維持 |
| [rules](../../crates/lizhisim-rules/src/rule_set.rs) | 各色赤牌枚数0〜4を検証し、enumで限定した親の開始方式を保持 | ruleset全体のsemantic/capability検証、verified preset |
| [牌構成](../../crates/lizhisim-core/src/tile_set.rs)・[手牌](../../crates/lizhisim-core/src/bingpai.rs) | 赤牌を分けた牌数、通常五との合計上限、手牌追加・除去の検証 | 全局面の牌保存propertyの完成 |
| [牌山](../../crates/lizhisim-core/src/bipai.rs) | 四人の固定牌列、完全multiset検証、配牌、通常取得、補充取得、表示牌参照 | 三人ルール、槓の合法性や成立順序。低水準操作の存在と局進行への統合は別 |
| [局](../../crates/lizhisim-core/src/round.rs) | 配牌、親を基準にした座席対応、ツモ、手切・摸切、反応なしの巡回、枯渇時の荒牌平局 | 合法手の全列挙、和了・副露・立直・フリテン・応答窓・非同期要求 |
| [対局](../../crates/lizhisim-core/src/table_match.rs) | 得点・場・局位置・本場・供託・親の台帳と、終了局を移送する境界 | 実際の点数移動、連荘・次局・対局終了の算出。`RoundSettlement`は現時点では入力を保持する境界 |
| 記録・推論 | 設計文書に要求・event・replay・batchingの将来像がある | 実装済みprotocol、event schema、stable hash、scheduler、評価adapter、conformance app |

`Round::new`はcrate-privateであり、公開入口から一連の完全なsimulationを提供済みという意味でもない。`no_reaction`は反応なしを前提に進める低水準の経路であり、反応可能性をすべて計算していない。新しい合法手モデルの正解oracleには使わない。

## 3. 過去の設計と変更理由

| 根拠 | 過去の設計・変更 | 確認できた理由 | 再設計への示唆 |
|---|---|---|---|
| `836fa35`の`lizhisim/src/bipai/core.rs`、`provider.rs`、`provider/random.rs` | 旧牌山traitは乱数による構築、mutableな取得・槓処理を持ち、固定/乱数providerを分離。乱数providerのcloneで同じ山になるtestがある | 実装で確認。大域的な再現性設計の理由はこのコードからは不明 | RNG状態cloneによる再現と、worker順から独立した実験再現は別の契約 |
| `fcb7eb6`、`4746362`、`0a8cd0f` | hand-tools撤去、旧ファイル撤去、新規設計への再出発 | 削除とmanifest変更は確認。各撤去の詳細な動機はcommit subjectだけでは不明 | 旧手牌評価器を復活させる根拠にはしない。現方針の評価adapter境界を維持 |
| [ADR-0001](../adr/0001-event-driven-typed-continuations.md) | 同期step、callback中心からイベント駆動・型付き継続を選択 | 非同期多数卓、GPU batch、応答競合、永続化可能性 | イベント駆動は要求として維持。closed enum等の具体表現は候補として再評価 |
| [ADR-0002](../adr/0002-versioned-rule-layers.md)、[0003](../adr/0003-separate-competition-domain.md) | 万能設定や卓内への大会混在を避け、ルール層と大会を分離 | 改定周期・責務が異なり、再利用と再生を阻害するため | 層別化の目的を維持し、個別の型名・集約構造は引き継がない |
| [ADR-0007](../adr/0007-content-addressed-source-evidence.md)→[0008](../adr/0008-source-review-without-copying.md)、`9996e91` | 公式原資料のcontent-addressed保存から、URL・版・取得日時・確認記録へ変更 | 原資料複製、著作権、運用負担を避ける | 証跡の確認と原資料保存を混同しない。再設計で保存必須を復活させない |
| [ADR-0009](../adr/0009-minimized-game-record-evidence.md)→[0010](../adr/0010-mahjong-soul-record-conformance.md)→[0011](../adr/0011-one-pass-majsoul-conformance.md)、`0410f87` | 最小fixture中心、外部牌譜の二段階検証を経て、local生牌譜の1パス比較へ | 中間出力管理・stale output・converter bugをoracle化する問題 | source traceと独立した最小testを残し、projector自体も検証する |
| `8fac102`、[ADR-0013](../adr/0013-tile-kind-without-copy-identity.md) | 赤を分けた37牌種とmultiset。同種牌のcopy identityを持たない | source間に存在しないIDをcoreや観測へ漏らさず、赤を行為で区別するため | 赤の区別とsourceからの独立は有用。37要素配列・index・内部表現は再評価 |
| `86e1db5`、[ADR-0014](../adr/0014-facade-and-core-crates.md) | 単一crateからre-export facadeとcoreへ | owner指定と、公開入口を実装所有から分ける意図 | 性能実測で決まった分割ではない。公開互換を新設計の制約にしない |
| [ADR-0015](../adr/0015-rule-and-domain-tile-ownership.md)→[0017](../adr/0017-core-depends-on-rules.md)、`b4fa23e` | rules→coreからcore→rulesへ反転。牌構成解決をcoreへ移動 | ownerがrulesの独立と牌型のcore所有を指定 | rule意味論・入力schema・共有値型の所有を別々に検討する。現依存辺を必然としない |
| `bed389b`、[ADR-0018](../adr/0018-rule-set-validation-names.md) | ユーザー設定を`RuleSet`、検証後を`ValidatedRuleSet`へ改名 | ownerが無印/Validatedの対比を採用 | 名称は検証能力の完成を意味しない。現在の保証は限定的 |
| [ADR-0012](../adr/0012-normalize-dealer-first-draw.md)→[0016](../adr/0016-initial-deal-shouqie-action.md)、`ef783ec`、`ae32e9e`、`bffc635` | 親の14枚配牌を13＋1へ正規化。初回摸切を受理して表示を補正する案から、合法候補・適用とも摸切を拒否する案へ | actionの意味と公開結果がずれ、実際に存在しない選択肢を提示してしまうため | 内部正規化で利用者の選択意味を変えない。13＋1は唯一の解ではない |
| `e755189`、[ADR-0019](../adr/0019-first-zimo-origin-in-rules.md) | 親の開始方式をcore独自入力から検証済みrules経由へ | ownerが設定経路とenum共有を指定 | 開始方式は数値設定に留まらず合法手へ影響。開始方式と現在の取得由来を区別する必要 |

古いADR本文の旧名称は歴史として読む。今回の資料で旧型を挙げる箇所は調査対象の引用であり、新しいproduction識別子の提案ではない。

## 4. 実装・テストで具体化した制約

| 根拠 | 発見・具体化された制約 | 維持すべき契約 | 再検討する実装判断 |
|---|---|---|---|
| `09c06fa`、局の配牌test | 親からの配牌順と固定座席indexは同じでない | 親の位置を変えても正しい座席に配牌する | 配列rotateという実装方法 |
| `99cb437`、牌山の表示test | 配牌と同時に表示牌を自動公開するとrule判断が牌山へ入り込む | 牌の位置、表示の許可、読み取りを分ける | 表示countをどの構造が所有するか |
| `7b66f3c`、`ba2eb6e`、`b444f58`、補充取得test | 補充取得は通常取得できる残量にも影響する | 通常山と補充領域を独立countとして無関係に減らさない | 末尾から逆順のindex、固定配列長 |
| `d40c816`、`33939f9`、既存ドメイン設計 | 河が空でも第一巡資格を保持しているとは限らない | 資格に必要な履歴を保持する | 単一flagで各種の資格を兼用できるかは未検証 |
| `c2d4e6b`、[第一打test list](../test-lists/round-first-dapai.md) | 同じ牌種でも手切・摸切の意味が異なる | 意図と公開結果を一致させる | 同種牌のcount操作、取得牌の格納位置 |
| `e27702c`、`100caba` | 終局理由だけ返すと終局時の状態が失われる | 精算・診断に必要な終端事実を保つ | 終了した局全体を所有するか、十分な結果へ射影するか |
| `807c109`、`e66f83f`、`f91fed8` | 局内で決める終局と、得点・本場・次局の決定は異なる | 不採用の途中流局候補を確定局結果にしない | 精算の集約境界、局中供託の反映方法 |
| [王牌test list](../test-lists/wangpai-replacement-draw-and-baopai.md)、[局初期状態test list](../test-lists/round-player-initial-state.md) | 未実装の局全体観測・合法性・許可に依存する項目は後続へ移送 | 低水準test完了を全ルール検証と誤認しない | 検証用viewと公開観測の境界 |

上記の「再検討」は既存のbug修正が不要だったという意味ではない。内部表現を変えても同じ問題を再発させないため、契約を抽出している。

## 5. 維持する要件と再検討する判断

**Confirmed requirement:** deterministic simulation、非同期要求、応答競合の到着順独立、四人/三人と複数ruleset、情報秘匿、外部入力検証、評価adapter、版固定、出典、replay、TDDを維持する。詳細な要求IDは[設計案](design.md)で対応付ける。

**Candidate design:** 既存の13＋1正規化、37種表現、data化した継続、局結果と対局精算の分離は比較候補。facade/core/rulesの分割、依存方向、固定長容器、typestateの型数、公開methodは引継ぎ前提にしない。

**Unresolved question:** 既存の大会・preset全域を初期releaseへ含めるか、全局面のルール競合順序、再生schemaとRNG、性能目標、`hule`の実能力。未解決一覧の[Q-01〜Q-12](questions.md)へ移す。履歴に理由がない変更を、性能や正しさの証拠から決まったと推測しない。

## 6. Infrastructureの保全

| 対象 | 現状と今回の扱い |
|---|---|
| [Rust CI](../../.github/workflows/ci.yml) | fmt、Clippy、build、testを保持。今回は文書だけなのでRust検証は実施しない |
| [文書CI](../../.github/workflows/documentation.yml)・[lint設定](../../.markdownlint-cli2.jsonc) | Markdown lintとoffline link check、既存設定を保持 |
| [依存監査](../../.github/workflows/deps-audit.yml)・[deny設定](../../deny.toml) | 保持。依存追加なし |
| [toolchain](../../rust-toolchain.toml)・[workspace metadata](../../Cargo.toml)・[lockfile](../../Cargo.lock) | 保持。crate境界案のために実workspaceを変更しない |
| [LICENSE](../../LICENSE)・[THIRD-PARTY-NOTICES](../../THIRD-PARTY-NOTICES.md) | 保持。著作権・第三者通知を再設計に伴って消さない |
| Git設定対象ファイル・devcontainer等 | 今回変更しない。新設計に合わせた削除や初期化を行わない |

文書の[ADR-0006](../adr/0006-rust-toolchain-workspace-and-ci.md)と[検証手順](../development/verification.md)にはdocs-rsの記述が残る一方、`5375f6f`以後の現CIにはdocs-rs jobがない。この差はinfrastructure調査上の留意点として記録する。今回docs-rsを復活・削除する新判断は行わず、将来CIを編集する際に記述と実態を解消する。
