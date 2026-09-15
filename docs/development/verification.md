# 検証コマンドと完了条件

変更を完了するとき、またはCI・toolchainを変更するときに参照する。 [開発手順の入口](../development-guide.md)。節番号は移設前の参照との対応を保つ。

## 12. コマンド

repository rootで実行する。文書変更の検証:

```powershell
git diff --check
markdownlint-cli2 "**/*.md" "#target/**"
lychee --offline --no-progress --root-dir . "**/*.md"
```

Rust の標準検証 command は次のとおりである。

```powershell
cargo fmt -- --check
cargo clippy -- -D warnings
cargo build --verbose
cargo test --verbose
```

dependency auditは独立したworkflowで `cargo deny check` を実行する。変更範囲の確認には `git status --short`、ファイル探索には `rg --files` を使う。これらは検証の成功を示すコマンドではない。

文書だけの変更でRust検証は不要。必須検証の成功後は、追加変更や失敗などの根拠がなければ反復しない。ツール未導入などで未実行の場合は、代替確認の範囲と未実行項目を区別して報告する。

通常のCI jobもversionを別指定せず、repositoryの`rust-toolchain.toml`を使ってformat、Clippy、build、testを実行する。CIは加えてnightlyの`cargo docs-rs`を実行する。`cargo deny check`は`deny.toml`を使う独立workflowで実行する。cargo-deny Actionの`rust-version`はtoolchain fileから自動取得されないため、`rust-toolchain.toml`更新時に同じ値へ更新する。Markdown lintとrepository内リンク検査も独立workflowで実行し、外部URLはmerge gateに含めない。ローカルcommandは各toolを導入済みの場合に利用できる。空scaffoldingがbuildできることと、ドメインの振る舞いが検証済みであることを混同しない。

## 13. Definition of Done

### 文書

- 要求 ID、用語、リンク、ADR が整合する。
- 決定と未決事項が区別されている。
- 変更した公式情報に出典と確認日がある。
- `git diff --check` が成功する。

### 実装（開始後）

- 対象 test list 項目が完了し cycle log がある。
- red を確認し、最小 green、refactor を経ている。
- 三角測量を行った場合、追加した例、一般化した内容、testを削除した場合の根拠がcycle logにある。
- 関連 test、全 test、format、lint が成功する。
- error、event、metrics、replay への影響を検証している。
- public schema、rule preset、ADR、開発文書を必要に応じ更新している。
- benchmark-sensitive な変更は比較結果がある。
