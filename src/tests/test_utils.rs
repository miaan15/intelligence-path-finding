//! Test utilities and helpers for path-finding algorithms

use crate::data_structures::{Grid, Position};

/// Create test grids of various sizes and patterns
pub struct TestGridFactory;

impl TestGridFactory {
    /// Create a small empty grid for basic testing
    pub fn small_empty_grid() -> Grid {
        Grid::new(5, 5)
    }

    /// Create a medium empty grid
    pub fn medium_empty_grid() -> Grid {
        Grid::new(10, 10)
    }

    /// Create a large empty grid for performance testing
    pub fn large_empty_grid() -> Grid {
        Grid::new(100, 100)
    }

    /// Create a grid with a simple wall
    pub fn grid_with_wall() -> Grid {
        let mut grid = Grid::new(5, 5);

        // Create a vertical wall in the middle
        for y in 1..4 {
            grid.set_walkable(&Position::new(2, y), false);
        }

        grid
    }

    /// Create a grid with a maze pattern
    pub fn maze_grid() -> Grid {
        let data = vec![
            vec![true, true, true, true, true, true, true, true, true],
            vec![true, false, false, false, true, false, false, false, true],
            vec![true, false, true, false, true, false, true, false, true],
            vec![true, false, true, false, false, false, true, false, true],
            vec![true, false, true, true, true, true, true, false, true],
            vec![true, false, false, false, false, false, false, false, true],
            vec![true, true, true, true, true, true, true, true, true],
        ];

        Grid::from_bool_array(&data).unwrap()
    }

    /// Create a grid with no possible path
    pub fn no_path_grid() -> Grid {
        let mut grid = Grid::new(5, 5);

        // Create a complete wall barrier
        for y in 0..5 {
            grid.set_walkable(&Position::new(2, y), false);
        }

        grid
    }

    /// Create a grid from a string pattern
    pub fn from_string_pattern(pattern: &str) -> Grid {
        let lines: Vec<&str> = pattern.trim().lines().collect();
        if lines.is_empty() {
            return Grid::new(1, 1);
        }

        let width = lines[0].len();
        let height = lines.len();
        let mut grid = Grid::new(width, height);

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let walkable = match ch {
                    '.' | ' ' | '0' => true,
                    '#' | 'X' | '1' => false,
                    _ => true,
                };
                grid.set_walkable(&Position::new(x, y), walkable);
            }
        }

        grid
    }
}

/// Common test scenarios
pub struct TestScenarios;

impl TestScenarios {
    /// Get standard test start and end positions
    pub fn standard_positions() -> (Position, Position) {
        (Position::new(0, 0), Position::new(4, 4))
    }

    /// Get positions for the no-path scenario
    pub fn no_path_positions() -> (Position, Position) {
        (Position::new(0, 2), Position::new(4, 2))
    }

    /// Get positions for the maze scenario
    pub fn maze_positions() -> (Position, Position) {
        (Position::new(0, 0), Position::new(8, 6))
    }
}

/// Performance benchmark utilities
pub struct BenchmarkUtils;

impl BenchmarkUtils {
    /// Measure execution time of a function
    pub fn measure_time<F, R>(f: F) -> (R, std::time::Duration)
    where
        F: FnOnce() -> R,
    {
        let start = std::time::Instant::now();
        let result = f();
        let duration = start.elapsed();
        (result, duration)
    }

    /// Run multiple iterations and return average time
    pub fn benchmark<F, R>(f: F, iterations: usize) -> (R, std::time::Duration)
    where
        F: Fn() -> R + Clone,
    {
        let mut total_duration = std::time::Duration::new(0, 0);
        let mut last_result = None;

        for _ in 0..iterations {
            let (result, duration) = BenchmarkUtils::measure_time(f.clone());
            total_duration += duration;
            last_result = Some(result);
        }

        let avg_duration = total_duration / iterations as u32;
        (last_result.unwrap(), avg_duration)
    }
}

/// Assertion helpers for testing
pub struct PathAssertions;

impl PathAssertions {
    /// Assert that a path is valid (connected and walkable)
    pub fn assert_valid_path(grid: &Grid, path: &[Position]) {
        if path.is_empty() {
            return;
        }

        // Check that all positions are walkable
        for position in path {
            assert!(
                grid.is_walkable(position),
                "Path contains non-walkable position: {:?}",
                position
            );
        }

        // Check that consecutive positions are adjacent
        for window in path.windows(2) {
            let pos1 = &window[0];
            let pos2 = &window[1];

            let distance = pos1.manhattan_distance(pos2);
            assert!(
                distance <= 1,
                "Path contains non-adjacent positions: {:?} -> {:?} (distance: {})",
                pos1, pos2, distance
            );
        }
    }

    /// Assert that a path is optimal for Manhattan distance
    pub fn assert_optimal_manhattan_path(start: &Position, end: &Position, path: &[Position]) {
        if path.is_empty() {
            return;
        }

        let expected_length = start.manhattan_distance(end) + 1; // +1 to include both start and end
        assert_eq!(
            path.len(),
            expected_length,
            "Path length {} is not optimal. Expected {} for Manhattan distance from {:?} to {:?}",
            path.len(),
            expected_length,
            start,
            end
        );
    }

    /// Assert path contains expected positions
    pub fn assert_path_contains(path: &[Position], expected_positions: &[Position]) {
        for expected_pos in expected_positions {
            assert!(
                path.contains(expected_pos),
                "Path does not contain expected position: {:?}",
                expected_pos
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_grid_factory() {
        let grid = TestGridFactory::small_empty_grid();
        assert_eq!(grid.dimensions(), (5, 5));

        let wall_grid = TestGridFactory::grid_with_wall();
        assert!(!wall_grid.is_walkable(&Position::new(2, 1)));

        let maze_grid = TestGridFactory::maze_grid();
        assert_eq!(maze_grid.dimensions(), (9, 7));
    }

    #[test]
    fn test_string_pattern_grid() {
        let pattern = "
            .#.
            .#.
            ...
        ";
        let grid = TestGridFactory::from_string_pattern(pattern);
        assert_eq!(grid.dimensions(), (3, 3));
        assert!(grid.is_walkable(&Position::new(0, 0)));
        assert!(!grid.is_walkable(&Position::new(1, 0)));
        assert!(grid.is_walkable(&Position::new(2, 2)));
    }

    #[test]
    fn test_benchmark_utils() {
        let dummy_func = || 42;
        let (result, duration) = BenchmarkUtils::measure_time(dummy_func);
        assert_eq!(result, 42);
        assert!(duration.as_nanos() > 0);
    }
}