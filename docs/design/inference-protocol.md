# 推論キューと継続プロトコル

## 1. 目的

麻雀の卓は意思決定点へ非同期に到達する。推論 protocol は、多数卓を停止させずに要求を集約し、GPU が処理しやすい batch を作りつつ、各応答を正しい型付き継続へ一度だけ返す。

この protocol は Gym の `observation -> step(action)` を network 化したものではない。主語は環境ではなく要求であり、同時に複数 actor の応答を待つ call window、cancel、retry、backpressure を最初から扱う。

## 2. 要求の種類

初期 decision kind 候補:

- `ChooseTurnAction` — 通常打牌、立直打牌、暗槓、加槓、北抜き、自摸和を含む turn decision
- `ChooseCallResponse` — pass、和了、ポン、チー、明槓を含む call-window slot
- `ChooseAbortiveDraw` — 九種九牌など選択権がある規則
- `ChooseContinuation` — all-last の続行選択を公式規則が認める場合

モデル head を分けるか統合するかは observation/action schema ADR で決める。domain 上は decision point ごとの完全な合法手集合を一つの要求に含め、複数 request 間で排他的 action を選ばせない。

大会側の lineup 選択や opponent selection を将来 request にできるが、table decision と schema/priority を分ける。

## 3. Request envelope

概念 field:

| Field | 目的 |
|---|---|
| `request_id` | global または run 内で一意、idempotency key |
| `experiment_id` | run の分離 |
| `table_match_id` / `round_id` | 所属する卓と `Round` |
| `call_window_id` | call request の barrier、該当時のみ |
| `actor` | 観測主体の seat/agent |
| `decision_kind` | 期待する応答型 |
| `observation_schema` | feature 意味版 |
| `action_schema` | action ID 意味版 |
| `observation` | actor に見える情報の canonical payload |
| `legal_actions` | 選択可能な action ID と必要 metadata |
| `model_key` | artifact、head、sampling policy の選択 |
| `compatibility_key` | batch 可能性を決める値 |
| `rng_key` | sampling を batch packing から独立させる key |
| `deadline_class` | wall clock 値ではなく scheduling class。実時間 deadline は shell metadata |
| `continuation_token` | 外部が意味を解釈しない opaque token |
| `trace_context` | metrics/tracing。ルール意味論には使わない |

観測 payload と合法手は immutable である。retry で同じ `request_id` を使う場合は content hash が一致しなければ integrity error とする。

## 4. Response envelope

概念 field:

- `request_id`
- `model_artifact_hash`
- `observation_schema` / `action_schema`
- selected `action_id`
- optional policy logits/probability/value/auxiliary outputs
- inference backend metadata（device、batch ID、duration）
- sampling metadata（mode、temperature、rng counter）
- response content hash

backend metadata は学習・運用観測用であり、domain transition へ渡さない。選択 action と必要な sampling proof だけを検証済み型へ変換する。

## 5. 応答検証

順序を固定する。

1. envelope の構文、size、schema を検証する。
2. `request_id` の pending/finished/cancelled/unknown を調べる。
3. 保存済み request content hash と response metadata を照合する。
4. model key/artifact と decision kind を照合する。
5. `action_id` が保存済み `LegalActionSet` の member か確認する。
6. action metadata を domain の型付き action に decode する。
7. continuation ownership を compare-and-consume し、二重再開を防ぐ。
8. accepted event を追記してから core を再開する。

不正 action を自動的に pass や自摸切りへ置き換えない。置換方針が必要な実験は `InvalidResponsePolicy` として明示し、元の不正応答と代替 action を両方 event 化する。

## 6. Pending continuation の所有

継続 token は認可 token ではなく相関用 opaque value である。外部 response が token を知っていても、scheduler 内の pending entry と request ID、actor、schema が一致しなければ再開できない。

pending entry は次を保持する。

- typed continuation を含む table task への所有権
- expected response descriptor
- legal action set/hash
- request lifecycle
- call-window membership
- retry count と failure policy
- enqueue/dispatch/complete timestamp（metrics only）

継続自体を inference service や storage backend に送らない。process restart を跨ぐ必要がある場合は、defunctionalized continuation/state snapshot と event offset を durable store から復元する。

## 7. Queue topology

概念上は三段に分ける。

```text
table outbox
  -> admission/backpressure queue
  -> compatibility buckets
  -> device/backend batches
```

### 7.1 Admission

- global outstanding 上限
- per-experiment/per-model/per-table quota
- memory budget
- cancelled experiment の遮断
- priority class

上限に達した場合、table task を `SuspendedNotEnqueued` として保持し、request を失わない。無制限 channel に積んで OOM させない。

### 7.2 Compatibility bucket

同じ model 名だけで batch にせず、入力 shape/dtype、schema、head、sampling semantics、device を含む key で分ける。可変長入力を padding する場合、padding/mask の意味も schema に含める。

### 7.3 Flush

次のいずれかで batch を閉じる。

- maximum batch size/cost 到達
- oldest request の待機 budget 到達
- backend が要求する shape/capacity 到達
- experiment drain/shutdown
- high-priority request の latency budget

flush policy は performance にだけ影響し、request ごとの `rng_key` によって action sampling 結果を batch 配置から分離する。

## 8. 非同期性と公平性

一卓が連続で即時 decision を出す場合も、他卓を starvation させない。scheduler と broker の公平性は別で考える。

