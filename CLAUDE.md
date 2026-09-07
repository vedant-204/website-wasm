# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

# Orbital Work Map

A personal site for **Vedantdev Katyayan** (AI engineer, Delhi). The homepage is
an n-body orbital map: every role, product and skill is a body orbiting a core.
Written in Rust, compiled to WebAssembly, served as static files.

## The concept — do not dilute it

The physics carries meaning. If a change breaks one of these, it is wrong:

- **Mass = hours invested**, never a self-rating. NewEngen is the heaviest body
  because two years of production work outweighs a four-month internship.
- **Eccentricity = how settled something is.** A recent capture (Rust, `e=0.55`)
  rides a long ellipse. When it settles you lower one number and the site tells
  the truth on its own.
- **Moons = what was shipped there.** LIFT AI orbits NewEngen, not the core.
  Hierarchy falls out of the simulation instead of an indented list.
- **The skill belt** is the outer asteroid ring. It exists to fill the frame with
  real content — without it the composition reads as empty.

Anything that turns this into a generic particle screensaver with a resume
pasted next to it defeats the point.

## Architecture

```
index.html          Trunk entry. Chrome, controls, and the crawlable resume layer.
assets/style.css    All styling. No CSS in Rust.
src/data.rs         SINGLE SOURCE OF TRUTH for content. Bodies, moons, skills.
src/sim.rs          Kepler motion, trails, belt, hit-testing. No DOM, no drawing.
src/render/mod.rs   Renderer trait + Scene. The seam for a future wgpu backend.
src/render/canvas2d.rs  The only file allowed to call a drawing API.
src/lib.rs          wasm entry, RAF loop, event wiring, the JS boundary.
```

**Boundaries that must hold:**

- `sim.rs` stays pure — it compiles and tests on the host target (`cargo test`).
  Never import `web_sys` there.
- Rust owns the canvas. **The DOM owns all text.** Canvas text is invisible to
  screen readers, crawlers and text selection. `render_panel` in `lib.rs` writes
  the detail panel as real HTML.
- The only thing crossing into JS is a body id (`window.orbitalSelect`), used to
  keep the URL hash in sync for deep links.
- Content changes happen in `src/data.rs`. Nothing else hardcodes copy.

**JS-callable exports (via `#[wasm_bindgen]` in `lib.rs`):**

| Function | Called from |
|---|---|
| `set_speed(f32)` | speed slider in HTML |
| `toggle_running() → bool` | pause button |
| `toggle_orbits() → bool` | orbits button |
| `select_by_id(str)` | deep-link routing (`/#bodyid`) on page load |

Deep links work because `index.html` reads `window.location.hash` on load and
calls `select_by_id`. Every click also writes the hash via `window.orbitalSelect`.

**Coordinate system:**

- Origin is the centre of the canvas (`cx`, `cy`).
- `scale = min(w, h) * 0.42` — body `a` values are fractions of this.
- `FLAT = 0.60` squashes the y-axis to fake an inclined orbital plane. Apply it
  whenever converting between sim y and screen y.

**`render_panel` is safe to use `set_inner_html`** only because the content is
static string literals from `data.rs`. Never pass fetched or user-supplied data
through it — that would be XSS.

## Commands

```bash
rustup target add wasm32-unknown-unknown   # once
cargo install trunk                        # once

trunk serve                # dev at http://127.0.0.1:8080, hot reload
trunk build --release      # optimised bundle into dist/
cargo test                 # sim.rs unit tests (host target, fast)
cargo clippy --target wasm32-unknown-unknown
```

## Constraints

- **Bundle budget: under 400 KB gzipped.** Check after every dependency you add:
  `ls -lh dist/*.wasm`. `opt-level="z"`, LTO and `wasm-opt -Oz` are already set.
  A homepage that ships 2 MB of wasm has failed regardless of how it looks.
- **No backend.** This deploys as static files to Cloudflare Pages. If a feature
  needs a server (contact form, live GitHub data, an AI chat endpoint), it goes
  in a separate Cloudflare Worker in `worker/` — never a server for the site.
- **Deterministic visuals.** The starfield uses a fixed xorshift seed so the page
  screenshots identically every load. Do not introduce `rand`.
- No new dependencies without a reason that survives the bundle budget. In
  particular: no game engine, no `rapier` — the motion here is two lines of
  Kepler math.

## Adding or updating a body

All changes are in `src/data.rs`. Add an entry to `BODIES` (or edit an existing
one). Fields to set intentionally:

- `mass` — hours invested, not a rating. Drives the rendered radius (`4.0 + mass * 1.55`).
- `a` — semi-major axis as a fraction of `scale` (0.0–1.0). Inner bodies orbit faster (Kepler's third law: `n = 0.62 / a^1.5`).
- `e` — eccentricity. 0 = circle, 0.5+ = visibly elongated. Use high `e` for recently acquired skills/roles.
- `tilt` — rotation of the ellipse in radians.
- `moons` — shipped products that belong to this role. `dist` is a fraction of `scale`.

Also update the `<section id="resume">` in `index.html` to match — until the
build.rs TODO is resolved, these are duplicated.

## Known issues / TODO

- [ ] The resume `<section>` in `index.html` duplicates `src/data.rs`. Generate
      it at build time from `BODIES` (a `build.rs` emitting a partial) so there
      is one source of truth. **Do this before adding more bodies.**
- [ ] `prefers-reduced-motion` currently only unpins the stage height. It should
      render one good static frame with labels — recruiters screenshot.
- [ ] No keyboard navigation. Arrow keys should step between bodies and focus
      the panel.
- [ ] Depth: bodies passing behind the core should be dimmed and drawn first.
      Cheap, and it makes the scene read as 3D.
- [ ] Dust layer (a few thousand particles pulled by the core). This is what
      justifies wgpu — Canvas2D will not hold 60fps. Implement behind the
      `Renderer` trait as `render/wgpu.rs`, do not rewrite `canvas2d.rs`.

## Style

- Palette lives in `assets/style.css` and `data.rs::Kind::color`. Amber is
  employment, cyan is shipped product, violet is origin, coral is a recent
  capture, and the belt is grouped by discipline.
- Type: Bricolage Grotesque for display, IBM Plex Sans for body, IBM Plex Mono
  for every label, readout and number. Labels are uppercase with wide tracking.
- British spelling in prose. Copy is plain and specific — no "passionate about",
  no "innovative solutions".

## Agent skills

### Issue tracker

Issues live in GitHub (`vedant-204/website-wasm`) via the `gh` CLI. See `docs/agents/issue-tracker.md`.

### Triage labels

Default five-label vocabulary (`needs-triage`, `needs-info`, `ready-for-agent`, `ready-for-human`, `wontfix`). See `docs/agents/triage-labels.md`.

### Domain docs

Single-context layout (`CONTEXT.md` + `docs/adr/` at repo root). See `docs/agents/domain.md`.
