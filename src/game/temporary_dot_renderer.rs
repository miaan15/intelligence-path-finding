use macroquad::prelude::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct TemporaryDot {
    pub pos: Vec2,
    pub color: Color,
    pub radius: f32,
    pub created_at: Instant,
    pub lifetime: Duration,
}

pub struct TemporaryDotRenderer {
    dots: HashMap<u64, TemporaryDot>,
    next_id: u64,
}

impl TemporaryDotRenderer {
    pub fn new() -> Self {
        Self {
            dots: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn add_dot(&mut self, pos: Vec2, color: Color, radius: f32, time_seconds: f64) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let dot = TemporaryDot {
            pos,
            color,
            radius,
            created_at: Instant::now(),
            lifetime: Duration::from_secs_f64(time_seconds),
        };

        self.dots.insert(id, dot);
        id
    }

    pub fn remove_dot(&mut self, id: u64) -> bool {
        self.dots.remove(&id).is_some()
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.dots.retain(|_, dot| now.duration_since(dot.created_at) < dot.lifetime);
    }

    pub fn draw(&self) {
        let now = Instant::now();

        for dot in self.dots.values() {
            let age = now.duration_since(dot.created_at);
            let age_ratio = age.as_secs_f64() / dot.lifetime.as_secs_f64();

            if age_ratio < 1.0 {
                // Fade out based on age
                let alpha = 1.0 - age_ratio;
                let mut color = dot.color;
                color.a *= alpha as f32;

                draw_circle(dot.pos.x, dot.pos.y, dot.radius, color);
            }
        }
    }

    pub fn clear(&mut self) {
        self.dots.clear();
    }

    pub fn count(&self) -> usize {
        self.dots.len()
    }
}

// Static singleton instance
use std::sync::{Arc, Mutex, OnceLock};

static TEMPORARY_DOT_RENDERER: OnceLock<Arc<Mutex<TemporaryDotRenderer>>> = OnceLock::new();

/// Get the global TemporaryDotRenderer instance
pub fn get_temporary_dot_renderer() -> Arc<Mutex<TemporaryDotRenderer>> {
    TEMPORARY_DOT_RENDERER
        .get_or_init(|| Arc::new(Mutex::new(TemporaryDotRenderer::new())))
        .clone()
}

/// Convenience function to draw a temporary dot from anywhere
pub fn draw_temporary_dot(pos: Vec2, color: Color, radius: f32, time_seconds: f64) -> u64 {
    let renderer = get_temporary_dot_renderer();
    let mut renderer = renderer.lock().unwrap();
    renderer.add_dot(pos, color, radius, time_seconds)
}

/// Update all temporary dots (call this once per frame)
pub fn update_temporary_dots() {
    let renderer = get_temporary_dot_renderer();
    let mut renderer = renderer.lock().unwrap();
    renderer.update();
}

/// Draw all temporary dots (call this once per frame, after update)
pub fn draw_all_temporary_dots() {
    let renderer = get_temporary_dot_renderer();
    let renderer = renderer.lock().unwrap();
    renderer.draw();
}