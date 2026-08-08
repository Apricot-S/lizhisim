# ADR-0006: Rust toolchain、初期 workspace、CI baselineを固定する

- Status: Accepted
- Date: 2026-08-09
- Deciders: project owner

## Context

Phase 0 の空 scaffoldingにも、実装開始時の再現可能な開発環境と最低限の品質 gate が必要である。一方、将来の crate 分割や async runtime を先回りして固定すると、walking skeletonから学ぶ余地を失う。

## Decision

- workspace manifest の MSRV は Rust 1.97、Edition 2024 とする。
- 開発 toolchain は `rust-toolchain.toml` で Rust 1.97.1、`default` profileに固定する。
- 通常のCI jobもtoolchain versionを重複指定せず、repositoryの`rust-toolchain.toml`を使う。
- 初期 workspace は root workspace と単一の `lizhisim` library crateとする。
- CI baseline は format、Clippy、build、test、docs-rs とする。
- 通常jobのcommandは`cargo fmt -- --check`、`cargo clippy -- -D warnings`、`cargo build --verbose`、`cargo test --verbose`とする。
- docs-rs 相当の検査は nightly と `cargo docs-rs` を使う。
- dependency auditは`deny.toml`と`EmbarkStudios/cargo-deny-action@v2`を使い、通常CIとは別のworkflowで`cargo deny check`を実行する。当面は`main`へのpushと`main`向けpull requestだけを対象にする。Actionの`rust-version`は`rust-toolchain.toml`と同じ1.97.1に固定し、toolchain更新時に同期する。
- Markdown lint、リンク検査は未導入として追跡し、存在しない gate を完了扱いしない。
- domain の責務境界が実測できるまで crate を細分化しない。

## Consequences

- ローカル開発の compiler は patch version まで再現できる。
- ローカル開発と通常のCI jobは同じRust 1.97.1 toolchainを使う。compiler versionの差による検査結果のずれを避けられる。
- 空 crate のCI成功はドメインの正しさを意味しない。
- dependency supply-chain検査はCIで再現できる。文書検査は引き続きopen decisionである。

## Alternatives considered

- 最初から複数 crateに分割する: 境界の変更頻度と compile costを測れないため採用しない。
- CIで`stable` channelを明示する: `rust-toolchain.toml`と異なるcompilerを検査する意図はないため採用しない。
- dependency auditを通常CI workflowへ同居させる: 実行責務と結果を分離するため採用しない。

## Follow-up

- `deny.toml`の許可license、重複version、source方針はdependency追加時にreviewする。
- 実装開始時に `cargo check` を独立したCI jobにする要否を決める。
- crate分割は walking skeletonの依存方向と変更頻度を確認してから後継ADRで決める。
