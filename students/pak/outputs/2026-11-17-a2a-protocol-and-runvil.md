# Defer A2A until a prompt must call an agent Runvil does not run

**Type.** Analytical recommendation, feasibility variant. The call is **modify**.

**Date.** 2026-11-17.

**Audience.** Someone deciding whether the Runvil engine should adopt the Agent2Agent protocol (A2A).

**Decision this serves.** Whether A2A is relevant to Runvil, whether to adopt it, and how it relates to sub-agents in the engine.

## Summary: wait, then add a Pack if a prompt names a remote agent

Leave A2A out of the Engine and out of the reserved Pack `runvil/subagents`. The protocol is relevant only when a Runvil prompt must call, or must be called by, an agent whose process Runvil does not run. No decision page states that need. The call is modify: do not build A2A now, and if a later prompt names such an agent, add a Pack for outbound calls and a separate Host for inbound calls.

A2A, at the specification page fetched on 2026-11-17, is an open protocol for independent agents that do not share memory, tools, or plans. Runvil's shipped sub-agents are the opposite arrangement. `tasks.spawn` and `tasks.fanout` start chains inside the same run, on one thread, under one set of limits. The Pack `runvil/subagents` offers no tools. Its description says it reserves the name for Engine subagent tools that are not built yet. Those planned tools start the same kind of chain. They are not a wire protocol.

Confidence in that split is high. The specification, `registry.rs`, and the tasks decision page all state it. The planned-work list was read, and A2A is absent from it. Confidence that no prompt outside those pages needs a remote agent is low, because this check did not read the prompt corpus. On these sources alone, a product prompt is very unlikely to need A2A before replay and the subagent tools are built. Confidence in that forecast is low.

The do-nothing option is the right one until a named agent exists. Implementing `runvil/subagents` as an A2A client was set aside. That Pack is an Engine built-in, and an Engine crate may not depend on an HTTP client. Treating every remote agent as an MCP tool was also set aside for the case where the remote side keeps its own task life cycle. MCP remains the right path when the remote side is a tool.

**Next step.** Leave A2A off the planned-work list in the tasks decision page. The person who answers that page's open questions owns the decision. No name for that person appears in the sources read here.

## Contents

1. The call is modify
2. Five tests, scored in two places
3. A2A connects agents that stay opaque to each other
4. A Runvil sub-agent is a chain in the same run
5. `runvil/subagents` reserves those chains for the model
6. Four options, including doing nothing
7. A Pack is the adoption path, later
8. What would change the call
9. Limits of the check
10. References

## 1. The call is modify

Do not add A2A to the Engine. Do not implement it behind `runvil/subagents`. Keep the current design: Lua starts in-process tasks, MCP reaches remote tools, and the Gateway reaches models.

A2A becomes relevant on the day a prompt must delegate work to an agent Runvil does not execute, or the day an outside client must start a Runvil prompt through an A2A endpoint. Until that day, building the protocol adds a client, a version split, and a second meaning of "task" beside machinery the decision pages already list as unbuilt. Replay is unbuilt. Model-facing task tools are unbuilt.

This was checked by reading the A2A specification page and the GitHub release v1.0.1 on 2026-11-17, and by reading the Runvil decision pages, `registry.rs`, and two earlier research notes that already mention A2A. Vendor deployment claims gathered in a separate pass were not re-fetched. They are omitted where they would carry the conclusion. Section 9 lists the gaps.

## 2. Five tests, scored in two places

The feasibility tests are technical fit, cost, license, operations, and schedule. Table 2 scores each one twice: once if A2A were built into the Engine or into `runvil/subagents`, and once if outbound A2A were a Pack and inbound A2A were a Host. Cost and license are not scored. No cost figure appears in the sources, and the A2A license text was not re-read in this pass. A separate reading of the repository README reported Apache-2.0. That report is not treated as verified here.

Table 1 places the three mechanisms next to each other. The scores in Table 2 follow from that comparison and from the sources in sections 3 to 5.

**Table 1. Runvil tasks, the reserved subagents Pack, and an A2A peer.**

