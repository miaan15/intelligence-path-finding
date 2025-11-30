// use crate::algorithm::a_star::AStarStrategy;
use crate::algorithm::aco::AcoStrategy;
use crate::algorithm::problem::Problem;
use crate::algorithm::pso::PsoStrategy;
use crate::algorithm::strategy::Strategy;
use crate::game::camera::CameraManager;
use crate::game::map_renderer::MapRenderer;
use crate::game::path_renderer::PathRenderer;
use crate::game::temporary_dot_renderer::{draw_all_temporary_dots, update_temporary_dots};
use crate::game::ui::UIManager;
use macroquad::prelude::*;
use std::sync::{Arc, mpsc};
use std::thread;

#[derive(Debug, Clone)]
pub struct RenderConfig {
    pub background_color: Color,
    pub obstacle_color: Color,
    pub pixel_per_unit: u32,
    pub font_size: f32,
    pub path_color: Color,
    pub path_thickness: f32,
    pub start_color: Color,
    pub end_color: Color,
    pub marker_radius: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    Idle,
    Loading,
    SetStart,
    SetEnd,
}

pub struct GameManager {
    state: GameState,
    start_pos: Option<Vec2>,
    end_pos: Option<Vec2>,
    render_config: RenderConfig,
    pathfinding_receiver: Option<mpsc::Receiver<(Option<Vec<Vec2>>, Option<Vec<Vec2>>)>>,
    grid_map: Arc<crate::world::grid::GridMap>,

    map_renderer: Box<MapRenderer>,
    path_renderer: Box<PathRenderer>,
    camera_manager: Box<CameraManager>,
    ui_manager: Box<UIManager>,
}

impl GameManager {
    pub fn new(
        map_renderer: Box<MapRenderer>,
        path_renderer: Box<PathRenderer>,
        camera_manager: Box<CameraManager>,
        ui_manager: Box<UIManager>,
        render_config: RenderConfig,
        grid_map: Arc<crate::world::grid::GridMap>,
    ) -> Self {
        Self {
            state: GameState::Idle,
            start_pos: None,
            end_pos: None,
            render_config,
            pathfinding_receiver: None,
            grid_map,
            map_renderer,
            path_renderer,
            camera_manager,
            ui_manager,
        }
    }

    pub fn get_state(&self) -> GameState {
        self.state
    }
    pub fn set_state(&mut self, state: GameState) {
        self.state = state;
    }

    pub fn map_renderer(&self) -> &MapRenderer {
        self.map_renderer.as_ref()
    }
    pub fn map_renderer_mut(&mut self) -> &mut MapRenderer {
        self.map_renderer.as_mut()
    }

    pub fn camera_manager(&self) -> &CameraManager {
        self.camera_manager.as_ref()
    }
    pub fn camera_manager_mut(&mut self) -> &mut CameraManager {
        self.camera_manager.as_mut()
    }

    pub fn ui_manager(&self) -> &UIManager {
        self.ui_manager.as_ref()
    }
    pub fn ui_manager_mut(&mut self) -> &mut UIManager {
        self.ui_manager.as_mut()
    }

    pub fn path_renderer(&self) -> &PathRenderer {
        self.path_renderer.as_ref()
    }
    pub fn path_renderer_mut(&mut self) -> &mut PathRenderer {
        self.path_renderer.as_mut()
    }

    pub fn start_pos(&self) -> Option<Vec2> {
        self.start_pos
    }

    pub fn end_pos(&self) -> Option<Vec2> {
        self.end_pos
    }

