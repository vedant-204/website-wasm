# v2 brief — from flat orbits to a 3-D atom

Working notes to hand to Claude Code. Read `CLAUDE.md` first for architecture
and boundaries; this file describes **what changes and why**, not how the repo
is laid out.

v1 runs and is correct. It is also too flashy, too flat and too decorative.
v2 is a redesign of the visual model and the interaction model. The Rust,
the module boundaries and `data.rs` all stay.

---

## 1. What is wrong with v1

- Reads as a **2-D solar system**. The orbits are flat ellipses squashed on the
  Y axis, so it looks like a diagram of a diagram, not a thing in space.
- Too much chrome: a big title, three stat tiles, speed controls, a legend, a
  hint line. The frame is louder than the content.
- Motion is too fast to feel calm, and the default speed is wrong.
- It is a **visual**, not an **interface**. You look at it, you do not use it.
  Clicking a body swaps a panel — that is not exploration.

## 2. The direction

**An atom, not a solar system.** Bodies move in genuine 3-D on inclined shells
around a nucleus, with depth you can actually perceive. Quiet, black, minimal.
The only bright things on screen should be the particles themselves.

**Explorable, not scrollable.** Clicking a body flies the camera into it and you
are then inside that body's own system, looking at its sub-particles. Escape
flies back out. The site is navigated, not read.

---

## 3. Hard requirements

### Remove

- The `ORBITAL WORK MAP` heading block entirely. No title on the canvas.
- The stat tiles (`5 ROLES`, `33k LOC SOLO`, `150+ BRANDS`).
- The speed control button. Motion runs at one fixed rate.
- The `ORBITS` toggle. Shell paths are either always faint or always off —
  decide once, do not expose it.
- The nebula clouds and the amber corona glow around the nucleus.
- Every rounded panel border that is not carrying information.

### Keep

- The **skill belt as stars** — this is the best part of v1. Keep the dots, keep
  the hover-to-name behaviour, keep the discipline colouring. In the atom model
  they read as the electron cloud at the outer edge.
- The colour semantics: employment / product / origin / recent capture.
- `data.rs` as the single source of truth, and the crawlable resume section.
- Mass = hours. Eccentricity = how settled something is.

### Change

- **Background: pure black** (`#000`). Not `#04060E`. Stars are the only
  texture.
- **Default motion speed: the current `0.3×`.** Make that the new `1.0` in the
  data — do not ship a multiplier of 0.3, rescale `n` in `sim.rs` so the natural
  speed is the slow one.
- Chrome shrinks to: the detail panel, and a single small line of your name in
  one corner. Nothing else at rest.

---

## 4. The atom model

Replace the flat-ellipse projection (`FLAT = 0.60` squash) with real 3-D.

**Geometry**

- Each body orbits on a **shell**: a circle of radius `a` in its own plane,
  where the plane is defined by two Euler angles (`inclination`, `ascending
  node`). Give shells widely different inclinations — including near-vertical
  ones — so the result reads as an electron cloud, not a disc. This is the
  single change that makes it stop looking 2-D.
- Keep eccentricity: shells are ellipses, not circles. A recent capture still
  rides a long one.
- Project with a real perspective divide: `scale = f / (f + z)` with the camera
  on `+z`. Do not fake depth with a Y-squash.
- The whole system rotates slowly on its vertical axis so the 3-D structure is
  legible without any input. Roughly one revolution per 90 seconds.

**Depth cues — all four, or it will still look flat**

1. **Painter's algorithm.** Sort every drawable (bodies, moons, star dots, shell
   path segments) by `z` and draw far to near. Shell paths must be split so the
   half behind the nucleus is drawn *before* it and the half in front *after*.
2. **Size** from the perspective divide.
3. **Atmospheric fade** — far things dimmer and slightly desaturated.
4. **Labels only for the near hemisphere.** A label on a body behind the nucleus
   is noise; fade it out with `z`.

**Nucleus**

Not a glowing sun. Something small, tight and structural — a cluster of a few
overlapping spheres, or a slowly tumbling wireframe polyhedron. It should read
as dense matter, not as a light source. Your name sits under it in mono, small.

**Semantics — decide before building**

The atom metaphor gives you shells, and shells need to mean something or the
model is decoration. Pick one and write it into `CLAUDE.md`:

- **Shell = era** (2021-23 inner, 2024 middle, 2025-26 outer), or
- **Shell = kind** (employment / product / origin each on their own inclination
  band), or
- **Shell number = mass tier**, so heavy work sits close in.

Preference: **shell = kind**, because it lets employment, products and origin
each occupy a visually distinct plane, and the eye separates them instantly.

