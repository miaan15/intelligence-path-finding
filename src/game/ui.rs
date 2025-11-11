use macroquad::prelude::*;
use std::time::{Duration, Instant};

pub struct UIManager {
    font_size: f32,
    show_controls: bool,
    show_state: bool,
    timer_start: Option<Instant>,
    timer_elapsed: Option<Duration>,
    show_timer: bool,
}

impl UIManager {
    pub fn new(font_size: f32) -> Self {
        Self {
            font_size,
            show_controls: true,
            show_state: true,
            timer_start: None,
            timer_elapsed: None,
            show_timer: false,
        }
    }

    pub fn start_timer(&mut self) {
        self.timer_start = Some(Instant::now());
        self.timer_elapsed = None;
        self.show_timer = true;
    }

    pub fn stop_timer(&mut self) {
        if let Some(start_time) = self.timer_start {
            self.timer_elapsed = Some(start_time.elapsed());
            self.timer_start = None;
        }
    }

    pub fn get_elapsed_ms(&self) -> Option<f64> {
        if let Some(elapsed) = self.timer_elapsed {
            Some(elapsed.as_secs_f64() * 1000.0)
        } else if let Some(start_time) = self.timer_start {
            Some(start_time.elapsed().as_secs_f64() * 1000.0)
        } else {
            None
        }
    }

    pub fn reset_timer(&mut self) {
        self.timer_start = None;
        self.timer_elapsed = None;
        self.show_timer = false;
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

        // Timer display in top-right corner
        if self.show_timer {
            if let Some(elapsed_ms) = self.get_elapsed_ms() {
                let screen_top_right = camera.screen_to_world(vec2(screen_width(), 0.0));
                let offset = self.font_size * 1.0;
                let timer_text = format!("{:.2}ms", elapsed_ms);

                draw_text(
                    &timer_text,
                    screen_top_right.x - offset - timer_text.len() as f32 * self.font_size * 0.6,
                    screen_top_right.y + offset,
                    self.font_size,
                    YELLOW,
                );
            }
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
