#!/bin/zsh
# Sends a report by email via Gmail SMTP.
#   usage: notify.sh "Subject line" /path/to/body.md
#
# Credentials come from .agent/smtp (gitignored), two lines:
#   you@gmail.com
#   your-16-char-app-password
# Get the app password at https://myaccount.google.com/apppasswords
# (requires 2-step verification on the Google account).

set -u
REPO="$HOME/DEV-2/website-p"
SUBJECT="$1"
BODY_FILE="$2"
TO="katyayana2003@gmail.com"
CRED="$REPO/.agent/smtp"

[[ -f "$CRED" ]] || { echo "notify: no $CRED — skipping email"; exit 0; }
SMTP_USER=$(sed -n '1p' "$CRED")
SMTP_PASS=$(sed -n '2p' "$CRED")

MSG=$(mktemp)
{
  echo "From: Agent A <$SMTP_USER>"
  echo "To: $TO"
  echo "Subject: $SUBJECT"
  echo "Date: $(date -R)"
  echo "Content-Type: text/plain; charset=UTF-8"
  echo
  cat "$BODY_FILE"
} > "$MSG"

curl -s --url "smtps://smtp.gmail.com:465" --ssl-reqd \
  --mail-from "$SMTP_USER" --mail-rcpt "$TO" \
  --upload-file "$MSG" \
  --user "$SMTP_USER:$SMTP_PASS" \
  && echo "notify: sent to $TO" || echo "notify: FAILED"

rm -f "$MSG"
