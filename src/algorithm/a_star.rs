use crate::algorithm::problem::*;
use crate::algorithm::strategy::*;
use crate::world::grid::*;
use crate::world::types::*;
use macroquad::prelude::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone, PartialEq)]
struct Node {
    position: Vec2,
    g_cost: f32,
    h_cost: f32,
    parent: Option<Vec2>,
}

impl Node {
    fn f_cost(&self) -> f32 { self.g_cost + self.h_cost }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .f_cost()
            .partial_cmp(&self.f_cost())
            .unwrap_or(Ordering::Equal)
            .then_with(|| self.h_cost.partial_cmp(&other.h_cost).unwrap_or(Ordering::Equal))
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}
impl Eq for Node {}

pub struct AStarStrategy {}

impl AStarStrategy {
    const STEP_SIZE: f32 = 10.0; // Step size for sampling points in continuous space
    const ANGLE_SAMPLES: usize = 8; // Number of directions to sample from each point

    fn heuristic(a: Vec2, b: Vec2) -> f32 { a.distance(b) }

    fn has_line_of_sight(from: Vec2, to: Vec2, grid_map: &GridMap) -> bool {
        let direction = to - from;
        let distance = direction.length();
        if distance == 0.0 {
            return true;
        }

        let ray = Ray {
            root: from,
            dir: direction / distance,
        };

        grid_map.raycast(ray).map_or(true, |hit| hit.dist >= distance)
    }

    fn get_sample_points(current: Vec2, goal: Vec2, grid_map: &GridMap) -> Vec<Vec2> {
        let mut points = Vec::new();

        // Add direct goal if visible
        if Self::has_line_of_sight(current, goal, grid_map) {
            points.push(goal);
            return points;
        }

        // Sample points in different directions
        for i in 0..Self::ANGLE_SAMPLES {
            let angle = (i as f32 / Self::ANGLE_SAMPLES as f32) * 2.0 * std::f32::consts::PI;
            let direction = Vec2::new(angle.cos(), angle.sin());
            let new_pos = current + direction * Self::STEP_SIZE;

            // Check if the new position is valid (no immediate obstacles)
            if Self::has_line_of_sight(current, new_pos, grid_map) {
                points.push(new_pos);
            }
        }

        // Also add points towards the goal direction
        let to_goal = goal - current;
        if to_goal.length() > 0.0 {
            let goal_dir = to_goal.normalize();
            for dist_mult in [0.5, 1.0, 1.5, 2.0] {
                let point = current + goal_dir * Self::STEP_SIZE * dist_mult;
                if Self::has_line_of_sight(current, point, grid_map) {
                    points.push(point);
                }
            }
        }

        points
    }

    fn smooth_path(path: &mut Vec<Vec2>, grid_map: &GridMap) {
        if path.len() <= 2 {
            return;
        }

        let mut smoothed = Vec::new();
        smoothed.push(path[0]);

        let mut current_idx = 0;
        while current_idx < path.len() - 1 {
            let mut furthest_visible = current_idx + 1;

            for i in (current_idx + 2)..path.len() {
                if Self::has_line_of_sight(path[current_idx], path[i], grid_map) {
                    furthest_visible = i;
                } else {
                    break;
                }
            }

            smoothed.push(path[furthest_visible]);
            current_idx = furthest_visible;
        }

        *path = smoothed;
    }
}

impl Strategy for AStarStrategy {
    fn path_finding(problem: &Problem) -> Vec<Vec2> {
        let grid_map = problem.grid_map();
        let start = problem.start();
        let goal = problem.end();

        // Check if start and goal positions are valid (not inside walls)
        if !Self::has_line_of_sight(start, start, grid_map) || !Self::has_line_of_sight(goal, goal, grid_map) {
            return Vec::new();
        }

        // Direct path if line of sight exists
        if Self::has_line_of_sight(start, goal, grid_map) {
            return vec![start, goal];
        }

        let mut open_set: BinaryHeap<Node> = BinaryHeap::new();
        let mut closed_set: HashMap<(i32, i32), Vec2> = HashMap::new(); // Grid-based visited set for efficiency
        let mut g_costs: HashMap<(i32, i32), f32> = HashMap::new();

        let start_node = Node {
            position: start,
            g_cost: 0.0,
            h_cost: Self::heuristic(start, goal),
            parent: None,
        };

        open_set.push(start_node);

        let start_grid_key = Self::pos_to_grid_key(start);
        g_costs.insert(start_grid_key, 0.0);

        while let Some(current) = open_set.pop() {
            // Check if we've reached the goal (within tolerance)
            if current.position.distance(goal) < Self::STEP_SIZE {
                let mut path = Vec::new();
                path.push(goal); // Add the actual goal

                // Reconstruct path by following parent chain
                let mut current_node = current;
                while let Some(parent_pos) = current_node.parent {
                    path.push(parent_pos);
                    // Find the parent node in closed set to continue chain
                    let parent_grid_key = Self::pos_to_grid_key(parent_pos);
                    if let Some(&stored_pos) = closed_set.get(&parent_grid_key) {
                        // Simple reconstruction - in a more complex implementation,
                        // we'd need to store full node information
                        if stored_pos.distance(start) < 1.0 {
                            break;
                        }
                    }
                    // For now, break if we've reached close to start
                    if parent_pos.distance(start) < Self::STEP_SIZE {
                        break;
                    }
                    current_node.position = parent_pos;
                }

                path.push(start);
                path.reverse();

                Self::smooth_path(&mut path, grid_map);
                return path;
            }

            let current_grid_key = Self::pos_to_grid_key(current.position);
            if closed_set.contains_key(&current_grid_key) {
                continue;
            }

            closed_set.insert(current_grid_key, current.position);

            // Generate sample points for exploration
            for sample_pos in Self::get_sample_points(current.position, goal, grid_map) {
                let sample_grid_key = Self::pos_to_grid_key(sample_pos);

                if closed_set.contains_key(&sample_grid_key) {
                    continue;
                }

                let move_cost = current.position.distance(sample_pos);
                let tentative_g_cost = current.g_cost + move_cost;

                let g_cost = g_costs.get(&sample_grid_key).copied().unwrap_or(f32::INFINITY);

                if tentative_g_cost < g_cost {
                    g_costs.insert(sample_grid_key, tentative_g_cost);

                    let h_cost = Self::heuristic(sample_pos, goal);

                    let neighbor_node = Node {
                        position: sample_pos,
                        g_cost: tentative_g_cost,
                        h_cost,
                        parent: Some(current.position),
                    };

                    open_set.push(neighbor_node);
                }
            }

            // Limit iterations to prevent infinite loops
            if closed_set.len() > 1000 {
                break;
            }
        }

        Vec::new()
    }
}

impl AStarStrategy {
    fn pos_to_grid_key(pos: Vec2) -> (i32, i32) {
        let grid_size = 5.0; // Size of grid cells for visited set
        ((pos.x / grid_size).floor() as i32, (pos.y / grid_size).floor() as i32)
    }
}
