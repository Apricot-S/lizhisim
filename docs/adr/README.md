# Architecture Decision Records

重要な設計判断を時系列で残す。Accepted ADR の意味を変える場合、過去本文を書き換えず新しい ADR で `Supersedes` を示す。誤字やリンク修正は可能だが、判断理由と結果を後から美化しない。

## Status

- `Proposed`: review 中
- `Accepted`: 現在の規範
- `Deprecated`: まだ参照可能だが新規利用を推奨しない
- `Superseded`: 後継 ADR に置換済み
- `Rejected`: 検討したが採用しない

## Index

| ADR | Status | Decision |
|---|---|---|
| [0001](0001-event-driven-typed-continuations.md) | Accepted | Gym/step ではなくイベント駆動・型付き継続を採用する |
| [0002](0002-versioned-rule-layers.md) | Accepted | ルールを層別化し出典付き不変版として管理する |
| [0003](0003-separate-competition-domain.md) | Accepted | 大会ドメインを卓内エンジンから分離する |
| [0004](0004-pinyin-terminology-and-round.md) | Accepted | 麻雀用語はピンインを基本とし、Roundを局専用にする |
| [0005](0005-mahjong-soul-first.md) | Accepted | 雀魂段位戦を最優先のルール実装・検証対象にする |
| [0006](0006-rust-toolchain-workspace-and-ci.md) | Accepted | Rust toolchain、初期 workspace、CI baselineを固定する |

## Template

新しい ADR は次を含める。

- Title / Status / Date / Deciders
- Context
- Decision
- Consequences（positive/negative）
- Alternatives considered
- Follow-up / verification
- Supersedes / Superseded by（該当時）
