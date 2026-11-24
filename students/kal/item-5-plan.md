# Item 5 plan: static build checks and guardrails

Draft, 2026-11-18. A starting point for the meeting with Vinnie, not a finished plan.

## Goal

Every Rust repository runs the same base set of checks, plus the checks that belong to it alone. The check code lives in one place, so a fix reaches every repository. Each repository's rules are written down, and a check backs every rule we write down. When an AI breaks a rule, it gets a build or test failure.

## What I found

I checked the brief against `master` of each repository on 2026-11-18. Most of it holds up.

**Lint tables are already nearly uniform.** Every repository has the same `[workspace.lints]` table: `clippy::all` and `pedantic` denied, `unwrap_used`, `expect_used`, `allow_attributes_without_reason` and the rest. The one difference is that `runvil-plugin-api` and `runvil-plugin-std` lack the `assert_is_empty = "allow"` carve-out. The divergence is in the structural checks, not clippy.

**Structural checks differ a lot.** Each repository runs its checks as tests (`cargo test` or `cargo nextest`). `runvil-harness` and `runvil-gateway` also run `cargo xtask tidy` in CI for a full report.

| Repository | Check crate | Checks |
|---|---|---|
| `runvil-harness` | `build-xtask` | api, debug_spelling, doc_hidden, engine_clippy, engine_deps, engine_guards, facade_shape, harness_bans, manifest, no_doctests, product, test_support_leak, type_size, unsafe_allowlist |
| `runvil` | `build-xtask` | debug_spelling, manifest, no_doctests, product, site, type_size, unsafe_allowlist |
| `runvil-workshop` | `build-xtask` | debug_spelling, harness_pin, manifest, new_crate, no_doctests, product, type_size, unsafe_allowlist |
| `runvil-gateway` | `build-xtask` | debug_spelling, manifest, no_doctests, unsafe_allowlist, workspace, 500-line file ceiling |
| `runvil-plugin-api` | `build-xtask` | header |
| `runvil-plugin-std` | `build-checks` | type_size |
| `runvil-plugin-bashkit` | `build-checks` | type_size |
| `runvil-cloud-providers` | none | fmt, clippy, test only |

**Copies have drifted.** `type_size.rs` exists in five repositories as four different files (694 to 758 lines). The limits agree (12 fields, 25 methods) but the code doesn't. The two `build-checks` crates are byte-identical, as the brief says.

