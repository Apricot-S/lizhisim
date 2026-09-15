# 外部crate導入とプリセット開発

評価adapterの導入、公式プリセットの追加・改定時に参照する。 [開発手順の入口](../development-guide.md)。節番号は移設前の参照との対応を保つ。

## 7. `xiangting` と `hule` の導入

### 7.1 `xiangting`

1. version、MSRV、license、公開 API、三麻牌構成の扱いを確認する。
2. domain の `XiangtingPort` contract を fake で red/green にする。
3. adapter に同じ contract suite を適用する。
4. 既知牌姿と property/differential corpus を追加する。
5. crate 型を adapter 外へ export していないことを review する。

### 7.2 `hule`

未公開の間は production 相当の独自点数計算器を作って代替しない。port、context、capability、fake と test corpus を先に設計する。利用可能になった時点で次を確認する。

- source の取得・pin 方法
- license と配布可否
- MSRV と feature
- 役、符、上限、責任払い、三麻、local variation の対応範囲
- error semantics
- deterministic behavior

未対応 variation を LizhiSim adapter が補う場合は、その責務と根拠を ADR にし、独立 test list から実装する。

## 8. Rule preset の開発手順

1. source 台帳を更新する。
2. 対象 family/版と対応範囲を決める。
3. 全条項の mapping checklist を作る。
4. 未確認・矛盾・対象外を分類する。
5. test list を作り、差分一つずつ red/green にする。
6. canonical 完全設定を生成し hash を固定する。
7. review と golden suite 後に `verified` にする。

同時に多数 preset を埋めず、最初の一つで schema の不足を学び、次の preset を追加する。schema 変更で既存版の canonical content が変わる場合は migration/新 schema 版として扱う。

### 8.1 実装順

1. 雀魂段位戦（四人/三人）
2. 天鳳段位戦（四人/三人）
3. 麻雀一番街段位戦（四人/三人）

雀魂では [公式詳細ルール](https://mahjongsoul.com/news/46)を一次資料にする。記載外の corner case は [Cryolite/kanachan `src/simulation`](https://github.com/Cryolite/kanachan/tree/main/src/simulation) と [牌譜 ID 付き検証記録](https://gist.github.com/Cryolite/a026f41713f6a7ca88713737f5c2cfb6)から候補を得る。[ADR-0011](../adr/0011-one-pass-majsoul-conformance.md)に従い、CIでは必要な遷移を手書きした最小fixtureを検証する。CI外ではconformance appがuser管理の`majsoul-record`を逐次decode・projectし、中間`game_log`を必須にせずLizhiSimとcheckpointごとに比較する。

元牌譜を取得できない corner case は test list に残し、理由付きで `blocked` とする。先行実装からコードをコピーせず、状態分割・規則差分・牌譜 locator を調査資料として利用する。
