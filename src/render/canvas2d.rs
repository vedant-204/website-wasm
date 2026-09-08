use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use super::{Renderer, Scene};
use crate::data::{BODIES, SKILLS, SKILL_COLORS};
use crate::sim;

const VOID: &str = "#04060E";
const TAU: f64 = std::f64::consts::TAU;

fn fill(ctx: &CanvasRenderingContext2d, css: &str) {
    ctx.set_fill_style_str(css);
}
fn stroke(ctx: &CanvasRenderingContext2d, css: &str) {
    ctx.set_stroke_style_str(css);
}

fn alpha(hex: &str, a: f32) -> String {
    format!("{}{:02X}", hex, (a.clamp(0.0, 1.0) * 255.0) as u8)
}

pub struct Canvas2d {
    ctx: CanvasRenderingContext2d,
    stars: Vec<(f32, f32, f32)>,
    w: f32,
    h: f32,
}

impl Canvas2d {
    pub fn new(canvas: &HtmlCanvasElement) -> Result<Self, JsValue> {
        let ctx = canvas
            .get_context("2d")?
            .ok_or("no 2d context")?
            .dyn_into::<CanvasRenderingContext2d>()?;
        Ok(Canvas2d { ctx, stars: Vec::new(), w: 0.0, h: 0.0 })
    }

    fn seed_stars(&mut self) {
        self.stars.clear();
        let mut s: u32 = 0x2545F491;
        let mut next = || {
            s ^= s << 13;
            s ^= s >> 17;
            s ^= s << 5;
            (s % 10_000) as f32 / 10_000.0
        };
        for _ in 0..300 {
            self.stars.push((next() * self.w, next() * self.h, next()));
        }
    }

    fn background(&self) {
        let ctx = &self.ctx;
        fill(ctx, VOID);
        ctx.fill_rect(0.0, 0.0, self.w as f64, self.h as f64);
        for &(x, y, m) in &self.stars {
            fill(ctx, &format!("rgba(220,230,255,{:.2})", 0.10 + m * 0.45));
            ctx.begin_path();
            let _ = ctx.arc(x as f64, y as f64, (m * 1.1 + 0.25) as f64, 0.0, TAU);
            ctx.fill();
        }
    }

    fn core(&self, sim: &crate::sim::Sim) {
        let ctx = &self.ctx;
        // Project the nucleus (at world origin) through the camera
        let (cx, cy, _, _) = sim.camera.project(0.0, 0.0, 0.0, sim.cx, sim.cy);
        let (cx, cy) = (cx as f64, cy as f64);
        let t = sim.t as f64;
        let pulse = 1.0 + (t * 1.6).sin() * 0.05;

        if let Ok(g) = ctx.create_radial_gradient(cx, cy, 0.0, cx, cy, 64.0 * pulse) {
            let _ = g.add_color_stop(0.0, "rgba(255,244,222,0.90)");
            let _ = g.add_color_stop(0.22, "rgba(242,169,59,0.42)");
            let _ = g.add_color_stop(1.0, "rgba(242,169,59,0)");
            ctx.set_fill_style_canvas_gradient(&g);
            ctx.begin_path();
            let _ = ctx.arc(cx, cy, 64.0 * pulse, 0.0, TAU);
            ctx.fill();
        }

        ctx.save();
        let _ = ctx.translate(cx, cy);
        stroke(ctx, "rgba(242,169,59,0.55)");
        ctx.set_line_width(1.0);
        for i in 0..3 {
            let r = 16.0 + i as f64 * 8.0;
            let rot = t * if i % 2 == 0 { 0.7 } else { -0.5 } + i as f64;
            ctx.begin_path();
            let _ = ctx.arc(0.0, 0.0, r, rot, rot + std::f64::consts::PI * (1.1 - i as f64 * 0.22));
            ctx.stroke();
        }
        let _ = ctx.rotate(t * 0.18);
        stroke(ctx, "rgba(255,235,200,0.75)");
        ctx.set_line_width(1.4);
        ctx.begin_path();
        for i in 0..6 {
            let a = i as f64 / 6.0 * TAU;
            let (x, y) = (a.cos() * 10.0, a.sin() * 10.0);
            if i == 0 { ctx.move_to(x, y) } else { ctx.line_to(x, y) }
        }
        ctx.close_path();
        ctx.stroke();
        ctx.restore();

        ctx.set_text_align("center");
        ctx.set_font("600 11px 'IBM Plex Mono', monospace");
        fill(ctx, "rgba(237,234,226,0.95)");
        let _ = ctx.fill_text("VEDANTDEV KATYAYAN", cx, cy + 50.0);
        ctx.set_font("400 9.5px 'IBM Plex Mono', monospace");
        fill(ctx, "#5A6479");
        let _ = ctx.fill_text("DELHI, IN \u{00B7} BARYCENTRE", cx, cy + 64.0);
    }
}

