use crate::algorithm::problem::*;
use crate::algorithm::strategy::*;
use crate::world::grid::*;
use crate::world::types::*;
use macroquad::prelude::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq)]
struct Node {
    pos: Vec2,
    pre: Option<Vec2>,
    g: f32,
    h: f32,
}
impl Node {
    fn f(&self) -> f32 {
        self.g + self.h
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .f()
            .partial_cmp(&self.f())
            .unwrap_or(Ordering::Equal)
            .then_with(|| self.h.partial_cmp(&other.h).unwrap_or(Ordering::Equal))
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for Node {}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pos.x.to_bits().hash(state);
        self.pos.y.to_bits().hash(state);
    }
}

pub struct AStarStrategy {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProbeDirection {
    Right,
    UpRight,
    Up,
    UpLeft,
    Left,
    DownLeft,
    Down,
    DownRight,
}
impl ProbeDirection {
    pub fn iter() -> std::slice::Iter<'static, Self> {
        static DIRECTIONS: [ProbeDirection; 8] = [
            ProbeDirection::Right,
            ProbeDirection::UpRight,
            ProbeDirection::Up,
            ProbeDirection::UpLeft,
            ProbeDirection::Left,
            ProbeDirection::DownLeft,
            ProbeDirection::Down,
            ProbeDirection::DownRight,
        ];
        DIRECTIONS.iter()
    }
}
impl AStarStrategy {
    const STEP_SIZE: f32 = 10.0;
    const ANGLE_SAMPLES: usize = 8;

    fn heuristic(a: Vec2, b: Vec2) -> f32 {
        a.distance(b)
    }

    fn has_line_of_sight(from: Vec2, to: Vec2, grid_map: &GridMap) -> bool {
        let direction = (to - from).normalize_or_zero();
        let distance = from.distance(to);
        if distance == 0.0 {
            return true;
        }

        let ray = Ray {
            root: from,
            dir: direction,
        };

        grid_map.raycast(ray).map_or(true, |hit| hit.dist >= distance)
    }

    fn get_new_pos(root: Vec2, direction: ProbeDirection, grid_map: &GridMap) -> Vec2 {
        let offset = match direction {
            ProbeDirection::Right => Vec2::X,               // (1, 0)
            ProbeDirection::UpRight => Vec2::X + Vec2::Y,   // (1, 1)
            ProbeDirection::Up => Vec2::Y,                  // (0, 1)
            ProbeDirection::UpLeft => -Vec2::X + Vec2::Y,   // (-1, 1)
            ProbeDirection::Left => -Vec2::X,               // (-1, 0)
            ProbeDirection::DownLeft => -Vec2::X - Vec2::Y, // (-1, -1)
            ProbeDirection::Down => -Vec2::Y,               // (0, -1)
            ProbeDirection::DownRight => Vec2::X - Vec2::Y, // (1, -1)
        };

        let ray = Ray { root, dir: offset };

        if let Some(hit) = grid_map.raycast(ray) {
            root + offset * hit.dist.min(AStarStrategy::STEP_SIZE)
        } else {
            root + offset * AStarStrategy::STEP_SIZE
        }
    }
}

impl Strategy for AStarStrategy {
    fn path_finding(problem: &Problem) -> Vec<Vec2> {
        let grid_map = problem.grid_map();
        let start = problem.start();
        let goal = problem.end();

        let mut queue = BinaryHeap::new();
        let mut map = HashMap::new();

        let start_node = Node {
            pos: start,
            pre: None,
            g: 0.0,
            h: 0.0,
        };
        queue.push(start_node);
        map.insert(start_node, 0.0);

        let mut goal_node: Option<Node> = None;
        while !queue.is_empty() {
            let cur = match queue.pop() {
                Some(value) => value,
                None => break,
            };

            if AStarStrategy::has_line_of_sight(cur.pos, goal, grid_map) {
                goal_node = Some(Node {
                    pos: goal,
                    pre: Some(cur.pos),
                    g: cur.g + goal.distance(cur.pos),
                    h: 0.0,
                });
                break;
            }

            for dir in ProbeDirection::iter() {
                let new_pos = AStarStrategy::get_new_pos(cur.pos, dir.clone(), grid_map);
                let new_dist = new_pos.distance(cur.pos);
                let new = Node {
                    pos: new_pos,
                    pre: Some(cur.pos),
                    g: cur.g + new_dist,
                    h: new_pos.distance(goal),
                };

                if let Some(new_g) = map.get(&new) {
                    if new_g.into() > cur.g + new_dist {
                        queue.push(new);
                        map.insert(new, cur.g + new_dist);
                    }
                } else {
                    queue.push(new);
                    map.insert(new, cur.g + new_dist);
                }
            }
        }

        if goal_node.is_none() {
            Vec::new()
        } else {
        }
    }
}