| Mechanism | Who starts the work | Where the work runs | What it shares with the caller |
| --- | --- | --- | --- |
| Runvil task (`tasks.spawn`, `tasks.fanout`) | Lua in the prompt | Another chain in the same run, on one thread | Scheduler, limits, files, Packs, and the run log |
| `runvil/subagents`, once its tools exist | The model, through Engine tools | The same kind of chain | The same run |
| A2A peer | A client that has an Agent Card | A separate server | Declared skills and exchanged messages. The specification says the peer does not share internal state, memory, or tools |

**Table 2. Feasibility of A2A in two places. Cost and license are unscored.**

| Test | Inside the Engine or `runvil/subagents` | Outbound as a Pack, inbound as a Host |
| --- | --- | --- |
| Technical | Fails. Outside work belongs in a Pack, and an Engine crate may not depend on an HTTP client | Fits the existing tool effect and the existing Host boundary |
| Cost | Not scored. No figure in the sources | Not scored. Same gap |
| License | Not scored. License text not re-read | Not scored. Same gap |
| Operations | A second task model beside the scheduler, with its own auth and discovery | The Host already performs HTTP for MCP and for the Gateway. Agent Card trust and out-of-band credentials would be new |
| Schedule | Would land before replay and the model task tools, both listed as unbuilt | Waits until a prompt names a remote agent |

The in-engine column fails technical fit and schedule. The Pack column passes technical fit and can wait. Together those scores are a modify call. A go would mean building A2A on the current plan, and the sources name no user for that work. An abandon would mean refusing the protocol even after a prompt needs a remote agent. The sources do not support that refusal.

## 3. A2A connects agents that stay opaque to each other

The specification's opening states the problem in its own terms. "The Agent2Agent (A2A) Protocol is an open standard designed to facilitate communication and interoperability between independent, potentially opaque AI agent systems." One of the four goals is: "Securely exchange information to achieve user goals without needing access to each other's internal state, memory, or tools." A guiding principle, opaque execution, says: "Agents collaborate based on declared capabilities and exchanged information, without needing to share their internal thoughts, plans, or tool implementations." [1]

The same page's Appendix B separates A2A from the Model Context Protocol. Of MCP it says: "Focuses on standardizing how AI models and agents connect to and interact with tools, APIs, data sources, and other external resources." Of A2A it says: "Focuses on standardizing how independent, often opaque, AI agents communicate and collaborate with each other as peers." The appendix's picture of both in one system is an A2A client asking an A2A server to do a task, and that server using MCP for its own tools. [1]

The objects that matter for a Runvil comparison are the ones the terminology section defines. An Agent Card is "A JSON metadata document published by an A2A Server, describing its identity, capabilities, skills, service endpoint, and authentication requirements." A Task is "The fundamental unit of work managed by A2A, identified by a unique ID." A Message is a turn with a role and one or more Parts. An Artifact is an output of a task. [1] "A2A Servers MUST make an Agent Card available." Clients may find it at `/.well-known/agent-card.json`, in a registry, or by direct configuration. Section 8.2 names registries and does not, in that section, define a registry API. [1]

The page banners "Latest Released Version `1.0.0`" and lists 0.3.0 among previous versions. [1] GitHub's v1.0.1 release was published 2026-05-28, with notes dated 2026-05-26, and those notes are bug fixes. [2] Section 3.6 says negotiated versions use major.minor only, so the protocol version under either label is 1.0. Section 3.6.2 also says agents "MUST interpret empty value as 0.3 version." A client can send `A2A-Version: 1.0`. A server that rejects a missing header would be departing from that MUST.

Send Message disagrees with itself on the same page. Section 3.1.1 says the operation "MUST return immediately with either task information or response message." Section 3.3.3 says operations "return immediately." Section 3.2.2 says the default is to wait. In that case the operation "MUST wait until the task reaches a terminal (`COMPLETED`, `FAILED`, `CANCELED`, `REJECTED`) or interrupted (`INPUT_REQUIRED`, `AUTH_REQUIRED`) state before returning." [1] An implementer has to choose. This check did not re-read `specification/a2a.proto`. The specification calls that file the normative definition of the data objects. The prose conflict is enough to treat the blocking rule as unsettled until the proto is compared.

