use macroquad::prelude::*;

pub struct UIManager {
    font_size: f32,
    show_controls: bool,
    show_state: bool,
}

impl UIManager {
    pub fn new(font_size: f32) -> Self {
        Self {
            font_size,
            show_controls: true,
            show_state: true,
        }
    }

    pub fn draw(&self, camera: &Camera2D, state_description: &str) {
        let screen_top_left = camera.screen_to_world(vec2(0.0, 0.0));
        let screen_bottom_left = camera.screen_to_world(vec2(0.0, screen_height()));

        if self.show_state {
            let offset = self.font_size * 1.0;
            draw_text(
                &format!("> {}", state_description),
                screen_top_left.x + offset,
                screen_top_left.y + offset,
                self.font_size,
                WHITE,
            );
        }

        if self.show_controls {
            let offset = self.font_size * 1.0;
            let line_height = self.font_size * 1.2;
            let line_width = self.font_size * 8.0;
            let x = screen_bottom_left.x + offset;
            let y = screen_bottom_left.y - 1.0 * line_height - offset;

            draw_text(
                "[S] Set Start",
                x + line_width * 0.0,
                y + line_height * 0.0,
                self.font_size,
                WHITE,
            );
            draw_text(
                "[E] Set End",
                x + line_width * 0.0,
                y + line_height * 1.0,
                self.font_size,
                WHITE,
            );
            draw_text(
                "[G] GO GO",
                x + line_width * 1.0,
                y + line_height * 0.0,
                self.font_size,
                WHITE,
            );
            draw_text(
                "[C] Cancel",
                x + line_width * 1.0,
                y + line_height * 1.0,
                self.font_size,
                WHITE,
            );
        }
    }

    pub fn toggle_controls(&mut self) {
        self.show_controls = !self.show_controls;
    }
    pub fn toggle_state(&mut self) {
        self.show_state = !self.show_state;
    }
    pub fn set_controls_visible(&mut self, visible: bool) {
        self.show_controls = visible;
    }
    pub fn set_state_visible(&mut self, visible: bool) {
        self.show_state = visible;
    }
}
