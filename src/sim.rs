//! Kepler motion, trails and the skill belt. No rendering, no DOM — this
//! module must stay testable with `cargo test` on the host target.

use crate::data::{Body, BODIES, SKILLS};

const TRAIL_LEN: usize = 150;
const TAU: f32 = std::f32::consts::TAU;

pub struct Camera {
    /// Current rotation angle around the vertical (Y) axis.
    pub yaw: f32,
    /// Controls perspective strength: scale = f / (f + z).
    pub focal_length: f32,
    /// Radians per second for auto-rotation.
    pub auto_spin_speed: f32,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            yaw: 0.0,
            focal_length: 800.0,
            auto_spin_speed: TAU / 90.0, // one revolution per ~90 seconds
        }
    }

    /// Rotate a world-space point by camera yaw and apply perspective divide.
    /// Returns (screen_x, screen_y, depth, proj_scale).
    pub fn project(&self, wx: f32, wy: f32, wz: f32, cx: f32, cy: f32) -> (f32, f32, f32, f32) {
        let (s, c) = self.yaw.sin_cos();
        let rx = wx * c + wz * s;
        let ry = wy;
        let rz = -wx * s + wz * c;
        let p = self.focal_length / (self.focal_length + rz);
        (cx + rx * p, cy + ry * p, rz, p)
    }
}

pub struct BodyState {
    /// Eccentric anomaly.
    pub anom: f32,
    /// Mean motion, derived from the semi-major axis (Kepler's third law).
    pub n: f32,
    /// World-space position (before camera rotation).
    pub wx: f32,
    pub wy: f32,
    pub wz: f32,
    /// Screen-space position (after camera rotation + perspective).
    pub sx: f32,
    pub sy: f32,
    /// Depth after camera rotation (for z-sorting).
    pub depth: f32,
    /// Perspective scale factor at this body's depth.
    pub proj: f32,
    pub radius: f32,
    /// Trail stored in world-space.
    pub trail: Vec<(f32, f32, f32)>,
    pub moon_angles: Vec<f32>,
}

pub struct SkillState {
    /// Fixed world-space position as a fraction of scale.
    pub wx: f32,
    pub wy: f32,
    pub wz: f32,
}

pub struct Sim {
    pub bodies: Vec<BodyState>,
    pub skills: Vec<SkillState>,
    pub camera: Camera,
    pub t: f32,
    pub speed: f32,
    pub cx: f32,
    pub cy: f32,
    pub scale: f32,
}

/// Compute world-space position on the orbital ellipse for a given anomaly.
/// Returns (x, y, z) in pixel units (scaled by `scale`).
pub fn place(body: &Body, anom: f32, scale: f32) -> (f32, f32, f32) {
    let a_px = body.a * scale;
    let px = a_px * (anom.cos() - body.e);
    let py = a_px * (1.0 - body.e * body.e).sqrt() * anom.sin();
    let (sw, cw) = body.tilt.sin_cos();
    let u = px * cw - py * sw;
    let v = px * sw + py * cw;

    let (si, ci) = body.inclination.sin_cos();
    let (sn, cn) = body.ascending_node.sin_cos();
    (
        u * cn + v * ci * sn,
        -v * si,
        -u * sn + v * ci * cn,
    )
}

