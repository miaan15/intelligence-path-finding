use macroquad::prelude::*;
use pathfinding::game::camera::*;
use pathfinding::game::game::*;
use pathfinding::game::map_renderer::*;
use pathfinding::game::ui::*;
use pathfinding::world::grid::*;
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
    grid_map.grid_mut().set(2, 12, GridNodeValue::Obstacle);

    grid_map.grid_mut().set(5, 4, GridNodeValue::Obstacle);
    grid_map.grid_mut().set(6, 4, GridNodeValue::Obstacle);
    grid_map.grid_mut().set(14, 10, GridNodeValue::Obstacle);
    grid_map.grid_mut().set(15, 10, GridNodeValue::Obstacle);
    grid_map.grid_mut().set(16, 10, GridNodeValue::Obstacle);
}

#[macroquad::main("Path Finding Demo")]
async fn main() {
    let world_config = WorldConfig {
        grid_size: (20, 20),
        cell_size: 40.0,
    };

    let render_config = RenderConfig {
        background_color: DARKGRAY,
        obstacle_color: RED,
        pixel_per_unit: 100,
    };

    // ==================================================================
    let mut grid_map = GridMap::new(world_config.clone());
    setup_grid(&mut grid_map);

    let mut game_manager = {
        let mut map_renderer = MapRenderer::new(render_config);
        map_renderer.reset_mesh(&grid_map);

        let mut camera_manager = CameraManager::new();
        camera_manager.target_to_bound(
            vec2(0.0, 0.0),
            vec2(map_renderer.max_bound().x, map_renderer.max_bound().y),
            0.5,
        );

        let ui_manager = UIManager::new(75.0);

        GameManager::new(Box::new(map_renderer), Box::new(camera_manager), Box::new(ui_manager))
    };

    // ==================================================================
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        clear_background(BLACK);

        set_camera(game_manager.camera_manager().camera());

        game_manager.update();

        set_default_camera();

        next_frame().await;
    }
}
