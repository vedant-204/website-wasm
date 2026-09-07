---
description: Add a role, product or skill to the orbital map
---

Add a new body to the map: $ARGUMENTS

Steps:

1. Read `src/data.rs` and pick values that carry meaning, not just aesthetics:
   - `mass` — hours actually invested, on the same scale as the existing bodies
     (NewEngen 9.4 is two years; UpGrad 2.6 is four months).
   - `a` — semi-major axis. Keep bodies at least 0.05 apart or their labels
     collide.
   - `e` — how settled it is. Something started this year should be 0.4+.
   - `kind` — Employment / Product / Origin / Capture. This picks the colour.
   - `tilt` and `phase` — spread these so orbits do not visually stack.
2. Append the `Body` to `BODIES`. Sub-projects go in `moons`, not as new bodies.
3. Mirror the entry in the crawlable `<section class="resume">` in `index.html`
   (until the build-time generator in the TODO exists).
4. Run `cargo test` — `orbits_stay_on_their_ellipse` catches a bad `e`.
5. Run `trunk serve` and confirm the label does not overlap a neighbour at both
   1440px and 390px wide.
