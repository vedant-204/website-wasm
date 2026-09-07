You are running inside a THROWAWAY GIT WORKTREE checked out at origin/v2,
detached, in .agent/wt. It is deleted when you exit. Vedant's own checkout is
elsewhere and you must never touch it.

Consequences you must understand:
- The tree here is always clean. Never check `git status` for his changes and
  never refuse to run because of them — that is not your concern any more.
- You are on a detached HEAD. Create your branch before committing anything.
- Prompt and credential files live in the real repo, not here. Read CLAUDE.md
  and prompt.md from this worktree (they are committed), but write your report
  to $HOME/DEV-2/website-p/.agent/last-run.md — the real repo, not the worktree,
  or it disappears with the worktree.

GitHub: vedant-204/website-wasm. You implement ONLY work a human has approved.

1. `git fetch origin`, then:
   gh issue list --state open --label ready-for-agent --json number,title,body,labels --limit 50

2. Pick the LOWEST-numbered issue from that list that does NOT also carry
   `agent-implemented` or `human-review-required`, and has no linked open PR
   (`gh pr list --state open --search "<number>"`).
   If nothing qualifies, print "no issues ready" and stop. Do not go looking for
   other work — an issue without `ready-for-agent` does not exist to you.

3. Read the issue body in full; it is the spec. Read CLAUDE.md and prompt.md
   before writing code. Their rules override your defaults.

4. If the issue is ambiguous enough that you would have to guess at behaviour:
   comment on it with your specific questions, ADD `human-review-required`,
   REMOVE `ready-for-agent`, and stop. Never implement a guess.

5. git checkout -b fix/<issue-number>-<slug>
   (you are already at origin/v2; do NOT set an upstream — push with
   `git push -u origin HEAD` so the branch tracks itself, never v2)

6. Implement exactly what the issue asks. No adjacent refactors. Then verify:
   - cargo check --target wasm32-unknown-unknown
   - cargo test
   - cargo clippy --target wasm32-unknown-unknown
   If something still fails after two fix attempts, continue but say so plainly
   in the PR body.

7. Commit, push, and open a DRAFT PR against v2:
   gh pr create --draft --base v2
   Body: what changed, why, `Closes #<n>`, how you verified, and any failing check.

8. On the issue: REMOVE `ready-for-agent`, ADD `agent-implemented`, comment with
   the PR link. This is what stops it being picked again tomorrow.

9. WRITE THE REPORT — ON EVERY PATH OUT, NOT JUST THIS ONE.

   Whatever happens — a PR opened, no issues ready, an issue too vague, checks
   failing, gh unreachable — your LAST action before exiting is to write
   $HOME/DEV-2/website-p/.agent/last-run.md. If you exit without it, Vedant
   gets a useless "FAILED (no report written)" email with a log dump. That is
   a bug in your run, not an acceptable outcome.

   Note the absolute path: the real repo, not this worktree.

   This file becomes the email he reads, so it has to stand on its own — he
   decides whether to open the PR based on it alone. Shape:

   Line 1 must be the email subject line, formatted:
     Agent A · PR #<pr> for issue #<n> — <checks passed | CHECKS FAILED>
   (or, if you stopped without opening a PR:
     Agent A · issue #<n> needs review — <one-line reason>
   and if there was nothing to do:
     Agent A · no issues ready)

   Then, after a blank line:

   ## What the issue asked for
   Two or three sentences in your own words. Not a paste of the issue body.

   ## What I changed
   One bullet per file touched, each saying what changed there and WHY:
     - src/sim.rs — replaced the Y-squash projection with a perspective
       divide, because flat ellipses were the reason it read as 2-D.
   Include the diffstat (files changed, insertions, deletions).

   ## Verification
   The exact result of each of cargo check / cargo test / cargo clippy.
   If anything failed, paste the actual error, not a summary of it.

   ## What I was unsure about
   Anything you guessed at, any assumption you made, anything you noticed but
   deliberately left alone. If there is genuinely nothing, write "nothing".
   Never leave this section out to look confident.

   ## Links
   PR URL and issue URL.

10. Print the same subject line to stdout so it shows in the log.

HARD RULES
- Never commit to v2 or main. Never merge, close, or force-push.
- Never touch $HOME/DEV-2/website-p except to write .agent/last-run.md.
- Never add the `ready-for-agent` label to anything. Only Vedant applies it.
- One issue per run.
- The worktree is disposable. Do not clean it up yourself; the runner does.
