use crate::game::game::GameState;
use macroquad::prelude::*;

pub struct UIManager {
    show_controls: bool,
    show_state: bool,
}

impl UIManager {
    pub fn new() -> Self {
        Self {
            show_controls: true,
            show_state: true,
        }
    }

    pub fn draw_ui(&self, screen_width: f32, screen_height: f32, _game_state: GameState, state_description: &str) {
        if self.show_controls {
            self.draw_controls(screen_width, screen_height);
        }

        if self.show_state {
            self.draw_current_state(state_description);
        }
    }

    fn draw_controls(&self, _screen_width: f32, screen_height: f32) {
        // Draw simple keyboard controls text in bottom left
        let x = 10.0;
        let y_start = screen_height - 120.0;
        let line_height = 18.0;

        let font_size = 18.0;
        draw_text("[1] Add Tile", x, y_start + line_height, font_size, LIGHTGRAY);
        draw_text("[2] Remove Tile", x, y_start + line_height * 2.0, font_size, LIGHTGRAY);
        draw_text("[3] Set Start", x, y_start + line_height * 3.0, font_size, LIGHTGRAY);
        draw_text("[4] Set End", x, y_start + line_height * 4.0, font_size, LIGHTGRAY);
        draw_text("[C] Cancel", x, y_start + line_height * 5.0, font_size, LIGHTGRAY);
    }

    fn draw_current_state(&self, state_description: &str) {
        draw_text(&format!("> {}", state_description), 10.0, 25.0, 20.0, WHITE);
    }

    pub fn toggle_controls(&mut self) { self.show_controls = !self.show_controls; }

    pub fn toggle_state(&mut self) { self.show_state = !self.show_state; }

    pub fn set_controls_visible(&mut self, visible: bool) { self.show_controls = visible; }

    pub fn set_state_visible(&mut self, visible: bool) { self.show_state = visible; }
}