- scheduler: runnable table の weighted round-robin または work-stealing
- broker: model bucket 内の oldest-first を基本に priority aging
- experiment: quota により大規模 run が小規模評価を締め出さない

順番の変更が麻雀結果に影響しないことを property test する。同じ request/response mapping を異なる delivery order と batch partition で与え、終端 hash が一致する必要がある。

## 9. CallWindow barrier

候補のない seat には request を出さない。候補が pass のみの場合は policy により自動解決できるが、その最適化は観測/trajectory semantics と一致させる。

window 状態:

```text
Opened -> Collecting -> Ready -> Resolved
                    \-> Cancelled
```

各 slot は `Pending | Responded | TimedOut | Cancelled` のいずれかで、全 required slot が terminal のとき `Ready` になる。最初の和了応答で他要求を即 cancel してはならない。複数和了または三家和の可能性を失うためである。

head-bump rule でも、seat priority 上より前の候補の応答が未確定なら待つ。timeout policy による pass が確定した後に解決する。

## 10. Timeout・切断・失敗

wall clock は core に入れず、scheduler が次の typed result を作る。

- `Responded(Action)`
- `TimedOut(TimeoutPolicyVersion)`
- `BackendFailed(FailureClass, PolicyVersion)`
- `ExperimentCancelled(Cause)`

候補 policy:

- retry same backend
- retry fallback backend with same model artifact
- deterministic default（pass、自摸切りなど）
- forfeit actor/table match
- fail experiment

学習 run で default action を使うと distribution が変わるため、trajectory に mask/invalid flag を付け、通常 sample と区別する。silent retry で model version を変えない。

## 11. Retry と idempotency

transport retry は同じ `request_id` と payload hash を使う。backend が同じ request を複数回処理しても scheduler は最初に accepted された一つだけで continuation を再開する。

sampling backend が retry ごとに別 action を返さないよう、`rng_key` と sampling counter を request に固定する。これを保証できない backend では response を durable に記録してから ack する。

## 12. Observation

Observation schema は rule-independent core feature と optional feature blocks に分けられるが、欠落をゼロ埋めで曖昧にしない。

候補情報:

- own `bingpai`、`fulu`、摸牌
- public 打牌、`fulu`、`lizhi`、北抜き、宝牌表示
- points、seat、場風、本場、供託
- wall/remaining draw の公開情報
- match progression と rule features
- action history の必要範囲
- legal action mask/metadata

rule feature を省いて同じ model を異なるルールで使う場合、モデルがルール差を知る必要がないという明示的実験判断になる。schema metadata に使用 rule feature set を残す。

完全情報 view から observation を作る projection は pure function とし、以下をテストする。

- 他家の `bingpai` に属する牌 identity を含まない。
- 未公開 wall order を含まない。
- seat rotation に対する相対表現が一貫する。
- 赤牌、北抜き、三麻除外牌を schema どおり表す。
- legal mask と domain legal set が双方向に一致する。

## 13. Action schema

固定 action space は GPU に扱いやすい一方、ルールと decision kind ごとに未使用領域が増える。ragged legal list は compact だが gather/scatter が必要になる。選択は benchmark 前に固定しない。

いずれの場合も満たす条件:

- `ActionSchemaVersion` ごとに ID の意味が不変。
- discard copy と tile kind の区別が必要な場面を表現できる。
- 立直 + 打牌を原子的に選べる。
- 複数のチー/槓 decomposition を区別できる。
- pass/和了/自摸和/北抜きを decision kind と整合させる。
- legal set から domain action への decode が全単射として検査できる。

## 14. Trajectory

request と response をそのまま学習 record にせず、versioned projector を使う。

```text
AcceptedDecision + later outcomes
  -> TrajectoryStep {
       observation,
       legal_actions,
       chosen_action,
       behavior_policy_metadata,
       rewards/returns,
       terminal/truncation/failure flags,
       rule/model/schema ids
     }
```

timeout、backend failure、administrative cancel は terminal outcome と区別する。Gym の `terminated/truncated` 互換を目的にせず、麻雀と experiment lifecycle に必要な理由 enum を持つ。

## 15. Metrics

- request rate / accepted response rate
- queue depth by model/schema/device/experiment
- wait、batch build、inference、resume latency
- batch size/cost distribution と padding waste
- runnable/suspended/completed table 数
- timeout/retry/duplicate/late/invalid response 数
- GPU utilization/memory/OOM
- table throughput、round throughput、decision throughput

高 cardinality の table/request ID を通常 metric label にしない。trace または sampled log で相関する。

## 16. Shutdown と drain

shutdown mode を区別する。

- graceful drain: 新規卓を止め、pending request と実行中卓を完了
- checkpoint drain: safe suspension まで進め snapshot/event offset を保存
- immediate cancel: 全 request/window を cancel event で閉じ、run を incomplete とする

process 終了で pending continuation を黙って失わない。成功 manifest は全 table/competition stream が terminal で integrity check 済みの場合だけ出す。

## 17. Gym 非対応の確認

次は提供しない。

- `reset()` / `step()` loop
- 一つの global observation/action space を仮定する interface
- vectorized Gym wrapper
- Gym の termination semantics への lossless でない変換

学習 framework との接続は request/response stream、trajectory file/stream、inference backend port で行う。