impl Renderer for Canvas2d {
    fn resize(&mut self, w: f32, h: f32, dpr: f32) {
        self.w = w;
        self.h = h;
        let _ = self.ctx.set_transform(dpr as f64, 0.0, 0.0, dpr as f64, 0.0, 0.0);
        self.seed_stars();
    }

    fn draw(&mut self, scene: &Scene) {
        let ctx = &self.ctx;
        let sim = scene.sim;
        let cam = &sim.camera;
        self.background();

        // Skill belt — project each skill through the camera.
        for (i, sk) in SKILLS.iter().enumerate() {
            let ss = &sim.skills[i];
            let (wx, wy, wz) = (ss.wx * sim.scale, ss.wy * sim.scale, ss.wz * sim.scale);
            let (sx, sy, _depth, _proj) = cam.project(wx, wy, wz, sim.cx, sim.cy);
            let near = ((sx - scene.pointer.0).powi(2) + (sy - scene.pointer.1).powi(2)).sqrt() < 26.0;
            let color = SKILL_COLORS[sk.group];
            fill(ctx, &if near { color.to_string() } else { alpha(color, 0.4) });
            ctx.begin_path();
            let _ = ctx.arc(sx as f64, sy as f64, if near { 3.2 } else { 1.9 }, 0.0, TAU);
            ctx.fill();
            if near {
                ctx.set_font("500 10px 'IBM Plex Mono', monospace");
                ctx.set_text_align("center");
                fill(ctx, "rgba(237,234,226,0.98)");
                let _ = ctx.fill_text(&sk.name.to_uppercase(), sx as f64, (sy - 10.0) as f64);
            }
        }

        // Orbit paths — sample the ellipse in 3D and project each point.
        if scene.show_orbits {
            ctx.set_line_width(1.0);
            for (i, body) in BODIES.iter().enumerate() {
                let on = scene.selected == Some(i);
                stroke(ctx, &if on { alpha(body.kind.color(), 0.33) } else { "rgba(150,170,210,0.09)".into() });
                ctx.begin_path();
                for k in 0..=80 {
                    let anom = k as f32 / 80.0 * std::f32::consts::TAU;
                    let (wx, wy, wz) = sim::place(body, anom, sim.scale);
                    let (sx, sy, _, _) = cam.project(wx, wy, wz, sim.cx, sim.cy);
                    if k == 0 { ctx.move_to(sx as f64, sy as f64) } else { ctx.line_to(sx as f64, sy as f64) }
                }
                ctx.close_path();
                ctx.stroke();
            }
        }

        // Trails — project world-space trail points through the camera.
        ctx.set_line_cap("round");
        for (i, body) in BODIES.iter().enumerate() {
            let st = &sim.bodies[i];
            let n = st.trail.len();
            for k in 1..n {
                let f = k as f32 / n as f32;
                stroke(ctx, &alpha(body.kind.color(), f * f * 0.6));
                let (_, _, _, p0) = cam.project(st.trail[k - 1].0, st.trail[k - 1].1, st.trail[k - 1].2, sim.cx, sim.cy);
                let (x0, y0, _, _) = cam.project(st.trail[k - 1].0, st.trail[k - 1].1, st.trail[k - 1].2, sim.cx, sim.cy);
                let (x1, y1, _, _) = cam.project(st.trail[k].0, st.trail[k].1, st.trail[k].2, sim.cx, sim.cy);
                ctx.set_line_width((f * st.radius * 0.55 * p0) as f64);
                ctx.begin_path();
                ctx.move_to(x0 as f64, y0 as f64);
                ctx.line_to(x1 as f64, y1 as f64);
                ctx.stroke();
            }
        }

        self.core(sim);

        // Bodies, moons, labels.
        ctx.set_text_align("center");
        for (i, body) in BODIES.iter().enumerate() {
            let st = &sim.bodies[i];
            let on = scene.selected == Some(i) || scene.hovered == Some(i);
            let color = body.kind.color();
            let (x, y) = (st.sx as f64, st.sy as f64);
            let r = (st.radius * st.proj) as f64;

            // Moons — project each moon position through the camera.
            for (j, moon) in body.moons.iter().enumerate() {
                let mr = moon.dist * sim.scale;
                let ang = st.moon_angles[j];
                let moon_wx = st.wx + ang.cos() * mr;
                let moon_wz = st.wz + ang.sin() * mr;
                let moon_wy = st.wy;
                let (mx, my, _, mp) = cam.project(moon_wx, moon_wy, moon_wz, sim.cx, sim.cy);
                let (mx, my) = (mx as f64, my as f64);

                // Moon orbit ring — sample points and project.
                stroke(ctx, if on { "rgba(150,170,210,0.20)" } else { "rgba(150,170,210,0.07)" });
                ctx.set_line_width(1.0);
                ctx.begin_path();
                for k in 0..=40 {
                    let a = k as f32 / 40.0 * std::f32::consts::TAU;
                    let ox = st.wx + a.cos() * mr;
                    let oz = st.wz + a.sin() * mr;
                    let oy = st.wy;
                    let (px, py, _, _) = cam.project(ox, oy, oz, sim.cx, sim.cy);
                    if k == 0 { ctx.move_to(px as f64, py as f64) } else { ctx.line_to(px as f64, py as f64) }
                }
                ctx.close_path();
                ctx.stroke();

                fill(ctx, &alpha(color, 0.85));
                ctx.begin_path();
                let _ = ctx.arc(mx, my, (moon.size * mp) as f64, 0.0, TAU);
                ctx.fill();
                if on {
                    ctx.set_font("400 9.5px 'IBM Plex Mono', monospace");
                    fill(ctx, "rgba(237,234,226,0.90)");
                    let _ = ctx.fill_text(&moon.name.to_uppercase(), mx, my - 9.0);
                }
            }

            // Body glow
            if let Ok(g) = ctx.create_radial_gradient(x, y, 0.0, x, y, r * 4.0) {
                let _ = g.add_color_stop(0.0, &alpha(color, if on { 0.53 } else { 0.27 }));
                let _ = g.add_color_stop(1.0, &alpha(color, 0.0));
                ctx.set_fill_style_canvas_gradient(&g);
                ctx.begin_path();
                let _ = ctx.arc(x, y, r * 4.0, 0.0, TAU);
                ctx.fill();
            }

            // Body disc
            fill(ctx, color);
            ctx.begin_path();
            let _ = ctx.arc(x, y, r, 0.0, TAU);
            ctx.fill();
            fill(ctx, "rgba(255,255,255,0.55)");
            ctx.begin_path();
            let _ = ctx.arc(x - r * 0.3, y - r * 0.3, r * 0.35, 0.0, TAU);
            ctx.fill();

            // Selection ring
            if on {
                stroke(ctx, color);
                ctx.set_line_width(1.0);
                ctx.begin_path();
                let _ = ctx.arc(x, y, r + 8.0 + (sim.t as f64 * 3.0).sin() * 1.5, 0.0, TAU);
                ctx.stroke();
            }

            // Label
            ctx.set_font(if on {
                "600 12px 'IBM Plex Mono', monospace"
            } else {
                "400 10.5px 'IBM Plex Mono', monospace"
            });
            fill(ctx, if on { "rgba(237,234,226,1)" } else { "rgba(150,160,184,0.82)" });
            let _ = ctx.fill_text(&body.name.to_uppercase(), x, y - r - 11.0);
        }
    }
}

use wasm_bindgen::JsCast;