**dylint is already in place in three repositories.** `runvil`, `runvil-harness` and `runvil-workshop` run `cargo dylint` in CI against [cppalliance/dylints](https://github.com/cppalliance/dylints), pinned to the same commit. That library has one lint so far, `io_try_without_context`. The other five repositories don't run it. The library is Runvil-specific: its README is titled `runvil-lints` and points to `cppalliance/runvil-lints`. It is one crate, `build-lints`, and its root `rust-toolchain.toml` pins `nightly-2026-08-20`.

**Some rules have no check.**
- The Harness `AGENTS.md` bans `thiserror` in Engine and Harness crates, but `thiserror = "2"` is a workspace dependency there and `plugin-web` uses it. I need to check whether that use is an allowed exception.
- The Harness `AGENTS.md` bans inline `mod` blocks and `#[path]` in production code. I found no check for either.
- The Gateway enforces a 500-line file limit, which Vinnie dropped from the Harness in favor of the type size limit.

## Open questions

### 1. One shared crate, or keep the copies?

Options:
- **a.** Keep copying, add a CI job that diffs the copies. Cheap, still drifts in intent.
- **b.** One shared crate in its own repository (or inside `dylints`), pulled in as a git dependency pinned to a commit, like dylint does today. Each repository keeps a thin `build-xtask` that picks which checks to run.
- **c.** Turn the text checks into dylint lints. Works for code-shape rules, poor fit for manifest and file-layout checks.

Leaning toward **b** for file and manifest checks, and **c** for anything that needs types or MIR. The pinned-commit pattern already works for dylint, so the team knows it.

The shared crate should live in the existing lints repository rather than a new one. That gives one home for every guardrail, and each repository pins both the lints and the checks to the same commit. The catch is the toolchain. The lints need the pinned nightly, and the checks build on each repository's stable compiler. If the checks sat under the root `rust-toolchain.toml`, the lints repository's CI would test them on a compiler nobody else uses. So split the repository into two folders, each its own workspace with its own toolchain:

```
runvil-lints/
  lints/     the dylint crate, nightly rust-toolchain.toml
  checks/    the shared xtask checks, stable
  README.md  every rule, and which check enforces it
```

Moving the lint crate into `lints/` means the three repositories that use it must update their `[workspace.metadata.dylint]` entry to point at the subfolder. I believe dylint's `pattern` field does this. Verify before relying on it.

What goes in `checks/`: plain functions that take the repository root and return violations, like `unsafe_allowlist_violations(root)` in the Harness today. Two things make the move more than a copy. Some checks hardcode repository-specific settings (the Harness's `OWNED` crates in `unsafe_allowlist`), so each repository must pass its own. And the checks call workspace-walking helpers (`product::workspace_crates`, `tidy::rust_files`) that must move with them.

### 2. Which rules apply everywhere?

First guess at the shared base: the lint table, `type_size`, `unsafe_allowlist`, `no_doctests`, `debug_spelling`, `manifest` (lint inheritance), and the dylint library. Repository-specific: `product`, `facade_shape`, `engine_*`, `harness_bans`, `harness_pin`, `site`, `header`.

Need Vinnie's call on the Gateway's 500-line limit: drop it, or keep it as a Gateway-only rule?

The lint table can't be shared through a crate. Cargo has no way to inherit `[workspace.lints]` from another repository, so each repository keeps its own copy. A shared check can compare a repository's table against the standard one and fail on a difference.

### 3. Which new checks first?

Candidates, cheapest first:
1. Turn on `clippy::redundant_clone` as `warn` in one workspace and look at the noise. Justin's idea, from Vinnie's [gist](https://gist.github.com/vinniefalco/7db28479281e9aa6222d03bd8f69a4fe). One line.
2. Text checks for slop inventory rows from 2026-11-11: ticket ids in comments (row 7), `#[non_exhaustive]` on structs with only private fields (row 17), `#[serde(default)]` on `Option` fields (row 17).
3. Checks for the unbacked `AGENTS.md` rules: inline `mod`, `#[path]`, `thiserror`.
4. A dylint lint for a hand-written `Debug` that redacts a type that already redacts itself (row 15). Harder, needs type info.
5. Mateo's dylint proof of concept for "pub struct with one pub built-in field". Ask Mateo where it lives and whether it goes into `dylints`.

### 4. Documentation

One page that lists every rule, which repositories it applies to, and which check enforces it. It lives in the lints repository's README, next to the code that enforces each rule. Each repository's `AGENTS.md` links to it and lists only that repository's own rules.

## Order of work

1. Get answers on questions 1 and 2 from Vinnie.
2. Write the rules page from what exists today, marking each rule as checked or unchecked.
3. Try `redundant_clone` in one workspace.
4. Split the lints repository into `lints/` and `checks/`, and update the three repositories that pin it.
5. Move `type_size` into `checks/` first, since it has the most copies.
6. Bring the missing repositories up to the shared base, starting with the two plugin repositories.
7. Add new checks one at a time, each with a test.

## Questions for Vinnie

- OK to put the shared checks in the lints repository and split it into `lints/` and `checks/`?
- The repository is `dylints` on GitHub but `runvil-lints` in its README. Once it holds more than lints, rename it, for example to `runvil-guardrails`?
- Gateway's 500-line limit: keep or drop?
- Does `runvil-cloud-providers` count? It publishes a JSON model sheet, so it may not need the full set.
- Is the `thiserror` use in Harness `plugin-web` allowed?
- Warn or deny for new checks while they settle?
