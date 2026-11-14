# Runvil Vision

## Vision

Prompts should be real programs: readable by people, predictable to run, and open to everyone. Runvil is an open-source prompting language where a prompt is one Markdown file. The prose is for the model, the Lua is the logic, and the Markdown is the program while the model is the CPU.

We want building with models to feel like engineering again. You can read what a prompt does, you know what will happen next, and every run leaves a record you can look back on.

## Why Runvil exists

Prompting started as pasting paragraphs into a chat box. As models take on real work, teams chain prompts together, wrap them in scripts, and hope the model follows every rule in the paragraph. It often does not. The fix is to stop asking the model to do what code can do.

Runvil gives each job to the part that is good at it. The model handles judgment and language. Lua handles everything that needs no judgment: branching, looping, counting, formatting, and deciding when a section is finished. The prose stays readable on its face, and the logic stays in code where it can be checked.

Underneath that is a belief about people. Human judgment is the scarce resource, and model output is abundant. Models should advise and compare, and humans should decide. Runvil exists to keep the human's judgment in the loop and on the record.

## What a prompt looks like

A prompt is one Markdown file with a small header, prose, and Lua blocks side by side. This is one of the sample prompts in the repository:

````markdown
---
name: greet
description: Greet a name using a Lua-computed value
runvil: 0
args:
  type: object
  properties:
    name: { type: string, description: The name to greet }
  required: [name]
---

Computes a greeting for its `name` argument in Lua, substitutes it into the prose, and has the model echo it.

## Main

```lua
var.greeting = "Hello, " .. args.name .. "!"
```

Repeat exactly, with no extra words: {{ var.greeting }}

```lua
return models.infer(prose)
```
````

The header declares the one argument the prompt takes. The first Lua block computes a value from it. The placeholder puts it into the prose. The last block sends that prose to the model. Nothing reaches a model until Lua sends it.

## Principles

- **Readable on its face.** A person can open a prompt and see what it does, in order, without a tool.
- **Deterministic by default.** The language itself performs no I/O and reads no clock. Anything that touches the outside world goes through the Harness, so the same prompt with the same inputs gives the same run.
- **The model is a role, not a vendor.** A prompt asks for one of five roles: `default`, `strong`, `small`, `fast`, or `creative`. The application running the prompt decides which model fills each role, so changing providers never means rewriting a prompt.
- **Keys live in one place.** The Gateway is the one process that holds model credentials. No other process sees a vendor key. It serves an OpenAI-compatible API and routes each request to a cloud provider or to a model on your own machine.
- **Every run is recorded.** The Harness records what happened, and the log only grows.
- **Small pieces, easy to review.** The Engine parses and steps a run. The Harness performs every effect and records it. A Host is any application that runs prompts through the Harness. We prefer more, smaller crates so a reviewer can hold one in their head.
- **Open at the edges.** Tools arrive through Packs and Plugins, through MCP servers, and through a plugin API with a C header and a safe Rust SDK.

## Open source

Runvil is open source under the Boost Software License 1.0. The code lives in the `cppalliance` organization on GitHub, the documentation is at [runvil.org](https://runvil.org), and nightly builds are published as a prerelease. Pull requests are welcome, and every change has to pass the formatting, lint, and test checks that CI runs.

## Where we are today

Runvil is young. The prompt format is at version 0, and we change it freely while we learn what it needs.

- **Working today:** prompts run end to end with sandboxed Lua, model roles, and tool loops. Agents can search and read the web, ask the user questions, and use remote MCP servers. Every run is recorded, and Workshop is the main Host built on it.
- **Not built yet:** file-writing tools for agents are still stubs, subagents do not exist yet, local MCP servers are not supported, and the Plugin API can read a run's files and cannot write or delete them yet. Replaying a recorded run has no user-facing driver.

## Where we are going

A Host is any program that embeds the Harness, so the same prompts should run in an IDE, a command line tool, a server, or an application someone builds next year. The interviewers, tutors, and analysts our team writes today are the kind of prompts we want to run on it, and the knowledge capture products we are planning are built to run on Runvil.

The goal is simple. Anyone who can write clearly and think in steps should be able to build a dependable agent, read how it works, and trust what it did.
