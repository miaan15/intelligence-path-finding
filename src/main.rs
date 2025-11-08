// Import from the local library
use macroquad::prelude::*;
use pathfinding::game::game_manager::GameManager;
use pathfinding::game::grid_render::GridRenderer;
use pathfinding::game::ui_manager::UIManager;
use pathfinding::game::RenderConfig;
use pathfinding::world::grid::{GridMap, GridNodeValue};
use pathfinding::world::WorldConfig;

fn setup_grid(grid_map: &mut GridMap) {
    for x in 0..20 {
        grid_map.grid_mut().set(x, 0, GridNodeValue::Obstacle);
        grid_map.grid_mut().set(x, 14, GridNodeValue::Obstacle);
    }
    for y in 0..15 {
        grid_map.grid_mut().set(0, y, GridNodeValue::Obstacle);
        grid_map.grid_mut().set(19, y, GridNodeValue::Obstacle);
    }
    for y in 3..12 {
        grid_map.grid_mut().set(8, y, GridNodeValue::Obstacle);
        grid_map.grid_mut().set(12, y, GridNodeValue::Obstacle);
    }
    for x in 2..8 {
        grid_map.grid_mut().set(x, 7, GridNodeValue::Obstacle);
    }
    for x in 13..18 {
        grid_map.grid_mut().set(x, 7, GridNodeValue::Obstacle);
    }

    grid_map.grid_mut().set(5, 4, GridNodeValue::Obstacle);
    grid_map.grid_mut().set(6, 4, GridNodeValue::Obstacle);
    grid_map.grid_mut().set(14, 10, GridNodeValue::Obstacle);
    grid_map.grid_mut().set(15, 10, GridNodeValue::Obstacle);
    grid_map.grid_mut().set(16, 10, GridNodeValue::Obstacle);
}

#[macroquad::main("Path Finding Demo")]
async fn main() {
    let world_config = WorldConfig {
        grid_size: (20, 15),
        cell_size: 40.0,
    };

    let render_config = RenderConfig {
        background_color: DARKGRAY,
        obstacle_color: RED,
    };

    // ==================================================================
    let mut grid_map = GridMap::new(20, 15, world_config);
    setup_grid(&mut grid_map);

    let mut game_manager = {
        let grid_renderer = GridRenderer::new(render_config);
        GameManager::new(Box::new(grid_renderer))
    };

    let ui_manager = UIManager::new();

    // ==================================================================
    game_manager.get_renderer_mut().update_mesh(&grid_map);
    game_manager.update_camera_dimensions(&grid_map);

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        clear_background(BLACK);

        let (screen_width, screen_height) = game_manager.get_screen_dimensions();
        set_camera(&game_manager.setup_camera());

        game_manager.update();

        game_manager.get_renderer().draw();

        set_default_camera();

        ui_manager.draw_ui(
            screen_width,
            screen_height,
            game_manager.get_state(),
            game_manager.get_state_description(),
        );

        next_frame().await;
    }
}
