# Runvil: How to Interoperate with Other Agents (A2A)

Report type: evaluation / review. It judges Runvil (the Gateway, the Harness and the plugin API) against 5 popular codebases sharing its technique, Rust/tokio/axum servers that speak Google's Agent2Agent (A2A) protocol, and prescribes idioms to adopt, in payoff order.

## Executive summary

Runvil has the best task engine in this field and no way for any other agent to reach it. Inside one run, its task state machine, multi-turn replies, deterministic provenance and cancellation tree match or beat every reference. But nothing outside the process can discover a Runvil agent, address one of its tasks, replay its progress or authenticate as a distinct peer. The top finding is an agent card whose advertised capabilities also gate the server. It is small to build, all five references converge on it, and it is the prerequisite for everything else.

### Key findings

1. **Publish an agent card whose capabilities gate the server (all five references).** Serve `/.well-known/agent-card.json` and check the same flags in the core, so what the card claims and what the server does cannot drift. Confidence: high.
2. **Make task progress a resubscribable stream backed by a per-task event log (turul-a2a, a2a-rs, ra2a).** A subscriber gets the latest snapshot and then only events after its sequence number, and a client disconnect never cancels the task. Confidence: high.
3. **Route every task write through one sink with atomic terminal transitions (turul-a2a, a2a-rs).** Use pure state sets, a terminal compare-and-set and a single atomic `begin_cancel`. Confidence: high.
4. **Resolve multi-turn `(task_id, context_id)` pairs in one table (a2a-rs, ra2a).** This maps Runvil's Idle and `tasks.send` directly onto A2A's `input-required`. Confidence: high.
5. **Have agent code emit events while one handler owns the protocol (a2a-rs, ra2a).** Transports become thin adapters, and the client mirrors the same operations. Confidence: medium.
6. **Seal session state into the outward id (agentgateway).** Any instance can then resume it without a shared store. Confidence: low.
7. **Adopt a typed part and artifact model kept apart from transport (ra2a).** That means text, raw, URL and data parts, plus chunked artifacts. Confidence: medium.
8. **Send push notifications through an SSRF-safe sender and a delivery queue (a2a-rs, turul-a2a, ra2a).** The safety check happens at DNS-resolution time, and secrets are redacted. Confidence: high.
9. **Give each peer its own identity (a2a-switchboard, agentgateway, turul-a2a).** Mint per-peer tokens, substitute credentials at the proxy, and make 401s name the OAuth metadata URL. Confidence: medium.
10. **Build an outbound peer client that picks its transport from the card and can reach firewalled peers (ra2a, a2a-switchboard, a2a-rs).** Confidence: medium.

## Method

Runvil was profiled first through nine lenses: modules, state, boundaries, errors, lifecycle, testing, comments, build and messes. That profile produced seven ranked deficits and five strengths. A field survey found Rust A2A servers and gateways and verified each candidate's stack against its manifest and source, not its README. Five references were cloned and read in parallel at pinned commits. Each idiom then got a provenance tag from its file history, and where AI-marked commits touched a cited file, it was compared against the last pre-AI version. The synthesis clustered idioms by how many references converge on them and mapped each to a Runvil deficit. Finally, all 50 citations were checked against the pinned clones: 47 passed as written, 3 were corrected, and one range was dropped as unverifiable. No finding was lost.

## Reference projects and provenance

