# Deep-dive questions

Questions raised by the review agent. Answer inline under each `A:`.

Q: prompt.md contradicts itself on where the name sits — 3 says "a single small line of your name in one corner", 4 says "Your name sits under it [the nucleus] in mono, small". Once the zoom navigation in #8 lands the nucleus is no longer at screen centre during a flight. Should the DOM name line be fixed in a corner, or track the projected nucleus and move with the camera?
A:

Q: data.rs carries `inclination` and `ascending_node` for all nine bodies (shell = kind, per the comment at data.rs:47) but nothing reads them, and PR #15 (the 3-D camera) was closed unmerged. Is shell = kind still the confirmed answer to prompt.md 10 Q1, and should it be written into CLAUDE.md now so the next agent does not re-litigate it?
A:

Q: prompt.md 10 Q2 asks whether shell paths stay as faint rings or disappear once the 3-D motion carries the structure. Issue #4 (shell path splitting at z=0) is specced as though they stay. Which is it — and if they stay, does the selected body's ring take the accent colour, or does 7's "one accent colour visible at a time" mean rings are always greyscale?
A:

Q: Does the crawlable resume `<section>` follow prompt.md 7's mono-only type rule, or keep a sans face for ~700 words of long-form prose? This decides whether the page ships one webfont family or two (issue #25).
A:

Q: With prefers-reduced-motion on (issue #23), which pose should the single static frame show — bodies at their authored `phase` seeds, so the page screenshots byte-identically forever, or a fixed number of steps in, so the composition is spread out and reads better? The two goals conflict.
A:
