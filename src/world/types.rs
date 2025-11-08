use macroquad::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Quad {
    pub pos: Vec2,
    pub siz: Vec2,
}

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub root: Vec2,
    pub dir: Vec2,
}

#[derive(Debug, Clone, Copy)]
pub struct RayHitInfo {
    pub pt: Vec2,
    pub nor: Vec2,
    pub dist: f32,
}

pub fn quad(x: f32, y: f32, w: f32, h: f32) -> Quad {
    Quad {
        pos: Vec2::new(x, y),
        siz: Vec2::new(w, h),
    }
}
