#!/bin/zsh
# Runs the implementation agent inside a throwaway git worktree, so Vedant's
# working tree, branch and uncommitted edits are irrelevant to it.
set -u
REPO="$HOME/DEV-2/website-p"
cd "$REPO" || exit 1

export PATH="$HOME/.cargo/bin:/opt/homebrew/bin:/usr/local/bin:$PATH"

STAMP=$(date +%Y-%m-%d_%H%M)
LOG="$REPO/.agent/logs/implement_$STAMP.log"
REPORT="$REPO/.agent/last-run.md"
WT="$REPO/.agent/wt"              # scratch checkout, gitignored
mkdir -p "$REPO/.agent/logs"
rm -f "$REPORT"

cleanup() {
  cd "$REPO" || return
  git worktree remove --force "$WT" 2>/dev/null
  git worktree prune 2>/dev/null
}
trap cleanup EXIT INT TERM

{
  echo "=== $(date) ==="
  gh auth status || { echo "gh not authenticated — aborting"; exit 1; }

  git fetch origin --prune || { echo "git fetch failed — aborting"; exit 1; }

  # Fresh detached worktree at origin/v2. Never reuses a stale one.
  cleanup
  git worktree add --detach "$WT" origin/v2 || { echo "worktree add failed"; exit 1; }
  echo "worktree at $WT on $(git -C "$WT" rev-parse --short HEAD)"

  cd "$WT" || exit 1

  # Credentials and prompts live in the real repo, not the worktree.
  claude -p "$(cat "$REPO/.agent/implement.md")" \
    --allowedTools "Read,Grep,Glob,Edit,Write,Bash(git:*),Bash(gh:*),Bash(cargo:*),Bash(trunk:*)"
} >> "$LOG" 2>&1
STATUS=$?

if [[ -f "$REPORT" ]]; then
  SUBJECT=$(sed -n '1p' "$REPORT" | sed 's/^#* *//')
  [[ -n "$SUBJECT" ]] || SUBJECT="Agent A — run complete"
else
  SUBJECT="Agent A — FAILED (no report written)"
  REPORT=$(mktemp)
  {
    echo "The run did not produce .agent/last-run.md, so it failed before finishing."
    echo "Exit status: $STATUS"
    echo
    echo "Last 60 lines of $LOG:"
    echo "----------------------------------------"
    tail -60 "$LOG"
  } > "$REPORT"
fi

"$REPO/.agent/notify.sh" "$SUBJECT" "$REPORT"
osascript -e "display notification \"$(echo "$SUBJECT" | cut -c1-120 | tr -d '\"')\" with title \"Agent: implement\" subtitle \"exit $STATUS\""
exit $STATUS
