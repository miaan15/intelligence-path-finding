use macroquad::prelude::*;
use pathfinding::game::camera::*;
use pathfinding::game::game::*;
use pathfinding::game::map_renderer::*;
use pathfinding::game::path_renderer::*;
use pathfinding::game::ui::*;
use pathfinding::gridmaker::setup_grid;
use pathfinding::world::WorldConfig;
use pathfinding::world::grid::*;
use std::sync::Arc;

fn window_conf() -> Conf {
    Conf {
        window_title: "T1 vo dich".to_string(),
        window_width: 800,
        window_height: 900,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let world_config = WorldConfig {
        grid_size: (25, 25),
        cell_size: 100.0,
    };

    let render_config = RenderConfig {
        background_color: DARKGRAY,
        obstacle_color: RED,
        pixel_per_unit: 100,
        font_size: 100.0,
        path_color: YELLOW,
        path_thickness: 10.0,
        start_color: GREEN,
        end_color: BLUE,
        marker_radius: 20.0,
    };

    // ==================================================================
    let mut grid_map = GridMap::new(world_config.clone());
    setup_grid(&mut grid_map);

    let mut game_manager = {
        let mut map_renderer = MapRenderer::new(render_config.clone());
        map_renderer.reset_mesh(&grid_map);

        let path_renderer = PathRenderer::new(render_config.clone());

        let mut camera_manager = CameraManager::new();
        camera_manager.target_to_bound(
            vec2(0.0, 0.0),
            vec2(map_renderer.max_bound().x, map_renderer.max_bound().y),
            0.8,
        );

        let ui_manager = UIManager::new(render_config.font_size);
        let grid_map_arc = Arc::new(grid_map);

        GameManager::new(
            Box::new(map_renderer),
            Box::new(path_renderer),
            Box::new(camera_manager),
            Box::new(ui_manager),
            render_config,
            grid_map_arc,
        )
    };
    // ==================================================================
    // let test_case = 15;
    // let problem = create_problem_with_testcase(Arc::new(grid_map), test_case);
    // let path_points = AStarStrategy::path_finding(&problem);
    // game_manager.path_renderer_mut().set_path(path_points.clone());

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