---

## 5. Zoom-in navigation

This is the main new feature. Three states.

```
SYSTEM  →  (click a body)  →  FOCUSED  →  (click a sub-particle)  →  DETAIL
        ←      (Esc)       ←            ←        (Esc)            ←
```

**SYSTEM** — the whole atom. Bodies labelled, skill stars at the edge.

**FOCUSED** — the camera flies to the selected body over ~900ms with an ease-out
cubic. During the flight:

- The camera target lerps from the nucleus to the body.
- The zoom scale lerps in so the body fills roughly a third of the frame.
- Every other body fades to ~8% opacity but keeps moving. Do not delete them —
  seeing the rest of the system faintly is what makes it feel like space.
- The body's own moons spread out and become the new primary objects, each with
  its own label.
- The detail panel slides in with the body's bullets and stack chips.

For a body with no moons (UpGrad, Traveey), generate sub-particles from its
`chips` so there is always something inside to look at. Every body must reward a
click.

**DETAIL** — clicking a moon pins one line of copy about that specific thing.
Small. This is a caption, not a third page.

**Rules**

- `Esc` and a back affordance both fly out. Browser back should work too.
- The URL hash tracks state: `#newengen` and `#newengen/lift-ai`.
- Never cut — always fly. A jump cut destroys the sense of a continuous space.
- Camera state belongs in `sim.rs` as plain numbers (`target`, `zoom`, `t`), so
  it stays testable and the renderer just reads it.

---

## 6. Make it an interface, not a poster

At least two of these, chosen for what they say about the work:

- **Hover on a skill star** highlights every body that used it, and draws thin
  lines from the star to those bodies. Hovering `Kubernetes` lighting up
  NewEngen and Reliable AI at once tells a career story no bullet list does.
- **Keyboard navigation**: arrows step between bodies on the current shell,
  Enter zooms in, Esc out. Fully operable without a mouse.
- **A time control** — drag through 2021→2026 and bodies appear as they were
  actually joined, the atom assembling itself. This is the strongest idea if
  you only build one, but it is also the most work.
- **Drag to orbit the camera** — hold and drag rotates the whole atom. Cheap,
  and it is the thing that proves to a visitor that it is really 3-D.

Build **drag-to-orbit** and **skill-star linking** first. They are the two that
change the page from a video into a thing you operate.

---

## 7. What "minimal" means here

Not "fewer features" — fewer *marks*.

- One accent colour visible at a time: the selected body's. Everything else is
  greyscale or near-greyscale until you touch it.
- No borders unless they separate two things that would otherwise merge. The
  detail panel can be type on black with a single hairline rule.
- No box shadows, no blur backdrops, no rounded corners over 2px.
- Type: mono for every label, number and caption. One weight, two sizes.
  The only large type on the page is a body's name in the detail panel.
- Motion is slow and continuous. Nothing pulses, nothing blinks, nothing
  bounces. Easing is always ease-out, never ease-in-out.
- At rest, before any interaction, the screen should be: black, ~300 stars, one
  atom, one small name. That is the whole composition.

---

## 8. Acceptance

- [ ] Screenshot at rest reads unmistakably 3-D — you can tell which bodies are
      in front without moving anything.
- [ ] Background is pure black; the brightest pixels on screen are particles.
- [ ] Zero speed controls, zero stat tiles, no page title.
- [ ] Clicking any body flies in, and every body has something inside it.
- [ ] Esc, browser back and the URL hash all agree on the current state.
- [ ] Drag rotates the atom; releasing it keeps the momentum and settles.
- [ ] Skill stars still name themselves on hover.
- [ ] `cargo test` passes; the sim tests are updated for 3-D positions.
- [ ] Release `.wasm` still under 400 KB gzipped.
- [ ] Works at 390px wide: fewer stars, panel becomes a bottom sheet, drag still
      rotates.

## 9. Out of scope

- No wgpu yet. Canvas2D with a hand-rolled projection and z-sort is enough for
  ~40 bodies and ~300 stars. Revisit only when the dust layer arrives.
- No backend. Still static.
- No new dependencies. Projection is a 3x3 matrix and a divide.
- Do not touch the crawlable resume `<section>` other than keeping it in sync.

## 10. Open questions to answer before coding

1. Shell semantics — confirm **shell = kind**, or pick another and record it.
2. Do shell paths stay visible as faint rings, or disappear entirely once the
   3-D motion carries the structure? Try both, keep the quieter one.
3. Does the skill cloud rotate with the atom, or stay fixed as a backdrop?
   Fixed probably reads better as "sky", rotating reads better as "part of me".
