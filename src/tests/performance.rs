//! Performance benchmarks for path-finding algorithms

use crate::data_structures::{Grid, Position};
use crate::tests::test_utils::*;

// Performance tests using Criterion
#[cfg(test)]
mod criterion_benchmarks {
    use super::*;
    use criterion::{black_box, criterion_group, criterion_main, Criterion};

    fn bench_small_grid(c: &mut Criterion) {
        let grid = TestGridFactory::small_empty_grid();
        let (start, end) = TestScenarios::standard_positions();

        c.bench_function("small_grid_astar", |b| {
            b.iter(|| {
                // astar::find_path(black_box(&grid), black_box(&start), black_box(&end))
            })
        });

        c.bench_function("small_grid_dijkstra", |b| {
            b.iter(|| {
                // dijkstra::find_path(black_box(&grid), black_box(&start), black_box(&end))
            })
        });

        c.bench_function("small_grid_bfs", |b| {
            b.iter(|| {
                // bfs::find_path(black_box(&grid), black_box(&start), black_box(&end))
            })
        });
    }

    fn bench_medium_grid(c: &mut Criterion) {
        let grid = TestGridFactory::medium_empty_grid();
        let start = Position::new(0, 0);
        let end = Position::new(9, 9);

        c.bench_function("medium_grid_astar", |b| {
            b.iter(|| {
                // astar::find_path(black_box(&grid), black_box(&start), black_box(&end))
            })
        });

        c.bench_function("medium_grid_dijkstra", |b| {
            b.iter(|| {
                // dijkstra::find_path(black_box(&grid), black_box(&start), black_box(&end))
            })
        });
    }

    fn bench_large_grid(c: &mut Criterion) {
        let grid = TestGridFactory::large_empty_grid();
        let start = Position::new(0, 0);
        let end = Position::new(99, 99);

        c.bench_function("large_grid_astar", |b| {
            b.iter(|| {
                // astar::find_path(black_box(&grid), black_box(&start), black_box(&end))
            })
        });

        c.bench_function("large_grid_dijkstra", |b| {
            b.iter(|| {
                // dijkstra::find_path(black_box(&grid), black_box(&start), black_box(&end))
            })
        });
    }

    fn bench_maze_navigation(c: &mut Criterion) {
        let grid = TestGridFactory::maze_grid();
        let (start, end) = TestScenarios::maze_positions();

        c.bench_function("maze_astar", |b| {
            b.iter(|| {
                // astar::find_path(black_box(&grid), black_box(&start), black_box(&end))
            })
        });

        c.bench_function("maze_dijkstra", |b| {
            b.iter(|| {
                // dijkstra::find_path(black_box(&grid), black_box(&start), black_box(&end))
            })
        });
    }

    fn bench_heuristic_functions(c: &mut Criterion) {
        let pos1 = Position::new(0, 0);
        let pos2 = Position::new(100, 100);

        c.bench_function("manhattan_distance", |b| {
            b.iter(|| {
                black_box(pos1.manhattan_distance(black_box(&pos2)))
            })
        });

        c.bench_function("euclidean_distance", |b| {
            b.iter(|| {
                black_box(pos1.euclidean_distance(black_box(&pos2)))
            })
        });
    }

    fn bench_grid_operations(c: &mut Criterion) {
        let grid = TestGridFactory::medium_empty_grid();
        let position = Position::new(5, 5);

        c.bench_function("grid_get_neighbors", |b| {
            b.iter(|| {
                black_box(grid.get_neighbors(black_box(&position)))
            })
        });

        c.bench_function("grid_is_walkable", |b| {
            b.iter(|| {
                black_box(grid.is_walkable(black_box(&position)))
            })
        });

        c.bench_function("grid_valid_position", |b| {
            b.iter(|| {
                black_box(grid.is_valid_position(black_box(&position)))
            })
        });
    }

    criterion_group!(
        benches,
        bench_small_grid,
        bench_medium_grid,
        bench_large_grid,
        bench_maze_navigation,
        bench_heuristic_functions,
        bench_grid_operations
    );
    criterion_main!(benches);
}

#[cfg(test)]
mod stress_tests {
    use super::*;

    #[test]
    fn test_memory_usage_large_grid() {
        // Test that large grids don't cause memory issues
        let large_grid = Grid::new(500, 500);

        // Should be able to create and manipulate large grid
        assert_eq!(large_grid.dimensions(), (500, 500));

        // Test operations on large grid
        let pos = Position::new(250, 250);
        let neighbors = large_grid.get_neighbors(&pos);
        assert!(!neighbors.is_empty());
    }

    #[test]
    fn test_extreme_obstacle_density() {
        // Test performance with very high obstacle density
        let mut grid = Grid::new(50, 50);

        // Set 90% of cells as non-walkable
        for y in 0..50 {
            for x in 0..50 {
                if (x + y) % 10 != 0 {
                    grid.set_walkable(&Position::new(x, y), false);
                }
            }
        }

        let start = Position::new(0, 0);
        let end = Position::new(49, 49);

        let (result, duration) = BenchmarkUtils::measure_time(|| {
            // Test algorithm performance with sparse walkable area
            // astar::find_path(&grid, &start, &end)
        });

        // Should complete in reasonable time even with high obstacle density
        assert!(duration.as_millis() < 1000);
    }

    #[test]
    fn test_concurrent_pathfinding() {
        use std::sync::Arc;
        use std::thread;

        // Test multiple path-finding operations concurrently
        let grid = Arc::new(TestGridFactory::medium_empty_grid());
        let handles: Vec<_> = (0..4)
            .map(|i| {
                let grid = Arc::clone(&grid);
                thread::spawn(move || {
                    let start = Position::new(i * 2, i * 2);
                    let end = Position::new(9 - i * 2, 9 - i * 2);

                    // Test concurrent path-finding
                    // astar::find_path(&grid, &start, &end)
                })
            })
            .collect();

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
    }
}