    fn start_pathfinding(&mut self) {
        if let (Some(start), Some(end)) = (self.start_pos, self.end_pos) {
            self.set_state(GameState::Loading);
            self.ui_manager_mut().start_timer();

            let (sender, receiver) = mpsc::channel();
            self.pathfinding_receiver = Some(receiver);

            let grid_map = Arc::clone(&self.grid_map);

            thread::spawn(move || {
                let problem = Problem::new(grid_map, start, end);
                // let path = AStarStrategy {}.path_finding(&problem);
                let aco_path = AcoStrategy {
                    node_dist: 100.0 / 2.0,
                    alpha: 1.0,
                    beta: 5.0,
                    deposit_constant: 2000.0,
                    evaporation: 0.2,
                    init_pheromone: 0.05,
                    min_ant_count: 1000,
                    max_ant_try: 500,
                }
                .path_finding(&problem)
                .or_else(|| {
                    std::println!("================================== \n== NOT FOUND PATH");
                    None
                });

                let pso_path = if let Some(ref aco_solution) = aco_path {
                    Some(
                        PsoStrategy {
                            init_random_offset: 20.0,
                            swarms_count: 100,
                            inertia_weight: 0.9,
                            local_factor: 1.0,
                            global_factor: 1.0,
                            iterate_count: 100,
                            max_velocity: 1000.0,
                        }
                        .upgrade_path(&problem, aco_solution),
                    )
                } else {
                    None
                };

                let _ = sender.send((aco_path, pso_path));
            });
        }
    }

    pub fn update(&mut self) {
        // Check if we have a pending pathfinding result
        if let Some(receiver) = &mut self.pathfinding_receiver {
            if let Ok((aco_path, pso_path)) = receiver.try_recv() {
                if let Some(aco) = aco_path {
                    self.path_renderer_mut().set_aco_path(aco);
                }
                if let Some(pso) = pso_path {
                    self.path_renderer_mut().set_pso_path(pso);
                }
                self.ui_manager_mut().stop_timer();
                self.pathfinding_receiver = None;
                self.set_state(GameState::Idle);
            }
        }

        // Update temporary dots
        update_temporary_dots();

        self.handle_input();

        self.render();
    }

    fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::G) {
            if self.start_pos.is_some() && self.end_pos.is_some() {
                self.start_pathfinding();
            }
        }
        if is_key_pressed(KeyCode::S) {
            self.set_state(GameState::SetStart);
        }
        if is_key_pressed(KeyCode::E) {
            self.set_state(GameState::SetEnd);
        }
        if is_key_pressed(KeyCode::C) {
            self.set_state(GameState::Idle);
            self.path_renderer_mut().unset_paths();
            self.ui_manager_mut().reset_timer();
        }

        // Handle mouse clicks for setting start and end positions
        let roundup = 25.0;
        if is_mouse_button_pressed(MouseButton::Left) {
            match self.state {
                GameState::SetStart => {
                    let mut world_pos = self.camera_manager.screen_to_world(mouse_position());
                    world_pos = (world_pos / roundup).floor() * roundup;
                    self.start_pos = Some(world_pos);
                    self.set_state(GameState::Idle);
                }
                GameState::SetEnd => {
                    let mut world_pos = self.camera_manager.screen_to_world(mouse_position());
                    world_pos = (world_pos / roundup).floor() * roundup;
                    self.end_pos = Some(world_pos);
                    self.set_state(GameState::Idle);
                }
                _ => {}
            }
        }
    }

    fn render(&self) {
        self.map_renderer.draw();
        self.ui_manager
            .draw(&self.camera_manager.camera(), self.get_state_description());
        self.path_renderer.draw();

        // Draw all temporary dots
        draw_all_temporary_dots();

        // Draw start and end position markers
        if let Some(start_pos) = self.start_pos {
            draw_circle(
                start_pos.x,
                start_pos.y,
                self.render_config.marker_radius,
                self.render_config.start_color,
            );
        }

        if let Some(end_pos) = self.end_pos {
            draw_circle(
                end_pos.x,
                end_pos.y,
                self.render_config.marker_radius,
                self.render_config.end_color,
            );
        }
    }

    pub fn get_state_description(&self) -> &'static str {
        match self.state {
            GameState::Idle => "Idle",
            GameState::Loading => "Loading...",
            GameState::SetStart => "Set Start",
            GameState::SetEnd => "Set End",
        }
    }
}
