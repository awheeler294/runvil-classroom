# Item 5: static build checks, what I found so far

Class share, 2026-11-24. Full notes are in `item-5-plan.md`.

## The goal

Every Rust repo runs the same base set of checks, plus its own. The check code lives in one place, so a fix reaches every repo. If we write a rule down, a check enforces it, and an AI that breaks it gets a build failure.

## What I found

The clippy lint tables already match. Every repo denies the same lints. The two plugin repos are missing one small carve-out, and that's the only difference.

The structural checks are all over the place:

| Repo | Number of checks |
|---|---|
| `runvil-harness` | 14 |
| `runvil-workshop` | 8 |
| `runvil` | 7 |
| `runvil-gateway` | 6 |
| `runvil-plugin-api`, `-std`, `-bashkit` | 1 each |
| `runvil-cloud-providers` | 0 |

The copies have drifted. `type_size.rs` lives in five repos as four different files. They use the same limits, but the code differs.

Some written rules have no check behind them. The Harness `AGENTS.md` bans `thiserror` and inline `mod` blocks, but nothing enforces either, and `thiserror` is in use there today.

## The proposal

1. Put the shared checks in the repo that already holds our dylint lints, so every guardrail lives in one place. Each repo pins it to a commit, the way three repos already do for the lints. The lints and the checks go in separate folders, because the lints need a nightly compiler and the checks don't.
2. Write one page in that repo's README that lists every rule, which repos it covers, and which check enforces it.
3. Start adding new checks. First up is `clippy::redundant_clone`, which catches the "clone to dodge the borrow checker" pattern Vinnie mentioned last week.

## Questions for you

- Should new checks start as warnings or go straight to errors?
- Is there a rule you've seen an AI break that you'd want a check for?
