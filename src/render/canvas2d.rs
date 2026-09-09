use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use super::{Renderer, Scene};
use crate::data::{BODIES, CORE_PROJECTS, SKILLS, SKILL_COLORS};

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

    fn core(&self, sim: &crate::sim::Sim, scene: &Scene) {
        let ctx = &self.ctx;
        let (cx, cy) = (sim.cx as f64, sim.cy as f64);
        let pulse = 1.0 + (sim.t as f64 * 1.2).sin() * 0.08;
        let r = sim.core_radius as f64 * pulse;
        let flat = sim.flat() as f64;
        let zoomed = scene.zoom_level >= 1;

        // Radial glow — pulses with the core
        let glow_r = r * (4.0 + (sim.t as f64 * 1.2).sin() * 0.5);
        if let Ok(g) = ctx.create_radial_gradient(cx, cy, 0.0, cx, cy, glow_r) {
            let glow_a = 0.18 + (sim.t as f64 * 1.2).sin() * 0.06;
            let _ = g.add_color_stop(0.0, &format!("rgba(237,234,226,{:.2})", glow_a));
            let _ = g.add_color_stop(1.0, "rgba(237,234,226,0.0)");
            ctx.set_fill_style_canvas_gradient(&g);
            ctx.begin_path();
            let _ = ctx.arc(cx, cy, glow_r, 0.0, TAU);
            ctx.fill();
        }

        // Core body
        fill(ctx, "rgba(237,234,226,0.70)");
        ctx.begin_path();
        let _ = ctx.arc(cx, cy, r, 0.0, TAU);
        ctx.fill();

        // Specular
        fill(ctx, "rgba(255,255,255,0.35)");
        ctx.begin_path();
        let _ = ctx.arc(cx - r * 0.3, cy - r * 0.3, r * 0.3, 0.0, TAU);
        ctx.fill();

        // Name label
        ctx.set_text_align("center");
        ctx.set_font("600 11px 'IBM Plex Mono', monospace");
        fill(ctx, "rgba(237,234,226,0.95)");
        let _ = ctx.fill_text("VEDANTDEV KATYAYAN", cx, cy + r + 18.0);

        // Core projects (moons of the core)
        for (i, cp) in CORE_PROJECTS.iter().enumerate() {
            let cps = &sim.core_projects[i];
            let orbit_r = (cp.dist * sim.scale) as f64;
            let ang = cps.angle as f64;
            let px = cx + ang.cos() * orbit_r;
            let py = cy + ang.sin() * orbit_r * flat;
            let proj_color = cp.kind.color();

            if zoomed {
                // Full rendering at zoom level 1
                let pr = cps.radius as f64;

                // Body
                fill(ctx, proj_color);
                ctx.begin_path();
                let _ = ctx.arc(px, py, pr, 0.0, TAU);
                ctx.fill();

                // Specular
                fill(ctx, "rgba(255,255,255,0.45)");
                ctx.begin_path();
                let _ = ctx.arc(px - pr * 0.3, py - pr * 0.3, pr * 0.3, 0.0, TAU);
                ctx.fill();

                // Label
                ctx.set_font("600 11px 'IBM Plex Mono', monospace");
                ctx.set_text_align("center");
                fill(ctx, "rgba(237,234,226,0.95)");
                let _ = ctx.fill_text(&cp.name.to_uppercase(), px, py - pr - 8.0);

                // Sub-moons (like Device mesh)
                for (j, moon) in cp.moons.iter().enumerate() {
                    let mr = (moon.dist * sim.scale) as f64;
                    let mang = cps.moon_angles[j] as f64;
                    let mx = px + mang.cos() * mr;
                    let my = py + mang.sin() * mr * flat;

                    fill(ctx, &alpha(proj_color, 0.85));
                    ctx.begin_path();
                    let _ = ctx.arc(mx, my, moon.size as f64, 0.0, TAU);
                    ctx.fill();

                    ctx.set_font("400 9px 'IBM Plex Mono', monospace");
                    fill(ctx, "rgba(237,234,226,0.80)");
                    let _ = ctx.fill_text(&moon.name.to_uppercase(), mx, my - 8.0);
                }
            } else {
                // Tiny dots at zoom level 0
                fill(ctx, &alpha(proj_color, 0.6));
                ctx.begin_path();
                let _ = ctx.arc(px, py, 2.5, 0.0, TAU);
                ctx.fill();
            }
        }
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

        // zoom is 0.0 (full view) to 1.0 (projects only). Used as fade for employment.
        let fade = 1.0 - scene.zoom;

        self.background();

        // Skill belt — always visible
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
                ctx.set_font("500 10px 'IBM Plex Mono', monospace");
                ctx.set_text_align("center");
                fill(ctx, "rgba(237,234,226,0.98)");
                let _ = ctx.fill_text(&sk.name.to_uppercase(), x as f64, (y - 10.0) as f64);
            }
        }

        // Orbit paths — fade with employment
        if scene.show_orbits && fade > 0.01 {
            for (i, body) in BODIES.iter().enumerate() {
                let on = scene.selected == Some(i);
                let base_a = if on { 0.33 } else { 0.09 };
                let color = if on {
                    alpha(body.kind.color(), base_a * fade)
                } else {
                    format!("rgba(150,170,210,{:.2})", base_a * fade)
                };
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

        // Employment bodies — fade out
        if fade > 0.01 {
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
                    self.core(sim, scene);
                    core_drawn = true;
                }

                let on = scene.selected == Some(i) || scene.hovered == Some(i);
                let color = body.kind.color();
                let depth_norm = (st.z / sim.scale).clamp(-1.0, 1.0);
                let df = 1.0 - depth_norm.max(0.0) * 0.6;
                let size_scale = 1.0 - depth_norm.max(0.0) * 0.3;
                let (x, y) = (st.x as f64, st.y as f64);
                let r = (st.radius * size_scale) as f64;
                let f = df * fade; // combine depth fade with zoom fade

                for (j, moon) in body.moons.iter().enumerate() {
                    let mr = (moon.dist * sim.scale) as f64;
                    let ang = st.moon_angles[j] as f64;
                    let mx = x + ang.cos() * mr;
                    let my = y + ang.sin() * mr * sim.flat() as f64;
                    stroke(ctx, &format!("rgba(150,170,210,{:.2})", if on { 0.20 } else { 0.07 } * fade));
                    ctx.set_line_width(1.0);
                    ctx.begin_path();
                    let _ = ctx.ellipse(x, y, mr, mr * sim.flat() as f64, 0.0, 0.0, TAU);
                    ctx.stroke();
                    fill(ctx, &alpha(color, 0.85 * f));
                    ctx.begin_path();
                    let _ = ctx.arc(mx, my, moon.size as f64, 0.0, TAU);
                    ctx.fill();
                    if on {
                        ctx.set_font("400 9.5px 'IBM Plex Mono', monospace");
                        fill(ctx, &format!("rgba(237,234,226,{:.2})", 0.90 * f));
                        let _ = ctx.fill_text(&moon.name.to_uppercase(), mx, my - 9.0);
                    }
                }

                fill(ctx, &alpha(color, f));
                ctx.begin_path();
                let _ = ctx.arc(x, y, r, 0.0, TAU);
                ctx.fill();
                fill(ctx, &format!("rgba(255,255,255,{:.2})", 0.55 * f));
                ctx.begin_path();
                let _ = ctx.arc(x - r * 0.3, y - r * 0.3, r * 0.35, 0.0, TAU);
                ctx.fill();

                if on && fade > 0.3 {
                    stroke(ctx, &alpha(color, fade));
                    ctx.set_line_width(1.0);
                    ctx.begin_path();
                    let _ = ctx.arc(x, y, r + 8.0 + (sim.t as f64 * 3.0).sin() * 1.5, 0.0, TAU);
                    ctx.stroke();
                }

                if fade > 0.1 {
                    ctx.set_font(if on {
                        "600 12px 'IBM Plex Mono', monospace"
                    } else {
                        "400 10.5px 'IBM Plex Mono', monospace"
                    });
                    let label_a = if on { fade } else { (0.82 * df).max(0.15) * fade };
                    fill(ctx, &if on {
                        format!("rgba(237,234,226,{:.2})", label_a)
                    } else {
                        format!("rgba(150,160,184,{:.2})", label_a)
                    });
                    let _ = ctx.fill_text(&body.name.to_uppercase(), x, y - r - 11.0);
                }
            }
            if !core_drawn {
                self.core(sim, scene);
            }
        } else {
            // Fully zoomed in — only draw core + projects
            self.core(sim, scene);
        }
    }
}

use wasm_bindgen::JsCast;
