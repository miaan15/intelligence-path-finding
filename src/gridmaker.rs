use crate::world::grid::*;

pub fn setup_grid(grid_map: &mut GridMap) {
    let width = 36;
    let height = 36;

    // === OUTER WALLS ===
    for x in 0..width {
        grid_map.grid_mut().set(x, 0, GridNodeValue::Obstacle);
        grid_map.grid_mut().set(x, height - 1, GridNodeValue::Obstacle);
    }
    for y in 0..height {
        grid_map.grid_mut().set(0, y, GridNodeValue::Obstacle);
        grid_map.grid_mut().set(width - 1, y, GridNodeValue::Obstacle);
    }

    // === LARGE ROOM DIVIDERS ===
    // Vertical divider with gaps
    for y in 5..31 {
        if y < 12 || y > 18 {
            // Gap in middle
            grid_map.grid_mut().set(12, y, GridNodeValue::Obstacle);
        }
    }

    for y in 5..31 {
        if y < 15 || y > 21 {
            // Gap in middle (offset from first)
            grid_map.grid_mut().set(24, y, GridNodeValue::Obstacle);
        }
    }

    // Horizontal divider with gaps
    for x in 5..31 {
        if x < 15 || x > 21 {
            // Gap in middle
            grid_map.grid_mut().set(x, 12, GridNodeValue::Obstacle);
        }
    }

    for x in 5..31 {
        if x < 12 || x > 18 {
            // Gap in middle (offset from first)
            grid_map.grid_mut().set(x, 24, GridNodeValue::Obstacle);
        }
    }

    // === DIAGONAL BARRIERS ===
    // Top-left to center
    for i in 0..8 {
        grid_map.grid_mut().set(5 + i, 5 + i, GridNodeValue::Obstacle);
        grid_map.grid_mut().set(4 + i, 5 + i, GridNodeValue::Obstacle);
    }

    // Top-right to center
    for i in 0..8 {
        grid_map.grid_mut().set(30 - i, 5 + i, GridNodeValue::Obstacle);
        grid_map.grid_mut().set(29 - i, 5 + i, GridNodeValue::Obstacle);
    }

    // Bottom-left to center
    for i in 0..8 {
        grid_map.grid_mut().set(5 + i, 30 - i, GridNodeValue::Obstacle);
        grid_map.grid_mut().set(4 + i, 30 - i, GridNodeValue::Obstacle);
    }

    // Bottom-right to center
    for i in 0..8 {
        grid_map.grid_mut().set(30 - i, 30 - i, GridNodeValue::Obstacle);
        grid_map.grid_mut().set(29 - i, 30 - i, GridNodeValue::Obstacle);
    }

    // === CENTRAL OBSTACLE ===
    for x in 17..19 {
        for y in 17..19 {
            grid_map.grid_mut().set(x, y, GridNodeValue::Obstacle);
        }
    }

    for x in 10..21 {
        grid_map.grid_mut().set(x, 15, GridNodeValue::Obstacle);
    }
    for y in 16..20 {
        grid_map.grid_mut().set(15, y, GridNodeValue::Obstacle);
    }
    for x in 14..22 {
        grid_map.grid_mut().set(x, 21, GridNodeValue::Obstacle);
    }
    for y in 16..20 {
        grid_map.grid_mut().set(22, y, GridNodeValue::Obstacle);
    }

    for x in 23..30 {
        grid_map.grid_mut().set(x, 16, GridNodeValue::Obstacle);
    }

    // === CORNER OBSTACLES ===
    // Top-left
    for x in 3..6 {
        for y in 3..6 {
            grid_map.grid_mut().set(x, y, GridNodeValue::Obstacle);
        }
    }

    // Top-right
    for x in 30..33 {
        for y in 3..6 {
            grid_map.grid_mut().set(x, y, GridNodeValue::Obstacle);
        }
    }

    // Bottom-left
    for x in 3..6 {
        for y in 30..33 {
            grid_map.grid_mut().set(x, y, GridNodeValue::Obstacle);
        }
    }

    // Bottom-right
    for x in 30..33 {
        for y in 30..33 {
            grid_map.grid_mut().set(x, y, GridNodeValue::Obstacle);
        }
    }

    // === STRATEGIC BLOCKS ===
    // These force interesting paths without too many vertices
    let blocks = [
        (8, 8, 2, 2),
        (26, 8, 2, 2),
        (8, 26, 2, 2),
        (26, 26, 2, 2),
        (18, 8, 2, 2),
        (8, 18, 2, 2),
        (26, 18, 2, 2),
        (18, 26, 2, 2),
    ];

    for (x, y, w, h) in blocks.iter() {
        for dx in 0..*w {
            for dy in 0..*h {
                grid_map.grid_mut().set(x + dx, y + dy, GridNodeValue::Obstacle);
            }
        }
    }
}
