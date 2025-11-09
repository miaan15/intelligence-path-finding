use macroquad::prelude::*;

pub struct CameraManager {
    camera: Camera2D,
}

impl CameraManager {
    pub fn new() -> Self {
        Self {
            camera: Camera2D {
                zoom: vec2(1.0, 1.0),
                target: vec2(0.0, 0.0),
                ..Default::default()
            },
        }
    }

    pub fn camera(&self) -> &Camera2D { &self.camera }

    pub fn set_zoom(&mut self, zoom: f32) { self.camera.zoom = vec2(zoom, zoom); }

    pub fn set_target(&mut self, x: f32, y: f32) { self.camera.target = vec2(x, y); }

    pub fn target_to_bound(&mut self, top_left: Vec2, bot_right: Vec2, zoom_factor: f32) {
        let center = Vec2::new((top_left.x + bot_right.x) / 2.0, (top_left.y + bot_right.y) / 2.0);

        let bound_width = (bot_right.x - top_left.x).abs();
        let bound_height = (bot_right.y - top_left.y).abs();

        let width_to_height = screen_width() / screen_height();

        let zoom_x = 1.0 / bound_width * 2.0;
        let zoom_y = 1.0 / bound_height * 2.0;
        let fit_zoom = zoom_x.max(zoom_y) * zoom_factor;

        self.camera.zoom = vec2(fit_zoom, fit_zoom * width_to_height);
        self.camera.target = center;
    }
}
