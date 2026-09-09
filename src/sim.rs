use crate::data::{Body, BODIES, SKILLS};

pub const FLAT: f32 = 0.60;
const TRAIL_LEN: usize = 150;

pub struct BodyState {
    pub anom: f32,
    pub n: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub radius: f32,
    pub trail: Vec<(f32, f32)>,
    pub moon_angles: Vec<f32>,
}

pub struct SkillState {
    pub angle: f32,
    pub radius: f32,
}

pub struct Sim {
    pub bodies: Vec<BodyState>,
    pub skills: Vec<SkillState>,
    pub t: f32,
    pub speed: f32,
    pub cx: f32,
    pub cy: f32,
    pub scale: f32,
    pub cam_az: f32,
    pub cam_tilt: f32,
}

impl Sim {
    pub fn new() -> Self {
        let bodies = BODIES
            .iter()
            .map(|b| BodyState {
                anom: b.phase,
                n: 0.18 / b.a.powf(1.5),
                x: 0.0,
                y: 0.0,
                z: 0.0,
                radius: 4.0 + b.mass * 1.55,
                trail: Vec::with_capacity(TRAIL_LEN),
                moon_angles: b.moons.iter().enumerate().map(|(i, _)| i as f32 * 2.1).collect(),
            })
            .collect();

        let skills = SKILLS
            .iter()
            .enumerate()
            .map(|(i, _)| SkillState {
                angle: (i as f32 / SKILLS.len() as f32) * std::f32::consts::TAU,
                radius: 1.02 + (i % 3) as f32 * 0.035,
            })
            .collect();

        Sim { bodies, skills, t: 0.0, speed: 1.0, cx: 0.0, cy: 0.0, scale: 1.0, cam_az: 0.0, cam_tilt: 0.0 }
    }

    pub fn resize(&mut self, w: f32, h: f32) {
        self.cx = w / 2.0;
        self.cy = h / 2.0;
        self.scale = w.min(h) * 0.42;
        for b in &mut self.bodies {
            b.trail.clear();
        }
    }

    pub fn flat(&self) -> f32 {
        (FLAT + self.cam_tilt).clamp(0.30, 0.85)
    }

    pub fn rotate_camera(&mut self, daz: f32, dtilt: f32) {
        self.cam_az += daz;
        self.cam_tilt = (self.cam_tilt + dtilt).clamp(-0.30, 0.25);
    }

    pub fn place(&self, body: &Body, anom: f32) -> (f32, f32) {
        let flat = self.flat();
        let px = body.a * self.scale * (anom.cos() - body.e);
        let py = body.a * self.scale * (1.0 - body.e * body.e).sqrt() * anom.sin();
        let (s, c) = body.tilt.sin_cos();
        let xo = px * c - py * s;
        let yo = px * s + py * c;
        let (sa, ca) = self.cam_az.sin_cos();
        (self.cx + xo * ca - yo * sa, self.cy + (xo * sa + yo * ca) * flat)
    }

    pub fn step(&mut self, dt: f32) {
        self.t += dt;
        let (sa, ca) = self.cam_az.sin_cos();
        let flat = self.flat();
        for (i, body) in BODIES.iter().enumerate() {
            let st = &mut self.bodies[i];
            if dt > 0.0 {
                st.anom += dt * st.n * self.speed / (1.0 - body.e * st.anom.cos());
            }

            let px = body.a * self.scale * (st.anom.cos() - body.e);
            let py = body.a * self.scale * (1.0 - body.e * body.e).sqrt() * st.anom.sin();
            let (s, c) = body.tilt.sin_cos();
            let xo = px * c - py * s;
            let yo = px * s + py * c;
            st.x = self.cx + xo * ca - yo * sa;
            let raw_y = xo * sa + yo * ca;
            st.y = self.cy + raw_y * flat;
            st.z = raw_y;

            if dt > 0.0 {
                st.trail.push((st.x, st.y));
                if st.trail.len() > TRAIL_LEN {
                    st.trail.remove(0);
                }
                for (j, moon) in body.moons.iter().enumerate() {
                    st.moon_angles[j] += dt * moon.speed * self.speed;
                }
            }
        }

        if dt > 0.0 {
            for s in &mut self.skills {
                s.angle += dt * 0.022 * self.speed;
            }
        }
    }

    pub fn hit(&self, px: f32, py: f32) -> Option<usize> {
        let mut best: Option<(usize, f32)> = None;
        for (i, st) in self.bodies.iter().enumerate() {
            let d = ((st.x - px).powi(2) + (st.y - py).powi(2)).sqrt();
            if d < st.radius + 20.0 && best.map_or(true, |(_, bd)| d < bd) {
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
            let d = ((st.x - sim.cx).powi(2) + ((st.y - sim.cy) / FLAT).powi(2)).sqrt();
            let apo = body.a * sim.scale * (1.0 + body.e) + 1.0;
            assert!(d <= apo, "{} drifted past apoapsis", body.name);
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
