// Import from the local library
use pathfinding::world::grid::{GridMap, GridNodeValue};
use pathfinding::world::{WorldConfig, types::Ray};
use pathfinding::render::{grid_render::GridRenderer, RenderConfig};
use macroquad::prelude::*;

#[macroquad::main("Path Finding Demo")]
async fn main() {
    // Set up the world configuration
    let world_config = WorldConfig {
        grid_size: (20, 15),
        cell_size: 40.0,
    };

    // Create the grid map
    let mut grid_map = GridMap::new(20, 15, world_config);

    // Add some obstacles (hardcoded pattern)
    // Create a border of obstacles
    for x in 0..20 {
        grid_map.grid_mut().set(x, 0, GridNodeValue::Obstacle);
        grid_map.grid_mut().set(x, 14, GridNodeValue::Obstacle);
    }
    for y in 0..15 {
        grid_map.grid_mut().set(0, y, GridNodeValue::Obstacle);
        grid_map.grid_mut().set(19, y, GridNodeValue::Obstacle);
    }

    // Add some obstacle patterns inside
    // Vertical wall
    for y in 3..12 {
        grid_map.grid_mut().set(8, y, GridNodeValue::Obstacle);
        grid_map.grid_mut().set(12, y, GridNodeValue::Obstacle);
    }

    // Horizontal walls
    for x in 2..8 {
        grid_map.grid_mut().set(x, 7, GridNodeValue::Obstacle);
    }
    for x in 13..18 {
        grid_map.grid_mut().set(x, 7, GridNodeValue::Obstacle);
    }

    // Some scattered obstacles
    grid_map.grid_mut().set(5, 4, GridNodeValue::Obstacle);
    grid_map.grid_mut().set(6, 4, GridNodeValue::Obstacle);
    grid_map.grid_mut().set(14, 10, GridNodeValue::Obstacle);
    grid_map.grid_mut().set(15, 10, GridNodeValue::Obstacle);
    grid_map.grid_mut().set(16, 10, GridNodeValue::Obstacle);

    // Set up the render configuration
    let render_config = RenderConfig {
        background_color: DARKGRAY,
        obstacle_color: RED,
    };

    // Create the grid renderer
    let mut grid_renderer = GridRenderer::new(render_config);

    // Generate the initial mesh
    grid_renderer.update_mesh(&grid_map);

    // Camera settings
    let camera_zoom = 1.0f32;
    let screen_width = grid_map.width() as f32 * grid_map.cell_size();
    let screen_height = grid_map.height() as f32 * grid_map.cell_size();

    // Ray setup
    let mut ray_angle = 0.0f32;
    let ray_origin = Vec2::new(screen_width / 2.0, screen_height / 2.0);
    let ray_length = 300.0f32;
    let ray_speed = 1.0f32; // radians per second

    loop {
        // Update ray angle
        ray_angle += ray_speed * get_frame_time();

        // Clear background
        grid_renderer.clear_background();

        // Set up camera
        set_camera(&Camera2D {
            target: vec2(screen_width / 2.0, screen_height / 2.0),
            zoom: vec2(1.0 / screen_width, -1.0 / screen_height) * camera_zoom,
            ..Default::default()
        });

        // Draw the grid
        grid_renderer.draw();

        // Create ray
        let ray_direction = Vec2::new(ray_angle.cos(), ray_angle.sin());
        let ray = Ray {
            root: ray_origin,
            dir: ray_direction,
        };

        // Cast ray and get hit info
        let _ray_end = if let Some(hit_info) = grid_map.raycast(ray) {
            // Draw ray to hit point
            draw_ray(ray_origin, hit_info.pt, YELLOW, 2.0);

            // Draw hit point
            draw_circle(hit_info.pt.x, hit_info.pt.y, 5.0, RED);

            // Draw normal vector at hit point
            let normal_end = hit_info.pt + hit_info.nor * 20.0;
            draw_line(hit_info.pt.x, hit_info.pt.y, normal_end.x, normal_end.y, 2.0, GREEN);

            hit_info.pt
        } else {
            // Draw ray to max length
            let max_end = ray_origin + ray_direction * ray_length;
            draw_ray(ray_origin, max_end, YELLOW, 1.0);
            max_end
        };

        // Draw ray origin
        draw_circle(ray_origin.x, ray_origin.y, 8.0, BLUE);

        // Helper function to draw a ray
        fn draw_ray(start: Vec2, end: Vec2, color: Color, thickness: f32) {
            draw_line(start.x, start.y, end.x, end.y, thickness, color);
        }

        // Draw grid lines for visual reference
        for x in 0..=grid_map.width() {
            let x_pos = x as f32 * grid_map.cell_size();
            draw_line(
                x_pos, 0.0,
                x_pos, screen_height,
                1.0,
                LIGHTGRAY,
            );
        }

        for y in 0..=grid_map.height() {
            let y_pos = y as f32 * grid_map.cell_size();
            draw_line(
                0.0, y_pos,
                screen_width, y_pos,
                1.0,
                LIGHTGRAY,
            );
        }

        // Reset camera for UI
        set_default_camera();

        // Draw some UI text
        draw_text("Path Finding Demo - Raycast Visualization", 10.0, 20.0, 20.0, WHITE);
        draw_text(&format!("Grid: {}x{}", grid_map.width(), grid_map.height()), 10.0, 50.0, 16.0, WHITE);
        draw_text(&format!("Cell size: {:.1}", grid_map.cell_size()), 10.0, 70.0, 16.0, WHITE);
        draw_text(&format!("Ray angle: {:.1}°", (ray_angle * 180.0 / std::f32::consts::PI) % 360.0), 10.0, 90.0, 16.0, WHITE);

        // Legend
        draw_text("Blue: Ray origin | Yellow: Ray", 10.0, screen_height - 60.0, 14.0, WHITE);
        draw_text("Red: Hit point | Green: Normal", 10.0, screen_height - 40.0, 14.0, WHITE);
        draw_text("Press ESC to exit", 10.0, screen_height - 20.0, 16.0, WHITE);

        // Exit condition
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
