//! Rendering seam. Canvas2D is the baseline; the wgpu backend slots in behind
//! the same trait when the dust layer arrives. Nothing outside this module may
//! touch a drawing API.

use crate::sim::Sim;

pub mod canvas2d;

pub struct Scene<'a> {
    pub sim: &'a Sim,
    pub selected: Option<usize>,
    pub hovered: Option<usize>,
    pub pointer: (f32, f32),
    pub show_orbits: bool,
    pub zoom: f32,
    pub zoom_level: u8,
}

pub trait Renderer {
    fn resize(&mut self, w: f32, h: f32, dpr: f32);
    fn draw(&mut self, scene: &Scene);
}
