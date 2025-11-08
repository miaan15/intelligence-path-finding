use crate::game::grid_render::GridRenderer;
use crate::world::grid::GridMap;
use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    Idle,
    Loading,
    AddTile,
    RemoveTile,
    SetStart,
    SetEnd,
}

pub struct GameManager {
    map_renderer: Box<GridRenderer>,
    state: GameState,
    camera_zoom: f32,
    screen_width: f32,
    screen_height: f32,
}

impl GameManager {
    pub fn new(map: Box<GridRenderer>) -> Self {
        Self {
            map_renderer: map,
            state: GameState::Idle,
            camera_zoom: 1.0,
            screen_width: 0.0,
            screen_height: 0.0,
        }
    }

    pub fn get_state(&self) -> GameState { self.state }
    pub fn set_state(&mut self, state: GameState) { self.state = state; }

    pub fn get_renderer(&self) -> &GridRenderer { self.map_renderer.as_ref() }
    pub fn get_renderer_mut(&mut self) -> &mut GridRenderer { self.map_renderer.as_mut() }

    pub fn update_camera_dimensions(&mut self, grid_map: &GridMap) {
        self.screen_width = grid_map.width() as f32 * grid_map.cell_size();
        self.screen_height = grid_map.height() as f32 * grid_map.cell_size();
    }

    pub fn get_screen_dimensions(&self) -> (f32, f32) { (self.screen_width, self.screen_height) }

    pub fn setup_camera(&self) -> Camera2D {
        Camera2D {
            target: vec2(self.screen_width / 2.0, self.screen_height / 2.0),
            zoom: vec2(1.0 / self.screen_width, -1.0 / self.screen_height) * self.camera_zoom,
            ..Default::default()
        }
    }

    pub fn update(&mut self) { self.handle_input(); }

    fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::Key1) {
            self.set_state(GameState::AddTile);
        }
        if is_key_pressed(KeyCode::Key2) {
            self.set_state(GameState::RemoveTile);
        }
        if is_key_pressed(KeyCode::Key3) {
            self.set_state(GameState::SetStart);
        }
        if is_key_pressed(KeyCode::Key4) {
            self.set_state(GameState::SetEnd);
        }
        if is_key_pressed(KeyCode::C) {
            self.set_state(GameState::Idle);
        }

        // Camera zoom controls
        if is_key_down(KeyCode::Up) {
            self.camera_zoom *= 1.02;
        }
        if is_key_down(KeyCode::Down) {
            self.camera_zoom /= 1.02;
        }
    }

    pub fn get_state_description(&self) -> &'static str {
        match self.state {
            GameState::Idle => "Idle",
            GameState::Loading => "Loading...",
            GameState::AddTile => "Add obstacles",
            GameState::RemoveTile => "Remove obstacles",
            GameState::SetStart => "Set start",
            GameState::SetEnd => "Set end",
        }
    }
}
