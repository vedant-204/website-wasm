# Agent automation

Two scheduled agents, deliberately split by what each environment can do.

## Task A — implement (runs on this Mac, via launchd)

Needs `cargo`, the `wasm32-unknown-unknown` target and an authenticated `gh`,
so it must run here — the cloud sandboxes can reach GitHub but cannot install
the Rust wasm toolchain, and an agent that opens PRs it never compiled is worse
than no agent.

Install:

```bash
cp .agent/dev.vedant.agent-implement.plist ~/Library/LaunchAgents/
launchctl load ~/Library/LaunchAgents/dev.vedant.agent-implement.plist

# make sure the Mac is awake for it
sudo pmset repeat wakeorpoweron MTWRFSU 09:25:00
```

Test it without waiting for the schedule:

```bash
./.agent/run.sh                                    # full run
launchctl start dev.vedant.agent-implement         # exactly as launchd will
```

Remove: `launchctl unload ~/Library/LaunchAgents/dev.vedant.agent-implement.plist`

Note: launchd will not fire with the lid shut on battery power.

**Why this repo does not live in ~/Desktop.** macOS TCC protects ~/Desktop,
~/Documents and ~/Downloads. Your Terminal has permission to read them; a bare
launchd agent does not, so the job fails with `can't open input file` before it
ever reaches Claude. Keeping the repo at ~/DEV-2 avoids the whole problem. Do
not move it back under one of those folders.

**The working tree must be clean at 09:30.** The agent refuses to run against a
dirty tree rather than stashing your work. Uncommitted changes at the scheduled
time mean a skipped run, not a lost one.

## Task B — deep dive (runs as a Claude scheduled task)

Reads this repo and raises issues over the GitHub API. It never writes code and
never opens a PR, so it does not need a toolchain.

It needs a fine-grained personal access token at `.agent/gh-token`
(gitignored), scoped to this repo with read/write on Issues and read on
Contents:

```bash
echo "github_pat_xxx" > .agent/gh-token && chmod 600 .agent/gh-token
```

## The label gate

```
Task B raises issue          → agent-review [+ human-review-required]
Vedant triages and approves  → ready-for-agent
Task A implements it         → agent-implemented + draft PR
Vedant reviews and merges
```

Neither agent may ever apply `ready-for-agent`. That label is the human gate,
and it is the only thing standing between a proposal and code being written.
