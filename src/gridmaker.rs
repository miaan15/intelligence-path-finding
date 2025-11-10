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

    // === CONCENTRIC SQUARE MAZE ===
    // Outer ring (4 cells from edge)
    for i in 4..32 {
        grid_map.grid_mut().set(i, 4, GridNodeValue::Obstacle);
        grid_map.grid_mut().set(i, 31, GridNodeValue::Obstacle);
        grid_map.grid_mut().set(4, i, GridNodeValue::Obstacle);
        grid_map.grid_mut().set(31, i, GridNodeValue::Obstacle);
    }
    // Gaps in outer ring
    for i in 10..12 {
        grid_map.grid_mut().set(i, 4, GridNodeValue::Air);
    }
    for i in 24..26 {
        grid_map.grid_mut().set(i, 31, GridNodeValue::Air);
    }
    for i in 15..17 {
        grid_map.grid_mut().set(4, i, GridNodeValue::Air);
    }
    for i in 26..28 {
        grid_map.grid_mut().set(31, i, GridNodeValue::Air);
    }

    // Middle ring (8 cells from edge)
    for i in 8..28 {
        grid_map.grid_mut().set(i, 8, GridNodeValue::Obstacle);
        grid_map.grid_mut().set(i, 27, GridNodeValue::Obstacle);
        grid_map.grid_mut().set(8, i, GridNodeValue::Obstacle);
        grid_map.grid_mut().set(27, i, GridNodeValue::Obstacle);
    }
    // Gaps in middle ring (offset from outer)
    for i in 18..20 {
        grid_map.grid_mut().set(i, 8, GridNodeValue::Air);
    }
    for i in 16..18 {
        grid_map.grid_mut().set(i, 27, GridNodeValue::Air);
    }
    for i in 21..23 {
        grid_map.grid_mut().set(8, i, GridNodeValue::Air);
    }
    for i in 13..15 {
        grid_map.grid_mut().set(27, i, GridNodeValue::Air);
    }

    // Inner ring (12 cells from edge)
    for i in 12..24 {
        grid_map.grid_mut().set(i, 12, GridNodeValue::Obstacle);
        grid_map.grid_mut().set(i, 23, GridNodeValue::Obstacle);
        grid_map.grid_mut().set(12, i, GridNodeValue::Obstacle);
        grid_map.grid_mut().set(23, i, GridNodeValue::Obstacle);
    }
    // Gaps in inner ring
    for i in 15..17 {
        grid_map.grid_mut().set(i, 12, GridNodeValue::Air);
    }
    for i in 19..21 {
        grid_map.grid_mut().set(i, 23, GridNodeValue::Air);
    }
    for i in 14..16 {
        grid_map.grid_mut().set(12, i, GridNodeValue::Air);
    }
    for i in 20..22 {
        grid_map.grid_mut().set(23, i, GridNodeValue::Air);
    }

    // === DIAGONAL BARRIERS ===
    // Top-left to center diagonal
    for i in 0..10 {
        grid_map.grid_mut().set(5 + i, 5 + i, GridNodeValue::Obstacle);
        grid_map.grid_mut().set(6 + i, 5 + i, GridNodeValue::Obstacle);
    }
    // Center to bottom-right diagonal
    for i in 0..10 {
        grid_map.grid_mut().set(20 + i, 20 + i, GridNodeValue::Obstacle);
        grid_map.grid_mut().set(21 + i, 20 + i, GridNodeValue::Obstacle);
    }
    // Top-right to center anti-diagonal
    for i in 0..9 {
        grid_map.grid_mut().set(30 - i, 5 + i, GridNodeValue::Obstacle);
        grid_map.grid_mut().set(29 - i, 5 + i, GridNodeValue::Obstacle);
    }
    // Center to bottom-left anti-diagonal
    for i in 0..9 {
        grid_map.grid_mut().set(15 - i, 21 + i, GridNodeValue::Obstacle);
        grid_map.grid_mut().set(14 - i, 21 + i, GridNodeValue::Obstacle);
    }

    // === COMPLEX CORNER CHAMBERS ===
    // Top-left chamber
    for x in 2..7 {
        grid_map.grid_mut().set(x, 3, GridNodeValue::Obstacle);
    }
    for y in 2..3 {
        grid_map.grid_mut().set(7, y, GridNodeValue::Obstacle);
    }
    grid_map.grid_mut().set(3, 2, GridNodeValue::Obstacle);
    grid_map.grid_mut().set(4, 2, GridNodeValue::Obstacle);

    // Top-right chamber
    for x in 29..34 {
        grid_map.grid_mut().set(x, 3, GridNodeValue::Obstacle);
    }
    for y in 2..3 {
        grid_map.grid_mut().set(28, y, GridNodeValue::Obstacle);
    }
    grid_map.grid_mut().set(31, 2, GridNodeValue::Obstacle);
    grid_map.grid_mut().set(32, 2, GridNodeValue::Obstacle);

    // Bottom-left chamber
    for x in 2..7 {
        grid_map.grid_mut().set(x, 32, GridNodeValue::Obstacle);
    }
    for y in 33..34 {
        grid_map.grid_mut().set(7, y, GridNodeValue::Obstacle);
    }
    grid_map.grid_mut().set(3, 33, GridNodeValue::Obstacle);
    grid_map.grid_mut().set(4, 33, GridNodeValue::Obstacle);

    // Bottom-right chamber
    for x in 29..34 {
        grid_map.grid_mut().set(x, 32, GridNodeValue::Obstacle);
    }
    for y in 33..34 {
        grid_map.grid_mut().set(28, y, GridNodeValue::Obstacle);
    }
    grid_map.grid_mut().set(31, 33, GridNodeValue::Obstacle);
    grid_map.grid_mut().set(32, 33, GridNodeValue::Obstacle);

    // === SPIRAL MAZE IN CENTER ===
    let center_x = 18;
    let center_y = 18;
    for radius in 1..4 {
        let r = radius * 2;
        // Top edge
        for x in (center_x - r)..(center_x + r - 1) {
            if x >= 1 && x < width - 1 {
                grid_map.grid_mut().set(x, center_y - r, GridNodeValue::Obstacle);
            }
        }
        // Right edge
        for y in (center_y - r)..(center_y + r - 1) {
            if y >= 1 && y < height - 1 {
                grid_map.grid_mut().set(center_x + r, y, GridNodeValue::Obstacle);
            }
        }
        // Bottom edge
        for x in (center_x - r + 1)..(center_x + r + 1) {
            if x >= 1 && x < width - 1 {
                grid_map.grid_mut().set(x, center_y + r, GridNodeValue::Obstacle);
            }
        }
        // Left edge (with gap for entrance)
        for y in (center_y - r + 1)..(center_y + r) {
            if y >= 1 && y < height - 1 && y != center_y - r + 1 {
                grid_map.grid_mut().set(center_x - r, y, GridNodeValue::Obstacle);
            }
        }
    }

    // === ZIGZAG CORRIDORS ===
    for i in (6..30).step_by(3) {
        for j in 0..2 {
            if i + j < width - 1 {
                grid_map.grid_mut().set(i + j, 18, GridNodeValue::Obstacle);
            }
            if i % 6 == 0 && i + j < height - 1 {
                grid_map.grid_mut().set(18, i + j, GridNodeValue::Obstacle);
            }
        }
    }

    // === T-JUNCTION PATTERNS ===
    // Horizontal T-junctions
    for base_y in [10, 25].iter() {
        for x in 9..12 {
            grid_map.grid_mut().set(x, *base_y, GridNodeValue::Obstacle);
        }
        for y in (base_y - 2)..(base_y + 3) {
            if y >= 1 && y < height - 1 {
                grid_map.grid_mut().set(10, y, GridNodeValue::Obstacle);
            }
        }
        grid_map.grid_mut().set(10, *base_y, GridNodeValue::Air);
    }

    for base_y in [10, 25].iter() {
        for x in 24..27 {
            grid_map.grid_mut().set(x, *base_y, GridNodeValue::Obstacle);
        }
        for y in (base_y - 2)..(base_y + 3) {
            if y >= 1 && y < height - 1 {
                grid_map.grid_mut().set(25, y, GridNodeValue::Obstacle);
            }
        }
        grid_map.grid_mut().set(25, *base_y, GridNodeValue::Air);
    }

    // === STRATEGIC OBSTACLE CLUSTERS ===
    let clusters = [
        (7, 18, 2, 2),
        (27, 18, 2, 2),
        (18, 7, 2, 2),
        (18, 27, 2, 2),
        (11, 11, 2, 2),
        (23, 11, 2, 2),
        (11, 23, 2, 2),
        (23, 23, 2, 2),
        (6, 21, 2, 3),
        (28, 21, 2, 3),
        (14, 6, 3, 2),
        (14, 28, 3, 2),
    ];
    for (cx, cy, w, h) in clusters.iter() {
        for dx in 0..*w {
            for dy in 0..*h {
                let x = cx + dx;
                let y = cy + dy;
                if x < width && y < height {
                    grid_map.grid_mut().set(x, y, GridNodeValue::Obstacle);
                }
            }
        }
    }

    // === CROSS PATTERNS ===
    for center in [(10, 18), (25, 18), (18, 10), (18, 25)].iter() {
        for i in 0..3 {
            grid_map.grid_mut().set(center.0 - 1 + i, center.1, GridNodeValue::Obstacle);
            grid_map.grid_mut().set(center.0, center.1 - 1 + i, GridNodeValue::Obstacle);
        }
        grid_map.grid_mut().set(center.0, center.1, GridNodeValue::Air);
    }

    // === NARROW PASSAGES ===
    let passage_walls = [
        (2, 8),
        (2, 9),
        (2, 10),
        (33, 8),
        (33, 9),
        (33, 10),
        (2, 25),
        (2, 26),
        (2, 27),
        (33, 25),
        (33, 26),
        (33, 27),
    ];
    for (x, y) in passage_walls.iter() {
        grid_map.grid_mut().set(*x, *y, GridNodeValue::Obstacle);
    }

    // === L-SHAPED BARRIERS ===
    // Top-left L
    for i in 15..18 {
        grid_map.grid_mut().set(i, 6, GridNodeValue::Obstacle);
        grid_map.grid_mut().set(15, i, GridNodeValue::Obstacle);
    }
    // Top-right L
    for i in 18..21 {
        grid_map.grid_mut().set(i, 6, GridNodeValue::Obstacle);
        grid_map.grid_mut().set(20, i, GridNodeValue::Obstacle);
    }
    // Bottom-left L
    for i in 15..18 {
        grid_map.grid_mut().set(i, 29, GridNodeValue::Obstacle);
        grid_map.grid_mut().set(15, i + 11, GridNodeValue::Obstacle);
    }
    // Bottom-right L
    for i in 18..21 {
        grid_map.grid_mut().set(i, 29, GridNodeValue::Obstacle);
        grid_map.grid_mut().set(20, i + 11, GridNodeValue::Obstacle);
    }
}