Bindings are three: JSON-RPC, gRPC, and HTTP+JSON. [1] The harness decision page records the owner's rule that "the engine, the harness would not know anything about streaming. only the host, through the inference broker." [3] That rule is about model tokens. It still says where a long A2A task would wait. The Host would hold the socket, then return one finished answer to the run.

## 4. A Runvil sub-agent is a chain in the same run

Runvil's public description is a prompting language: one Markdown file, prose for the model, Lua for the control flow. [4] The tasks decision page defines the execution model that sub-agents use. "A chain is one walk through a prompt's sections. A run has a chain for the main walk, one for each `call`, one for each `execute`, and one for each task, and a task is a chain that runs beside the chain that spawned it. The Engine's scheduler runs the chains in turns on one thread." [5]

`tasks.spawn` queues that chain and returns a handle, `{ task = id }`, before the chain runs. `tasks.fanout` is Lua that spawns one task per member of a table and joins the results in order. The chain that spawns a task owns it. Only the owner may wait, send, or cancel. `RunControl` "sends only to the main chain: a sub-agent is a task in the same run, and only a run has a `RunControl`." [5]

"The executed prompt is a part of the caller's run and not a run of its own." It shares the caller's scheduler, cancel flag, event log, files, Packs, and limits. The recorded reason is the owner's sentence: "you could have a sub-agent in the main prompt, and then you can have a sub-agent in the child prompt, but they all contribute to the same set of limits." [6]

The Engine never performs the HTTP itself. `Run::step` returns effects. `Run::resume` takes the answers. "`step` and `resume` are synchronous functions, and a structural check fails when an Engine crate depends on an async runtime or an HTTP client." The Engine "is a deterministic function of its inputs." [7] Effects include `Chat`, `Render`, `ToolCall`, `Vfs`, and `Timer`. A remote agent can enter this loop as a `ToolCall` answer. It cannot enter as a crate the Engine links against.

The public feature list is ahead of that code. It says `tasks.fanout` runs a section once per item, and then, "A model can start sub-agents of its own." [4] The tasks page's planned-work list says the opposite about the model: "only Lua starts a task, with `tasks.spawn` or `tasks.fanout`." [5] The fan-out sentence matches the decision page. The model-starts-sub-agents sentence does not match the page or the Pack stub in section 5.

## 5. `runvil/subagents` reserves those chains for the model

`registry.rs` builds two built-in Packs into every registry. One is `runvil/subagents`. The description string is "Reserves the subagents name for the Engine's subagent tools, which are not built yet." The type's `tools` method returns an empty vector. A comment on the built-ins says: "A built-in Pack's behavior and tools live in the Engine." The same comment says "a built-in must never offer tools." [8]

The planned-work entry ties that name to in-process tasks: "Tools with which a model starts tasks itself, as the tools of the Pack `runvil/subagents`." [5] The entry does not mention a socket or a remote server. By the tools page, outside work belongs in a Pack. "A Pack provides a feature that works on something outside the run, such as the web or an MCP server." A Pack for budget or model routing would duplicate work the Engine, the Lua, the Gateway, or the Harness already owns. [9] A remote A2A agent meets that test, as an MCP server does, so it belongs in a Pack. The factory "owns the state that runs share, such as MCP connections and HTTP clients." [9] That is the existing place for an A2A HTTP client.

Runvil research already drew this line, before the November decision pages. The 10 October 2026 tool catalog says "A2A (Agent2Agent) is a wire protocol between agents, not a set of model-facing tools." The next sentence says "The model does not see these method names; a harness that uses either protocol maps them to its own model-facing tools." [10] A 12 September 2026 survey of another product's features mapped outbound A2A to "an HTTP client tool set; auth via host-seeded secrets," and mapped an inbound A2A channel to a Host that "constructs runs from external events." [11] Those two labels still match sections 3 and 4. The same September note treats background sub-agents as a deferred task runtime. That part is stale. The November tasks page documents `tasks.spawn` as the current design, checked against the code. [5]

A Runvil sub-agent is a chain the scheduler runs inside one recorded run. An A2A agent is a remote server described by an Agent Card. The reserved Pack is the future model-facing handle on the chain. The protocol is a separate mechanism. A harness that someday speaks A2A would map `SendMessage` and `GetTask` onto its own Lua-facing tool, which is the mapping the October catalog already states. [10]

