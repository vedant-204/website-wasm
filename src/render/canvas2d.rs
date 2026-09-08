use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use super::{Renderer, Scene};
use crate::data::{BODIES, SKILLS, SKILL_COLORS};

const VOID: &str = "#000";
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
        let (cx, cy) = (sim.cx as f64, sim.cy as f64);
        fill(ctx, "rgba(237,234,226,0.70)");
        ctx.begin_path();
        let _ = ctx.arc(cx, cy, 3.0, 0.0, TAU);
        ctx.fill();
        ctx.set_text_align("center");
        ctx.set_font("600 11px 'IBM Plex Mono', monospace");
        fill(ctx, "rgba(237,234,226,0.95)");
        let _ = ctx.fill_text("VEDANTDEV KATYAYAN", cx, cy + 22.0);
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
        self.background();

        for (i, sk) in SKILLS.iter().enumerate() {
            let st = &sim.skills[i];
            let r = st.radius * sim.scale;
            let x = sim.cx + st.angle.cos() * r;
            let y = sim.cy + st.angle.sin() * r * sim.flat();
            let near = ((x - scene.pointer.0).powi(2) + (y - scene.pointer.1).powi(2)).sqrt() < 26.0;
            let color = SKILL_COLORS[sk.group];
            let period = 22.0 + ((i * 17 + 11) % 23) as f32;
            let phase = ((i * 73 + 37) % 100) as f32 / 100.0 * period;
            let t_in = (sim.t + phase) % period;
            let dist = (t_in - period * 0.5).abs();
            let pw = period * 0.07;
            let bright = ((1.0 - dist / pw).clamp(0.0, 1.0)).powi(2);
            if !near {
                let base_a = 0.3 + bright * 0.7;
                let dot_r = (1.6 + bright * 0.6) as f64;
                fill(ctx, &alpha(color, base_a));
                ctx.begin_path();
                let _ = ctx.arc(x as f64, y as f64, dot_r, 0.0, TAU);
                ctx.fill();
                if bright > 0.7 {
                    fill(ctx, &format!("rgba(255,255,255,{:.2})", (bright - 0.7) * 1.5));
                    ctx.begin_path();
                    let _ = ctx.arc(x as f64, y as f64, dot_r * 0.45, 0.0, TAU);
                    ctx.fill();
                    let label_a = ((bright - 0.7) / 0.3).clamp(0.0, 1.0);
                    ctx.set_font("500 9px 'IBM Plex Mono', monospace");
                    ctx.set_text_align("center");
                    fill(ctx, &format!("rgba(237,234,226,{:.2})", label_a * 0.85));
                    let _ = ctx.fill_text(&sk.name.to_uppercase(), x as f64, (y - 8.0) as f64);
                }
            } else {
                fill(ctx, color);
                ctx.begin_path();
                let _ = ctx.arc(x as f64, y as f64, 3.2, 0.0, TAU);
                ctx.fill();
            }
            if near {
                ctx.set_font("500 10px 'IBM Plex Mono', monospace");
                ctx.set_text_align("center");
                fill(ctx, "rgba(237,234,226,0.98)");
                let _ = ctx.fill_text(&sk.name.to_uppercase(), x as f64, (y - 10.0) as f64);
            }
        }

        if scene.show_orbits {
            for (i, body) in BODIES.iter().enumerate() {
                let on = scene.selected == Some(i);
                let color = if on { alpha(body.kind.color(), 0.33) } else { "rgba(150,170,210,0.09)".into() };
                fill(ctx, &color);
                for k in 0..160 {
                    if k % 4 < 2 { continue; }
                    let (x, y) = sim.place(body, k as f32 / 160.0 * std::f32::consts::TAU);
                    ctx.begin_path();
                    let _ = ctx.arc(x as f64, y as f64, 0.6, 0.0, TAU);
                    ctx.fill();
                }
            }
        }

        let mut order: Vec<usize> = (0..BODIES.len()).collect();
        order.sort_by(|&a, &b| {
            sim.bodies[b].z.partial_cmp(&sim.bodies[a].z).unwrap_or(std::cmp::Ordering::Equal)
        });

        ctx.set_text_align("center");
        let mut core_drawn = false;
        for &i in &order {
            let body = &BODIES[i];
            let st = &sim.bodies[i];

            if !core_drawn && st.z <= 0.0 {
                self.core(sim);
                core_drawn = true;
            }

            let on = scene.selected == Some(i) || scene.hovered == Some(i);
            let color = body.kind.color();
            let depth_norm = (st.z / sim.scale).clamp(-1.0, 1.0);
            let fade = 1.0 - depth_norm.max(0.0) * 0.6;
            let size_scale = 1.0 - depth_norm.max(0.0) * 0.3;
            let (x, y) = (st.x as f64, st.y as f64);
            let r = (st.radius * size_scale) as f64;

            for (j, moon) in body.moons.iter().enumerate() {
                let mr = (moon.dist * sim.scale) as f64;
                let ang = st.moon_angles[j] as f64;
                let mx = x + ang.cos() * mr;
                let my = y + ang.sin() * mr * sim.flat() as f64;
                stroke(ctx, if on { "rgba(150,170,210,0.20)" } else { "rgba(150,170,210,0.07)" });
                ctx.set_line_width(1.0);
                ctx.begin_path();
                let _ = ctx.ellipse(x, y, mr, mr * sim.flat() as f64, 0.0, 0.0, TAU);
                ctx.stroke();
                fill(ctx, &alpha(color, 0.85 * fade));
                ctx.begin_path();
                let _ = ctx.arc(mx, my, moon.size as f64, 0.0, TAU);
                ctx.fill();
                if on {
                    ctx.set_font("400 9.5px 'IBM Plex Mono', monospace");
                    fill(ctx, &format!("rgba(237,234,226,{:.2})", 0.90 * fade));
                    let _ = ctx.fill_text(&moon.name.to_uppercase(), mx, my - 9.0);
                }
            }

            if let Ok(g) = ctx.create_radial_gradient(x, y, 0.0, x, y, r * 4.0) {
                let ga = if on { 0.53 } else { 0.27 };
                let _ = g.add_color_stop(0.0, &alpha(color, ga * fade));
                let _ = g.add_color_stop(1.0, &alpha(color, 0.0));
                ctx.set_fill_style_canvas_gradient(&g);
                ctx.begin_path();
                let _ = ctx.arc(x, y, r * 4.0, 0.0, TAU);
                ctx.fill();
            }

            fill(ctx, &alpha(color, fade));
            ctx.begin_path();
            let _ = ctx.arc(x, y, r, 0.0, TAU);
            ctx.fill();
            fill(ctx, &format!("rgba(255,255,255,{:.2})", 0.55 * fade));
            ctx.begin_path();
            let _ = ctx.arc(x - r * 0.3, y - r * 0.3, r * 0.35, 0.0, TAU);
            ctx.fill();

            if on {
                stroke(ctx, color);
                ctx.set_line_width(1.0);
                ctx.begin_path();
                let _ = ctx.arc(x, y, r + 8.0 + (sim.t as f64 * 3.0).sin() * 1.5, 0.0, TAU);
                ctx.stroke();
            }

            ctx.set_font(if on {
                "600 12px 'IBM Plex Mono', monospace"
            } else {
                "400 10.5px 'IBM Plex Mono', monospace"
            });
            let label_a = if on { 1.0 } else { (0.82 * fade).max(0.15) };
            fill(ctx, &if on {
                format!("rgba(237,234,226,{:.2})", label_a)
            } else {
                format!("rgba(150,160,184,{:.2})", label_a)
            });
            let _ = ctx.fill_text(&body.name.to_uppercase(), x, y - r - 11.0);
        }
        if !core_drawn {
            self.core(sim);
        }
    }
}

use wasm_bindgen::JsCast;
