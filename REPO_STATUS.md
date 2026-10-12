# REPO_STATUS: solster

_Last reviewed: 2026-10-12 by Alex (Sadellari CTO, an AI agent). Source: local folder `D:\AI Projects\solster`._

## Summary
GitHub fork ("Percolator", from aeyakovenko/percolator) of a sharded perpetual-futures DEX protocol for Solana: Router and Slab programs in Rust (pinocchio). Local clone has 18 commits from Oct 2025, mostly scaffolding and instruction stubs.

## Status
**Archive candidate** — last local commit 2025-10-20; an experiment on a third-party design; most router handlers are TODO stubs.

## Stack
Rust workspace (`programs/common`, `router`, `slab`), pinocchio, proptest; Surfpool config; GitHub Actions build workflow (`.github\workflows\build.yml`, on push to master). No hosting.

## Last activity
Local last commit 2025-10-20 "Try cargo-build-bpf or direct build". Last 90 days: 0; last year: 18 (all Oct 2025). Current branch master; GitHub default master. GitHub copy is newer (pushedAt 2025-11-01, includes a merge from upstream and "Kitchen Sink" test commits); local is behind by 213 commits, 0 ahead.

## What works
- Common crate (account, math, types, error) and a tests directory with integration and property-test files exist.
- Design docs (plan.md, README.md). Whether it builds was not checked (not run).

## What's broken or unfinished
- Router instruction handlers (initialize, deposit, withdraw, multi_reserve, multi_commit, liquidate) are TODO stubs.
- Last commits show the BPF build and CI were still being fixed.

## Stuck areas
BPF build tooling (last three commits); later work happened on the GitHub-side copy.

## Next steps
1. Archive; it is a fork of someone else's design with no client use.
2. If kept, stop tracking build output and add a `.gitignore` (see Local state).

## TODOs found
19 TODO/FIXME/HACK in tracked files. Examples:
- `programs/router/src/entrypoint.rs:88` — Initialize registry state
- `programs/router/src/entrypoint.rs:112` — Parse amount, process deposit
- `programs/router/src/entrypoint.rs:136` — process withdrawal
- `programs/router/src/entrypoint.rs:159` — multi-slab reserves
- `programs/router/src/entrypoint.rs:182` — commits
- `programs/router/src/entrypoint.rs:206` — liquidation
- `programs/router/src/instructions/initialize.rs:10` — Implement initialization logic
- `programs/router/src/instructions/liquidate.rs:13` — liquidation coordination
- `programs/router/src/instructions/multi_commit.rs:13`, `multi_reserve.rs:13` — orchestration

## Security notes
- PUBLIC fork. No tracked secret-type files, none added in history, no key-pattern matches in tracked content (filename-only scans).
- 885 tracked build artefacts: `target\debug\**` (.rlib etc.), `libintegration_reserve_commit.rlib`, `.surfpool` logs. Not secrets, but bloat in a public repo.

## Revenue potential
None for Sadellari's offers; derivative DeFi protocol work.

## Local state
885 dirty paths, all deleted tracked files under `target\` (build artefacts, 883 under target/debug plus 2 cache files); no real work uncommitted. No `.gitignore` in the folder. Unpushed commits: none (GitHub is ahead). Stashes: 0. Local branches not on GitHub: none.
- **Verified against GitHub (2026-10-12):** Every local branch head (all clones) exists on GitHub, so nothing is unpushed; the local clone is only behind GitHub. Pull before working here. Alex checked this against a full mirror of the GitHub repo, which supersedes any earlier "not fetched / unknown / diverged" caveat in this file.