Trust handling would follow the tool rule. Pack output that an attacker can influence is marked untrusted, and the Engine wraps it. The docs call that wrap "defense in depth, not a security boundary." Operator text from `tasks.receive()` stays plain, so the script can see the operator's words. [9] Text from a remote agent is closer to MCP server text than to the operator. It would be marked untrusted.

## 6. Four options, including doing nothing

**Do nothing.** Lua keeps starting tasks. MCP keeps reaching remote tools. The Gateway keeps reaching models. This spends no time on a protocol the planned-work list does not name. The loss is that Runvil cannot call a remote agent, or expose a prompt as one, until the work is scheduled. No source read here names an agent that is blocked on that gap. This is the recommended option for now.

**Build A2A as `runvil/subagents`.** The model would gain tools that call remote agents. This was set aside. The Pack's comment says the tools are the Engine's, and the Engine is forbidden an HTTP client. [7] [8] Using that name would also merge two designs the October catalog separated: model-facing task tools, and a wire protocol the model does not see. [10] A variant that emits a new effect from the Engine, and lets the Harness perform the HTTP, avoids the link-time ban. The remote agent is still outside the run, which the tools page assigns to a Pack, and the variant still uses the subagents name for a different mechanism. [9]

**Call remote agents only through MCP.** This is already the documented tool path, and it remains right when the other side is a tool. [9] It was set aside as a complete answer. Appendix B of the A2A specification describes a peer that has its own task, its own tools, and no shared memory. [1] Wrapping that peer as one MCP tool drops the task states the specification spends its pages on, including `TASK_STATE_INPUT_REQUIRED` and `TASK_STATE_AUTH_REQUIRED`. [1] Those states are the part a long remote job needs. They can be mapped later onto a run that is waiting. They are a poor fit for a single MCP call that returns one text blob and ends.

**Add a Pack later, and a Host only if inbound is required.** Outbound calls are a Pack tool. Inbound calls are a Host that starts one run from one A2A task, on the pattern of Workshop starting a run from operator input. This is the option that passes Table 2. It is not work to start this week. Section 7 states it for a later decision.

## 7. A Pack is the adoption path, later

Start this only when a prompt names the remote agent. Until then, section 6's first option stands.

1. Leave the Engine effect list unchanged. Leave `runvil/subagents` as the reservation for in-process model tools.

2. Add a different Pack. Its factory holds the HTTP client, as the tools page already says factories hold MCP connections and HTTP clients. [9]

3. Give Lua a tool. Keep `SendMessage` off the model's tool list. The October catalog's rule is that the harness maps protocol methods onto its own tools. [10]

4. Send `A2A-Version: 1.0`. Implement the JSON-RPC binding first. The card lists interfaces, and the client uses one it implements. [1] Add gRPC or HTTP+JSON when a named peer offers nothing else.

5. Treat one tool call as one recorded effect answer. Code the wait against section 3.2.2's blocking default, and test that choice against `a2a.proto` before shipping, because sections 3.1.1 and 3.3.3 contradict it. [1] If the remote task outlives a single wait, the Host polls or reads the stream and resumes the run with the finished task state. The Engine stays free of that socket, consistent with the streaming rule for model tokens. [3]

6. Store the request and the returned task as the effect answer. The replay driver is still unbuilt. [5] A recorded answer is what would let that driver avoid calling the network again. The specification's prose does not promise that the server will perform the work once.

7. Mark the returned text untrusted. [9] Keep Agent Card URLs in Host configuration, the way MCP servers are configured. Prefer direct configuration over fetching `/.well-known/agent-card.json` from a model-supplied host. The specification allows direct configuration. [1]

8. Build an inbound Host only after the outbound Pack has a user. One A2A task starts one run. `TASK_STATE_INPUT_REQUIRED` becomes a wait on that run's inbox. The run's result becomes the task's artifact. This Host is a process that constructs runs. The scheduler stays as it is.

Step 7 is the one that most affects security. This check did not audit the specification's security section. A pinned card and an untrusted envelope match rules Runvil already applies to MCP. They do not show that fetching a card from an unknown host is safe.

## 8. What would change the call

Build the Pack when a prompt, a partner, or a Host feature names an agent Runvil does not run, and MCP is the wrong shape because the peer keeps its own task state. That is a go for section 7, still a modify for the Engine.

