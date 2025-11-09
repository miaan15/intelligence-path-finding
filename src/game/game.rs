use crate::game::camera::CameraManager;
use crate::game::map_renderer::MapRenderer;
use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct RenderConfig {
    pub background_color: Color,
    pub obstacle_color: Color,
}

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
    state: GameState,

    map_renderer: Box<MapRenderer>,
    camera_manager: Box<CameraManager>,
}

impl GameManager {
    pub fn new(map_renderer: Box<MapRenderer>, camera_manager: Box<CameraManager>) -> Self {
        Self {
            state: GameState::Idle,
            map_renderer,
            camera_manager,
        }
    }

    pub fn get_state(&self) -> GameState { self.state }
    pub fn set_state(&mut self, state: GameState) { self.state = state; }

    pub fn map_renderer(&self) -> &MapRenderer { self.map_renderer.as_ref() }
    pub fn map_renderer_mut(&mut self) -> &mut MapRenderer { self.map_renderer.as_mut() }

    pub fn camera_manager(&self) -> &CameraManager { self.camera_manager.as_ref() }
    pub fn camera_manager_mut(&mut self) -> &mut CameraManager { self.camera_manager.as_mut() }

    pub fn update(&mut self) {
        self.handle_input();

        self.render();
    }

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
    }

    fn render(&self) {
        if let Some(mesh) = self.map_renderer().mesh() {
            draw_mesh(mesh);
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
