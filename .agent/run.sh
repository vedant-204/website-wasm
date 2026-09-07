#!/bin/zsh
# Runs the implementation agent. Invoked by launchd; safe to run by hand.
set -u
REPO="$HOME/DEV-2/website-p"
cd "$REPO" || exit 1

STAMP=$(date +%Y-%m-%d_%H%M)
LOG="$REPO/.agent/logs/implement_$STAMP.log"
REPORT="$REPO/.agent/last-run.md"
mkdir -p "$REPO/.agent/logs"

# launchd gives a minimal PATH; add the tools we need.
export PATH="$HOME/.cargo/bin:/opt/homebrew/bin:/usr/local/bin:$PATH"

rm -f "$REPORT"

{
  echo "=== $(date) ==="
  gh auth status || { echo "gh not authenticated — aborting"; exit 1; }

  claude -p "$(cat "$REPO/.agent/implement.md")" \
    --allowedTools "Read,Grep,Glob,Edit,Write,Bash(git:*),Bash(gh:*),Bash(cargo:*),Bash(trunk:*)"
} >> "$LOG" 2>&1
STATUS=$?

# The agent writes .agent/last-run.md as its final step. If it did not get that
# far, email the tail of the log instead — a silent failure is the worst outcome.
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

TAIL=$(echo "$SUBJECT" | cut -c1-120)
osascript -e "display notification \"${TAIL//\"/}\" with title \"Agent: implement\" subtitle \"exit $STATUS\""
exit $STATUS