| Reference | Popularity | Why chosen | License | Provenance of cited idioms |
|---|---|---|---|---|
| [a2aproject/a2a-rs](https://github.com/a2aproject/a2a-rs) | 116 stars | Official Rust SDK on Runvil's exact stack (edition 2024, tokio, axum 0.8, reqwest) | Apache-2.0 | 4 strong human signal, 7 unknown, 6 explicit AI marker (each held up or tightened against its pre-AI form) |
| [agentgateway/agentgateway](https://github.com/agentgateway/agentgateway) | 6,256 stars | A single Rust proxy binary brokering LLM, MCP and A2A, the same technique as the Runvil Gateway | Apache-2.0 | 2 strong human signal, 8 explicit AI marker (all tightened or still present against the pre-AI form) |
| [bacnh85/a2a-switchboard](https://github.com/bacnh85/a2a-switchboard) | 3 stars | A small axum broker binary that is readable whole, the closest to a local-broker shape | Apache-2.0 | 8 strong human signal |
| [aussierobots/turul-a2a](https://github.com/aussierobots/turul-a2a) | 5 stars | A2A v1.0 with durable task storage, a hardened push dispatcher and an auth crate | MIT OR Apache-2.0 | 13 strong human signal |
| [qntx-labs/ra2a](https://github.com/qntx-labs/ra2a) | 169 stars | A compact v1.0 SDK, a second and leaner take on the executor/task-store split | MIT OR Apache-2.0 | 8 unknown |

Popularity is compared only within GitHub stars. The shortlist is the best verified match found within the search budget, not the field's most popular projects.

## Baseline: where the subject stands

Runvil is about 70% Rust by file count. The Gateway is one always-on loopback binary, an OpenAI-compatible inference broker that local tools find through `gateway.json`. The Harness and Engine are git-pinned libraries a host links, and plugins load through a stepwise C ABI. Orchestration exists only as in-process Engine tasks (`tasks.spawn/send/reply/receive/status/cancel/fanout`) inside one run. There is no A2A surface at all: no agent card, no `/.well-known` route, no task endpoint and no webhooks.

Its strengths are real. The task arena is explicit (Running, Idle, Ended, Canceled and Abandoned, with the outcome kept separate from the state) in [`scheduler/tasks.rs`](https://github.com/cppalliance/runvil-harness/blob/db4cf1f4adbda68ca7ed8a22fa8df794110b5d97/crates/runvil-internal/runtime/src/execute/scheduler/tasks.rs). Multi-turn replies are built in, events carry deterministic provenance, cancellation is a tree, and route tiers are encoded in types.

The deficits the findings map to, worst first:

- D1. Peers have no agent identity or discovery.
- D2. Tasks are confined to one process and one run, and can't be addressed, persisted, resumed or replayed.
- D3. Inbox messages are text only, with no parts and no artifacts.
- D4. Models can't delegate, because the `runvil/subagents` Task tools are reserved but unbuilt.
- D5. Streaming covers model tokens only. Progress is a snapshot with no replay, and there's no push.
- D6. Auth is one shared bearer key plus loopback trust. `X-Runvil-Client` is self-asserted.
- D7. There's no outbound peer-agent client.

## Detailed findings, ranked by payoff

### Finding 1: Publish an agent card whose capabilities gate the server

turul-a2a checks capabilities in the core before anything touches storage ([`capability.rs:1-18`](https://github.com/aussierobots/turul-a2a/blob/fa691278f4565bac9cc80529cffe25cdd5442192/crates/turul-a2a/src/capability.rs#L1-L18), [`capability.rs:62`](https://github.com/aussierobots/turul-a2a/blob/fa691278f4565bac9cc80529cffe25cdd5442192/crates/turul-a2a/src/capability.rs#L62)). Its validating card builder leaves the streaming, push and extended-card flags unset by default ([`card_builder.rs:178-200`](https://github.com/aussierobots/turul-a2a/blob/fa691278f4565bac9cc80529cffe25cdd5442192/crates/turul-a2a/src/card_builder.rs#L178-L200), [`card_builder.rs:313`](https://github.com/aussierobots/turul-a2a/blob/fa691278f4565bac9cc80529cffe25cdd5442192/crates/turul-a2a/src/card_builder.rs#L313)), and its auth middleware contributes the card's security schemes ([`traits.rs:22-40`](https://github.com/aussierobots/turul-a2a/blob/fa691278f4565bac9cc80529cffe25cdd5442192/crates/turul-a2a/src/middleware/traits.rs#L22-L40)). a2a-rs serves the card with an ETag and a five-minute max-age ([`agent_card.rs:15-83`](https://github.com/a2aproject/a2a-rs/blob/700a6a28eb001f6fc32bc9446078ff87bdd18c6c/a2a-server/src/agent_card.rs#L15-L83)). It refuses only on an explicit `false`, enforces required extensions, and checks part media types against `defaultInputModes` ([`handler.rs:682-774`](https://github.com/a2aproject/a2a-rs/blob/700a6a28eb001f6fc32bc9446078ff87bdd18c6c/a2a-server/src/handler.rs#L682-L774), [`handler.rs:813-858`](https://github.com/a2aproject/a2a-rs/blob/700a6a28eb001f6fc32bc9446078ff87bdd18c6c/a2a-server/src/handler.rs#L813-L858)). ra2a pairs a public card route with an authenticated extended card ([`lib.rs:69-83`](https://github.com/qntx-labs/ra2a/blob/1a76d6e5af906737974fecfb4b5960fe8431dd9d/ra2a/src/lib.rs#L69-L83), [`http.rs:105`](https://github.com/qntx-labs/ra2a/blob/1a76d6e5af906737974fecfb4b5960fe8431dd9d/ra2a/src/server/http.rs#L105), [`http.rs:138`](https://github.com/qntx-labs/ra2a/blob/1a76d6e5af906737974fecfb4b5960fe8431dd9d/ra2a/src/server/http.rs#L138)). The two gateways show what a broker adds on top. a2a-switchboard returns admitted peers to authenticated callers as a directory, with URLs rewritten to `/peer/{name}/` ([`peers.rs:431-509`](https://github.com/bacnh85/a2a-switchboard/blob/235dc934bf4967999819ad26c6190b8bdd6dc8db/src/peers.rs#L431-L509), [`lib.rs:157-158`](https://github.com/bacnh85/a2a-switchboard/blob/235dc934bf4967999819ad26c6190b8bdd6dc8db/src/lib.rs#L157-L158)). agentgateway rewrites both the v0.3 `url` and the v1.0 `supportedInterfaces[].url` with whole-segment matching, covered by 30 tests ([`mod.rs:184-240`](https://github.com/agentgateway/agentgateway/blob/bdb36de0ccdb1d5954486423698f5231a176b157/crates/agentgateway/src/a2a/mod.rs#L184-L240), [`tests.rs`](https://github.com/agentgateway/agentgateway/blob/bdb36de0ccdb1d5954486423698f5231a176b157/crates/agentgateway/src/a2a/tests.rs)).

This replaces Runvil's local-only `gateway.json` discovery and its static route registry, which advertise nothing to peers. The fix: add a card type to the plugin API, serve it from the Gateway's open tier, and derive both the card and the server's refusals from one capability struct. Authenticated callers can additionally see a directory of the Engine agents they're allowed to reach.

Confidence: high, because all five references converge and the cost is small.

### Finding 2: Make task progress a resubscribable stream backed by a per-task event log

turul-a2a stores every task event with a sequence number assigned by the store, and one stream function serves every transport. It sends the snapshot first, then only events after the client's sequence, and it closes at a terminal state or `input-required` ([`task_stream.rs:1-16`](https://github.com/aussierobots/turul-a2a/blob/fa691278f4565bac9cc80529cffe25cdd5442192/crates/turul-a2a/src/streaming/task_stream.rs#L1-L16), [`event_store.rs:18-56`](https://github.com/aussierobots/turul-a2a/blob/fa691278f4565bac9cc80529cffe25cdd5442192/crates/turul-a2a/src/storage/event_store.rs#L18-L56)). a2a-rs keeps a registry of live executions, each with a broadcast channel, a sequence number and its latest snapshot. A waiter registry lets a client subscribe between turns, and a disconnecting client never stops the execution ([`handler.rs:20-239`](https://github.com/a2aproject/a2a-rs/blob/700a6a28eb001f6fc32bc9446078ff87bdd18c6c/a2a-server/src/handler.rs#L20-L239), [`handler.rs:453-503`](https://github.com/a2aproject/a2a-rs/blob/700a6a28eb001f6fc32bc9446078ff87bdd18c6c/a2a-server/src/handler.rs#L453-L503), [`handler.rs:1300-1338`](https://github.com/a2aproject/a2a-rs/blob/700a6a28eb001f6fc32bc9446078ff87bdd18c6c/a2a-server/src/handler.rs#L1300-L1338)). ra2a has the same split as a design ([`default.rs:302-370`](https://github.com/qntx-labs/ra2a/blob/1a76d6e5af906737974fecfb4b5960fe8431dd9d/ra2a/src/server/handler/default.rs#L302-L370)), but its streamed path never persists, so copy the shape and not the code.

This replaces the Gateway's `watch`-based progress hub, which has no replay, and gives Engine `Event`s, already serializable and provenance-stamped, a durable home. The fix: append each task's Engine events to a per-task log keyed by its dot-path id, and serve snapshot-then-tail over SSE from the Gateway.

Confidence: high, because three references converge and the design fits Runvil's existing event type.

### Finding 3: Route every task write through one sink with atomic terminal transitions

turul-a2a puts a pure transition function and its TERMINAL, EXECUTOR_HELD and CLIENT_HELD sets in its types crate ([`state_machine.rs:28-105`](https://github.com/aussierobots/turul-a2a/blob/fa691278f4565bac9cc80529cffe25cdd5442192/crates/turul-a2a-types/src/state_machine.rs#L28-L105)). It generates its SQL from the same constants ([`terminal_cas.rs:17-40`](https://github.com/aussierobots/turul-a2a/blob/fa691278f4565bac9cc80529cffe25cdd5442192/crates/turul-a2a/src/storage/terminal_cas.rs#L17-L40)) and makes an `EventSink` the executor's only write path. The sink commits the task and its event together and refuses writes after a terminal state ([`event_sink.rs:1-50`](https://github.com/aussierobots/turul-a2a/blob/fa691278f4565bac9cc80529cffe25cdd5442192/crates/turul-a2a/src/event_sink.rs#L1-L50), [`event_sink.rs:148-310`](https://github.com/aussierobots/turul-a2a/blob/fa691278f4565bac9cc80529cffe25cdd5442192/crates/turul-a2a/src/event_sink.rs#L148-L310)). a2a-rs makes cancel a single store operation, `begin_cancel`, deliberately with no default implementation, because any default would be a racy read-then-write ([`store.rs:41-50`](https://github.com/a2aproject/a2a-rs/blob/700a6a28eb001f6fc32bc9446078ff87bdd18c6c/a2a-server/src/task_store/store.rs#L41-L50), [`inmemory.rs:119-137`](https://github.com/a2aproject/a2a-rs/blob/700a6a28eb001f6fc32bc9446078ff87bdd18c6c/a2a-server/src/task_store/inmemory.rs#L119-L137)). A drop guard releases waiting subscribers if a cancel fails partway ([`handler.rs:1169-1298`](https://github.com/a2aproject/a2a-rs/blob/700a6a28eb001f6fc32bc9446078ff87bdd18c6c/a2a-server/src/handler.rs#L1169-L1298)).

This is the persistence half of D2. Today the scheduler's `TaskArena` is in-memory and process-scoped. The fix: express the arena's existing transitions as named state sets, and add a store trait whose terminal write and cancel are each one atomic operation. Keep Runvil's cancellation tree for propagation (see "Where the subject already matches or beats the references").

Confidence: high, because two references converge and it extends a structure Runvil already has.

### Finding 4: Resolve multi-turn `(task_id, context_id)` pairs in one table

a2a-rs rejects a follow-up turn when the task id is unknown, the context id doesn't match or the task is already terminal. Otherwise it appends the agent's last question and the new user message to the history ([`handler.rs:883-967`](https://github.com/a2aproject/a2a-rs/blob/700a6a28eb001f6fc32bc9446078ff87bdd18c6c/a2a-server/src/handler.rs#L883-L967)). ra2a puts the four id cases in one function and loads `reference_task_ids` as related tasks ([`default.rs:212-297`](https://github.com/qntx-labs/ra2a/blob/1a76d6e5af906737974fecfb4b5960fe8431dd9d/ra2a/src/server/handler/default.rs#L212-L297), [`executor.rs:98-165`](https://github.com/qntx-labs/ra2a/blob/1a76d6e5af906737974fecfb4b5960fe8431dd9d/ra2a/src/server/executor.rs#L98-L165)).

This maps Runvil's Idle state and `tasks.reply`/`tasks.send` onto A2A's `input-required` with no new state. The related-task load also gives the future `runvil/subagents` tools (D4) a way to reference earlier work. The fix: one resolver at the Gateway's A2A edge that turns an incoming id pair into an Engine task handle or one of three typed refusals.

Confidence: high, because two references converge and the change is small.

### Finding 5: Have agent code emit events while one handler owns the protocol

In a2a-rs, an agent implements one trait that returns a stream of task, status and artifact events ([`executor.rs:42-50`](https://github.com/a2aproject/a2a-rs/blob/700a6a28eb001f6fc32bc9446078ff87bdd18c6c/a2a-server/src/executor.rs#L42-L50)). JSON-RPC, REST and gRPC are adapters over one `RequestHandler` ([`handler.rs:584-647`](https://github.com/a2aproject/a2a-rs/blob/700a6a28eb001f6fc32bc9446078ff87bdd18c6c/a2a-server/src/handler.rs#L584-L647)). The client's `Transport` trait carries the same 11 operations ([`transport.rs:21-107`](https://github.com/a2aproject/a2a-rs/blob/700a6a28eb001f6fc32bc9446078ff87bdd18c6c/a2a-client/src/transport.rs#L21-L107)), so one end-to-end test drives the same agent over every transport ([`transports_e2e.rs`](https://github.com/a2aproject/a2a-rs/blob/700a6a28eb001f6fc32bc9446078ff87bdd18c6c/examples/tests/transports_e2e.rs)). ra2a has the same executor contract ([`executor.rs:17-45`](https://github.com/qntx-labs/ra2a/blob/1a76d6e5af906737974fecfb4b5960fe8431dd9d/ra2a/src/server/executor.rs#L17-L45)).

Runvil's Engine already returns effects plus events from `Run::step`, so this mostly means not letting A2A wire types leak inward. The fix: one A2A request handler in the Gateway over Engine tasks, with JSON-RPC and REST as thin modules. Mirror its operations in a client trait so Finding 10 reuses them.

Confidence: medium, because ra2a's handler code is buggy and only a2a-rs demonstrates the full pattern.

### Finding 6: Seal session state into the outward id

agentgateway encrypts session state with AES-256-GCM into the id it hands out. Any replica can resume it, and resume fails closed when the backend set changed ([`sessionpersistence.rs:1-75`](https://github.com/agentgateway/agentgateway/blob/bdb36de0ccdb1d5954486423698f5231a176b157/crates/agentgateway/src/http/sessionpersistence.rs#L1-L75), [`sessionpersistence.rs:158-185`](https://github.com/agentgateway/agentgateway/blob/bdb36de0ccdb1d5954486423698f5231a176b157/crates/agentgateway/src/http/sessionpersistence.rs#L158-L185), [`session.rs:932-967`](https://github.com/agentgateway/agentgateway/blob/bdb36de0ccdb1d5954486423698f5231a176b157/crates/agentgateway/src/mcp/session.rs#L932-L967), [`handler.rs:660-661`](https://github.com/agentgateway/agentgateway/blob/bdb36de0ccdb1d5954486423698f5231a176b157/crates/agentgateway/src/mcp/handler.rs#L660-L661)).

For Runvil this would let a restarted Gateway accept a resumed A2A context without a shared store. The fix: seal the Engine task's routing facts into the external context id with a key kept in the run directory.

Confidence: low, because there's one reference and the Runvil Gateway is a single local binary, where Finding 3's store may make this unnecessary.

### Finding 7: Adopt a typed part and artifact model kept apart from transport

ra2a mirrors the A2A proto in a `types` module. Parts are `PartContent::{Text,Raw,Url,Data}` plus media type, filename and metadata ([`part.rs:27-50`](https://github.com/qntx-labs/ra2a/blob/1a76d6e5af906737974fecfb4b5960fe8431dd9d/ra2a/src/types/part.rs#L27-L50)), and artifacts stream in chunks with `append` and `last_chunk` ([`event.rs:59-77`](https://github.com/qntx-labs/ra2a/blob/1a76d6e5af906737974fecfb4b5960fe8431dd9d/ra2a/src/types/event.rs#L59-L77), [`task.rs:279-295`](https://github.com/qntx-labs/ra2a/blob/1a76d6e5af906737974fecfb4b5960fe8431dd9d/ra2a/src/types/task.rs#L279-L295)). Hand-written serde tells variants apart by JSON member name, and tests pin the exact JSON ([`part.rs:152-260`](https://github.com/qntx-labs/ra2a/blob/1a76d6e5af906737974fecfb4b5960fe8431dd9d/ra2a/src/types/part.rs#L152-L260), [`event.rs:97-170`](https://github.com/qntx-labs/ra2a/blob/1a76d6e5af906737974fecfb4b5960fe8431dd9d/ra2a/src/types/event.rs#L97-L170), [`task.rs:94-129`](https://github.com/qntx-labs/ra2a/blob/1a76d6e5af906737974fecfb4b5960fe8431dd9d/ra2a/src/types/task.rs#L94-L129)).

This replaces the text-only inbox in [`scheduler/inbox.rs`](https://github.com/cppalliance/runvil-harness/blob/db4cf1f4adbda68ca7ed8a22fa8df794110b5d97/crates/runvil-internal/runtime/src/execute/scheduler/inbox.rs). The fix: a transport-free part and artifact type in the plugin API, with golden JSON tests, used by both the inbox and the A2A edge.

Confidence: medium, because it comes from one code reference, but that reference mirrors the published spec.

### Finding 8: Send push notifications through an SSRF-safe sender and a delivery queue

a2a-rs validates the webhook URL, then uses a DNS resolver that drops private and metadata addresses at connect time, which defeats DNS rebinding. It re-checks every redirect, disables the system proxy and rejects line breaks in credentials ([`sender.rs:20-78`](https://github.com/a2aproject/a2a-rs/blob/700a6a28eb001f6fc32bc9446078ff87bdd18c6c/a2a-server/src/push/sender.rs#L20-L78), [`sender.rs:176-260`](https://github.com/a2aproject/a2a-rs/blob/700a6a28eb001f6fc32bc9446078ff87bdd18c6c/a2a-server/src/push/sender.rs#L176-L260)). The validator is fuzzed ([`push_url_validation.rs`](https://github.com/a2aproject/a2a-rs/blob/700a6a28eb001f6fc32bc9446078ff87bdd18c6c/fuzz/fuzz_targets/push_url_validation.rs)). turul-a2a adds a redacted `Secret<T>` with no `PartialEq`, enum-only failure records and generation-fenced delivery claims ([`ssrf.rs:1-60`](https://github.com/aussierobots/turul-a2a/blob/fa691278f4565bac9cc80529cffe25cdd5442192/crates/turul-a2a/src/push/ssrf.rs#L1-L60), [`secret.rs:1-60`](https://github.com/aussierobots/turul-a2a/blob/fa691278f4565bac9cc80529cffe25cdd5442192/crates/turul-a2a/src/push/secret.rs#L1-L60), [`claim.rs:7-70`](https://github.com/aussierobots/turul-a2a/blob/fa691278f4565bac9cc80529cffe25cdd5442192/crates/turul-a2a/src/push/claim.rs#L7-L70)). ra2a shows per-task config CRUD ([`push.rs:14-300`](https://github.com/qntx-labs/ra2a/blob/1a76d6e5af906737974fecfb4b5960fe8431dd9d/ra2a/src/server/push.rs#L14-L300)).

Runvil has no push at all (D5). The fix: per-task push configs, delivered from a queue by a sender built like a2a-rs's, never inline in the event loop (see "Messes we should explicitly not copy").

Confidence: high, because three references converge and the sender is small.

### Finding 9: Give each peer its own identity

a2a-switchboard mints a per-peer caller token at enrolment, shows it once, and decides every management action in one function with constant-time compares ([`peers.rs:367-402`](https://github.com/bacnh85/a2a-switchboard/blob/235dc934bf4967999819ad26c6190b8bdd6dc8db/src/peers.rs#L367-L402), [`peers.rs:128-133`](https://github.com/bacnh85/a2a-switchboard/blob/235dc934bf4967999819ad26c6190b8bdd6dc8db/src/peers.rs#L128-L133), [`auth.rs:32-86`](https://github.com/bacnh85/a2a-switchboard/blob/235dc934bf4967999819ad26c6190b8bdd6dc8db/src/auth.rs#L32-L86)). It always strips the inbound Authorization header and sends outbound calls with only the target peer's token ([`peers.rs:678-689`](https://github.com/bacnh85/a2a-switchboard/blob/235dc934bf4967999819ad26c6190b8bdd6dc8db/src/peers.rs#L678-L689), [`peers.rs:776-801`](https://github.com/bacnh85/a2a-switchboard/blob/235dc934bf4967999819ad26c6190b8bdd6dc8db/src/peers.rs#L776-L801)). agentgateway's auth errors carry the OAuth protected-resource metadata URL, kept separate from a client-safe message ([`mod.rs:217`](https://github.com/agentgateway/agentgateway/blob/bdb36de0ccdb1d5954486423698f5231a176b157/crates/agentgateway/src/proxy/mod.rs#L217), [`mod.rs:435-440`](https://github.com/agentgateway/agentgateway/blob/bdb36de0ccdb1d5954486423698f5231a176b157/crates/agentgateway/src/proxy/mod.rs#L435-L440), [`auth.rs:162-200`](https://github.com/agentgateway/agentgateway/blob/bdb36de0ccdb1d5954486423698f5231a176b157/crates/agentgateway/src/mcp/auth.rs#L162-L200)). turul-a2a enforces at compile time that credential types never implement `Debug` ([`no_debug_guards.rs:1-16`](https://github.com/aussierobots/turul-a2a/blob/fa691278f4565bac9cc80529cffe25cdd5442192/crates/turul-a2a-auth/tests/no_debug_guards.rs#L1-L16)).

This replaces the shared bearer key and the self-asserted `X-Runvil-Client` header. The fix: per-peer tokens stored hashed (the switchboard stores them in plaintext), one authorization function, credential substitution on outbound calls, and a `WWW-Authenticate` challenge that names the metadata URL.

Confidence: medium, because two references converge, but neither is a full OAuth implementation.

### Finding 10: Build an outbound peer client that picks its transport from the card and can reach firewalled peers

ra2a's `ClientFactory` picks a transport from the peer card's `supported_interfaces` in the client's order of preference ([`factory.rs:111-191`](https://github.com/qntx-labs/ra2a/blob/1a76d6e5af906737974fecfb4b5960fe8431dd9d/ra2a/src/client/factory.rs#L111-L191)). a2a-switchboard reaches peers behind NAT through a reverse channel. Requests go down an SSE stream and replies come back by POST with a 32-byte secret minted per connection. A full bounded queue returns a retryable 503, and dropping the stream fails all pending requests with 502 ([`channel.rs:26-224`](https://github.com/bacnh85/a2a-switchboard/blob/235dc934bf4967999819ad26c6190b8bdd6dc8db/src/channel.rs#L26-L224), [`channel.rs:253-404`](https://github.com/bacnh85/a2a-switchboard/blob/235dc934bf4967999819ad26c6190b8bdd6dc8db/src/channel.rs#L253-L404)). a2a-rs's client `Transport` trait is the interface to code against ([`transport.rs:21-107`](https://github.com/a2aproject/a2a-rs/blob/700a6a28eb001f6fc32bc9446078ff87bdd18c6c/a2a-client/src/transport.rs#L21-L107)).

Runvil's only outbound interop is MCP and a single OpenAI `Upstream` (D7). The fix: an A2A client tool pack that resolves a card, picks a transport, and exposes peers to models as delegation targets, which also unlocks D4.

Confidence: medium, because three references each show a different half.

## Provenance

Two references carry AI-marked commits on cited files: a2aproject/a2a-rs and agentgateway.

- **a2a-rs (13 commits with Claude or Warp co-author trailers, 2026-09-30 to 2026-11-06).** Its `handler.rs` already had the live-execution registry, the `RequestHandler` shape, `begin_cancel`, the capability gates and the follow-up rules at the pre-AI commit `18fadd4`. Later AI-marked commits added input waiters, the release-on-drop guard and the required-extension check, so Findings 1–5 are tightened, not degraded. The in-memory store's `begin_cancel` body is unchanged since `9f6ee72`.
- **agentgateway (118 AI-marked commits across the repo).** In the cited files, the card rewrite, the exhaustive policy destructure, the new trace attributes and the auth challenge were all added after each file's pre-AI commit. The rewrite and the destructure trace to commits with no AI marker. The resume path, the protected-resource handling and the read-lock guard are unchanged since their pre-AI form.

Tags were assigned per file, not per line. For agentgateway, line-level blame on the partial clone timed out.

## Where the subject already matches or beats the references

- **Task state machine.** Runvil beats all five. turul-a2a's state sets come closest, but no reference keeps the outcome separate from the state or tells abandonment apart from cancellation.
- **Multi-turn replies.** Runvil matches a2a-rs and ra2a, and goes further by delivering messages in the middle of a model loop.
- **Deterministic provenance.** Runvil beats all five, because no reference derives task ids deterministically from a seed.
- **Cancellation.** Runvil beats a2a-rs and turul-a2a. They cancel one task atomically, while Runvil cancels a whole tree across the Engine, the Harness and the plugin ABI. The atomic store operation in Finding 3 complements the tree rather than replacing it.
- **Boundary errors.** Runvil matches ra2a's per-wire error projection ([`error.rs:26-206`](https://github.com/qntx-labs/ra2a/blob/1a76d6e5af906737974fecfb4b5960fe8431dd9d/ra2a/src/error.rs#L26-L206)) and beats it in practice, because ra2a's JSON-RPC error path is broken.

## Messes we should explicitly not copy

- **a2aproject/a2a-rs.**
  - It awaits webhooks one after another inside the event loop, before live subscribers see the event, with a 30-second timeout and no retry ([`handler.rs:540-551`](https://github.com/a2aproject/a2a-rs/blob/700a6a28eb001f6fc32bc9446078ff87bdd18c6c/a2a-server/src/handler.rs#L540-L551)).
  - Its `TaskVersion` is documented for optimistic concurrency but never compared ([`store.rs:7`](https://github.com/a2aproject/a2a-rs/blob/700a6a28eb001f6fc32bc9446078ff87bdd18c6c/a2a-server/src/task_store/store.rs#L7)).
  - Its SSE ids claim `Last-Event-ID` support that no handler reads ([`sse.rs:17-22`](https://github.com/a2aproject/a2a-rs/blob/700a6a28eb001f6fc32bc9446078ff87bdd18c6c/a2a-server/src/sse.rs#L17-L22)).
  - `handler.rs` is 5,488 lines long.
- **agentgateway.**
  - `build_agent_path` does a whole-URI string replace, which the code itself admits is a bug ([`mod.rs:276-281`](https://github.com/agentgateway/agentgateway/blob/bdb36de0ccdb1d5954486423698f5231a176b157/crates/agentgateway/src/a2a/mod.rs#L276-L281)).
  - It has multi-thousand-line files, among them `proxy/httpproxy.rs` at 5,754 lines.
- **bacnh85/a2a-switchboard.**
  - Its card claims streaming while the proxy buffers whole responses ([`peers.rs:501`](https://github.com/bacnh85/a2a-switchboard/blob/235dc934bf4967999819ad26c6190b8bdd6dc8db/src/peers.rs#L501), [`peers.rs:724`](https://github.com/bacnh85/a2a-switchboard/blob/235dc934bf4967999819ad26c6190b8bdd6dc8db/src/peers.rs#L724)).
  - Reconnect cleanup removes the newer channel ([`channel.rs:113-121`](https://github.com/bacnh85/a2a-switchboard/blob/235dc934bf4967999819ad26c6190b8bdd6dc8db/src/channel.rs#L113-L121)).
  - Its "deny-by-default egress" checks only scheme and host ([`state.rs:1044-1055`](https://github.com/bacnh85/a2a-switchboard/blob/235dc934bf4967999819ad26c6190b8bdd6dc8db/src/state.rs#L1044-L1055)).
  - It stores tokens in plaintext.
- **aussierobots/turul-a2a.**
  - `storage/dynamodb.rs` is 7,009 lines long.
  - Three storage backends duplicate the compare-and-set, claim and retention logic.
  - Stream payloads are untyped `serde_json::Value` ([`mod.rs:38-70`](https://github.com/aussierobots/turul-a2a/blob/fa691278f4565bac9cc80529cffe25cdd5442192/crates/turul-a2a/src/streaming/mod.rs#L38-L70)).
- **qntx-labs/ra2a.**
  - Streamed tasks are never saved or pushed, and artifacts are never stored ([`default.rs:331-370`](https://github.com/qntx-labs/ra2a/blob/1a76d6e5af906737974fecfb4b5960fe8431dd9d/ra2a/src/server/handler/default.rs#L331-L370)).
  - Its JSON-RPC errors come back re-wrapped as `-32603` with a null id ([`http.rs:184-190`](https://github.com/qntx-labs/ra2a/blob/1a76d6e5af906737974fecfb4b5960fe8431dd9d/ra2a/src/server/http.rs#L184-L190), [`http.rs:297`](https://github.com/qntx-labs/ra2a/blob/1a76d6e5af906737974fecfb4b5960fe8431dd9d/ra2a/src/server/http.rs#L297)).
  - Push URLs get only an `http(s)` prefix check ([`push.rs:156-170`](https://github.com/qntx-labs/ra2a/blob/1a76d6e5af906737974fecfb4b5960fe8431dd9d/ra2a/src/server/push.rs#L156-L170)).

## Recommended execution order

1. Define the transport-free types: the agent card (Finding 1) and parts and artifacts (Finding 7), in the plugin API, with golden JSON tests.
2. Serve the card from the Gateway's open tier and wire capability gating into the core (Finding 1).
3. Add the task store, with state sets, terminal compare-and-set and atomic cancel (Finding 3), together with the per-task event log (Finding 2). They move the same files, so do them as one step.
4. Build the A2A request handler with thin JSON-RPC and REST adapters (Finding 5) and the id-resolution table (Finding 4). Then serve resubscribable SSE from the log (Finding 2).
5. Add per-peer identity and the OAuth challenge (Finding 9) before exposing any task endpoint beyond loopback.
6. Add push configs, the SSRF-safe sender and the delivery queue (Finding 8).
7. Build the outbound client tool pack and the reverse channel (Finding 10). This also unblocks `runvil/subagents` (D4).
8. Revisit sealed session ids (Finding 6) only if the Gateway ever runs as more than one instance.

## Refactor notes

- **Pure structure versus behavior.** Findings 1 and 7 (types) and the state-set extraction in Finding 3 are pure structure. Findings 2, 4, 5, 8, 9 and 10 change behavior and add surface.
- **Tests.** The existing Engine recorded-answer suites and the Gateway `tests/it` suites are the invariant, and they move last. New A2A behavior gets its own integration suite with fake peers.
- **Do not touch.** Leave the Engine's `Run::step` effect contract, the C plugin ABI and the OpenAI-compatible routes alone. A2A sits beside them.
- **Per step.** Verify and commit after each step. If one step fails twice in a row, stop the run and re-plan.

## Sources

- [a2aproject/a2a-rs](https://github.com/a2aproject/a2a-rs): `700a6a28eb001f6fc32bc9446078ff87bdd18c6c`, Apache-2.0, analyzed 2026-11-17. PRE_AI_SHA `18fadd4` (`handler.rs`) and `9f6ee72` (`task_store/inmemory.rs`).
- [agentgateway/agentgateway](https://github.com/agentgateway/agentgateway): `bdb36de0ccdb1d5954486423698f5231a176b157`, Apache-2.0, analyzed 2026-11-17. PRE_AI_SHA `38331f4c87` (`a2a/mod.rs`), `bdfadd6713` (`mcp/session.rs`), `d7604e9aa0` (`mcp/handler.rs`), `bcc01c9a9c` (`proxy/httpproxy.rs`), `00f96438c4` (`telemetry/log.rs`), `245d2ae69b` (`proxy/mod.rs`), `a0d5a75b96` (`mcp/auth.rs`) and `c845ecb381` (`store/mod.rs`).
- [bacnh85/a2a-switchboard](https://github.com/bacnh85/a2a-switchboard): `235dc934bf4967999819ad26c6190b8bdd6dc8db`, Apache-2.0, analyzed 2026-11-17.
- [aussierobots/turul-a2a](https://github.com/aussierobots/turul-a2a): `fa691278f4565bac9cc80529cffe25cdd5442192`, MIT OR Apache-2.0, analyzed 2026-11-17.
- [qntx-labs/ra2a](https://github.com/qntx-labs/ra2a): `1a76d6e5af906737974fecfb4b5960fe8431dd9d`, MIT OR Apache-2.0, analyzed 2026-11-17.
- Subject: [cppalliance/runvil-gateway](https://github.com/cppalliance/runvil-gateway) at `fe136b4`, [cppalliance/runvil-harness](https://github.com/cppalliance/runvil-harness) at `db4cf1f` and cppalliance/runvil-plugin-api at `8844a3e`. The subject profile and the field survey are both dated 2026-11-17.

*2026-11-17 12:40 - model unidentified*