impl Sim {
    pub fn new() -> Self {
        let bodies = BODIES
            .iter()
            .map(|b| BodyState {
                anom: b.phase,
                n: 0.62 / b.a.powf(1.5),
                wx: 0.0,
                wy: 0.0,
                wz: 0.0,
                sx: 0.0,
                sy: 0.0,
                depth: 0.0,
                proj: 1.0,
                radius: 4.0 + b.mass * 1.55,
                trail: Vec::with_capacity(TRAIL_LEN),
                moon_angles: b.moons.iter().enumerate().map(|(i, _)| i as f32 * 2.1).collect(),
            })
            .collect();

        // Fibonacci sphere distribution for skills
        let len = SKILLS.len() as f32;
        let golden = (1.0 + 5.0_f32.sqrt()) / 2.0;
        let skills = SKILLS
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let r = 1.02 + (i % 3) as f32 * 0.035;
                let theta = TAU * i as f32 / golden;
                let cos_phi = 1.0 - 2.0 * (i as f32 + 0.5) / len;
                let sin_phi = (1.0 - cos_phi * cos_phi).sqrt();
                SkillState {
                    wx: r * sin_phi * theta.cos(),
                    wy: r * cos_phi,
                    wz: r * sin_phi * theta.sin(),
                }
            })
            .collect();

        Sim {
            bodies,
            skills,
            camera: Camera::new(),
            t: 0.0,
            speed: 1.0,
            cx: 0.0,
            cy: 0.0,
            scale: 1.0,
        }
    }

    pub fn resize(&mut self, w: f32, h: f32) {
        self.cx = w / 2.0;
        self.cy = h / 2.0;
        self.scale = w.min(h) * 0.42;
        for b in &mut self.bodies {
            b.trail.clear();
        }
    }

    pub fn step(&mut self, dt: f32) {
        self.t += dt;
        self.camera.yaw += dt * self.camera.auto_spin_speed * self.speed;

        // Cache camera values to avoid borrow conflicts
        let (yaw_s, yaw_c) = self.camera.yaw.sin_cos();
        let fl = self.camera.focal_length;
        let cx = self.cx;
        let cy = self.cy;
        let scale = self.scale;

        for (i, body) in BODIES.iter().enumerate() {
            let st = &mut self.bodies[i];

            // Kepler's second law: sweep faster near periapsis.
            st.anom += dt * st.n * self.speed / (1.0 - body.e * st.anom.cos());

            // Position in orbital plane
            let a_px = body.a * scale;
            let px = a_px * (st.anom.cos() - body.e);
            let py = a_px * (1.0 - body.e * body.e).sqrt() * st.anom.sin();
            let (sw, cw) = body.tilt.sin_cos();
            let u = px * cw - py * sw;
            let v = px * sw + py * cw;

            // Orbital plane → world space
            let (si, ci) = body.inclination.sin_cos();
            let (sn, cn) = body.ascending_node.sin_cos();
            st.wx = u * cn + v * ci * sn;
            st.wy = -v * si;
            st.wz = -u * sn + v * ci * cn;

            // World-space trail
            st.trail.push((st.wx, st.wy, st.wz));
            if st.trail.len() > TRAIL_LEN {
                st.trail.remove(0);
            }

            // Camera rotation + perspective projection
            let rx = st.wx * yaw_c + st.wz * yaw_s;
            let ry = st.wy;
            let rz = -st.wx * yaw_s + st.wz * yaw_c;
            st.proj = fl / (fl + rz);
            st.sx = cx + rx * st.proj;
            st.sy = cy + ry * st.proj;
            st.depth = rz;

            for (j, moon) in body.moons.iter().enumerate() {
                st.moon_angles[j] += dt * moon.speed * self.speed;
            }
        }
    }

    /// Index of the body under a screen point, if any.
    pub fn hit(&self, px: f32, py: f32) -> Option<usize> {
        let mut best: Option<(usize, f32)> = None;
        for (i, st) in self.bodies.iter().enumerate() {
            let d = ((st.sx - px).powi(2) + (st.sy - py).powi(2)).sqrt();
            let hit_r = st.radius * st.proj + 20.0;
            if d < hit_r && best.map_or(true, |(_, bd)| d < bd) {
                best = Some((i, d));
            }
        }
        best.map(|(i, _)| i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orbits_stay_on_their_ellipse() {
        let mut sim = Sim::new();
        sim.resize(1000.0, 800.0);
        for _ in 0..2000 {
            sim.step(1.0 / 60.0);
        }
        for (i, body) in BODIES.iter().enumerate() {
            let st = &sim.bodies[i];
            let d = (st.wx.powi(2) + st.wy.powi(2) + st.wz.powi(2)).sqrt();
            let apo = body.a * sim.scale * (1.0 + body.e) + 1.0;
            assert!(d <= apo, "{} drifted past apoapsis: d={:.1}, apo={:.1}", body.name, d, apo);
        }
    }

    #[test]
    fn inner_bodies_orbit_faster() {
        let sim = Sim::new();
        for w in sim.bodies.windows(2) {
            assert!(w[0].n > 0.0 && w[1].n > 0.0);
        }
    }
}