Build nothing, and close the option, if the only agents Runvil will call are its own prompts. `execute` and `tasks.fanout` already cover that, and they keep one log and one limit budget. [5] [6] The decision pages describe chains, Packs, MCP, and the Gateway. Confidence that those pages are the whole product is low. The prompt corpus was not read.

Reconsider the wire protocol itself if a peer that Runvil must reach speaks a different agent protocol and does not offer A2A. Nothing read here names such a peer.

## 9. Limits of the check

The specification page and the v1.0.1 release notes were fetched on 2026-11-17. [1] [2] The Runvil files in the reference list were read in the working tree the same day. A September integration survey and an October tool catalog were read at the cited lines. [10] [11]

Not re-read, and not used to carry the call: the normative proto file, the A2A license text, vendor documents that pin protocol 0.3, download counts, and papers about Agent Card attacks. A separate reading reported that several vendor documents still show 0.3, and that the Python, Go, Java, and JavaScript SDKs had releases in 2026. Those claims may be true. They are not confirmed here. The 0.3 fact this report does use is the specification's own rule that an empty version means 0.3. [1]

The tasks decision page says it was checked against `runvil` at commit `fa9936f64`, with later path updates. [5] This report did not re-run that check against a newer commit, except for the Pack stub in `registry.rs`, which was read directly. [8] If the stub's description has changed since the read, the "not built yet" claim needs another look.

The public page `why-runvil.md` is dated 2026-11-11 in its footer and states a model-started sub-agent feature the decision page still lists as planned. [4] [5] That gap is a documentation bug. It is independent of A2A, and it should not be read as evidence that remote agents already exist.

## 10. References

1. Agent2Agent (A2A) Protocol Specification. https://a2a-protocol.org/latest/specification/ Sections 1, 1.2, 3.1.1, 3.2.2, 3.3.3, 3.6, 3.6.2, 4 terminology as rendered in section 2.2, 8.1, 8.2, and Appendix B. Fetched 2026-11-17. The page prints no publication date. Banner text: latest released version 1.0.0.
2. GitHub release v1.0.1, repository `a2aproject/A2A`. https://github.com/a2aproject/A2A/releases/tag/v1.0.1 Published 2026-05-28T11:34:36Z. Notes heading dated 2026-05-26. Fetched 2026-11-17.
3. `runvil-harness/decisions/harness-and-host.md`, streaming rule. Working tree, read 2026-11-17.
4. `why-runvil.md` in a retired repository, executive summary and feature 6. Footer dated 2026-11-11. Working tree, read 2026-11-17.
5. `decisions/tasks-and-fanout.md` in a retired repository, summary, scheduler, `RunControl`, and planned work. Working tree, read 2026-11-17. The page names code commits `fa9936f64` and later path commits.
6. `runvil-harness/decisions/prompt-language.md`, the `execute` entry. Working tree, read 2026-11-17.
7. `runvil-harness/decisions/engine.md`, effects and the HTTP-client check. Working tree, read 2026-11-17.
8. `runvil-harness/crates/runvil-plugin/src/registry.rs`, `BUILTIN` and `Builtin::tools`. Working tree, read 2026-11-17.
9. `runvil-harness/decisions/tools-and-packs.md`, the Pack test, untrusted output, and factory-owned HTTP clients. Working tree, read 2026-11-17.
10. `research/2026-10-10-subagent-tool-catalog.md` in a retired repository, "Protocols define methods, not model-facing tools." Working tree, read 2026-11-17. The note pins A2A documents at commit `12e9d2fbb9badfe98f8ab4f660697870eeb1940b`, which it dates 2026-10-07. That pin was not re-fetched.
11. `research/2026-09/2026-09-12-everruns-integration-paths.md` in a retired repository, items 68 and 85. Working tree, read 2026-11-17. This is a mapping survey, not a decision page. Its claim that background sub-agents are still deferred conflicts with reference 5 and is treated as stale in section 5.

---

*2026-11-17. Grok 4.7, in Cursor. Written to the report rulebook in `how-to-write-reports.md` (the document titled "Report Rulebook"). Spec and release pages were fetched the same day. Claims that failed that check are marked in section 9.*
