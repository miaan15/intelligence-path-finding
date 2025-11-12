use crate::algorithm::problem::Problem;
use crate::algorithm::strategy::*;
use crate::game::temporary_dot_renderer::draw_temporary_dot;
use crate::world::{grid::*, types::*};
use macroquad::prelude::*;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Debug, Clone, Eq)]
pub struct Node {
    pos: (i32, i32),
}
impl Node {
    fn new(x: i32, y: i32) -> Self {
        Node { pos: (x, y) }
    }
}
impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

#[derive(Debug, Clone, Eq)]
struct Path {
    from: Node,
    to: Node,
}
impl Path {
    fn new(from: Node, to: Node) -> Self {
        Path { from, to }
    }
}
impl Hash for Path {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.from.hash(state);
        self.to.hash(state);
    }
}
impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.to == other.to
    }
}

pub struct AcoPsoStrategy {
    pub node_min_dist: f32,

    pub alpha: f64,
    pub beta: f64,
    pub evaporation: f64,
    pub deposit_constant: f64,

    pub init_pheromone: f64,
    pub min_ant_count: u32,
    pub max_ant_try: u32,
}

impl Strategy for AcoPsoStrategy {
    fn path_finding(&self, problem: &Problem) -> Vec<Vec2> {
        let mut path_pheromone: HashMap<Path, f64> = HashMap::new();

        let grid_map = problem.grid_map();
        let start = problem.start();
        let end = problem.end();

        let start_node = self.world_to_node_pos(start).unwrap();
        let end_node = self.world_to_node_pos(end).unwrap();

        let mut foundcnt = 0;

        for index in 0..self.min_ant_count {
            let mut try_count: u32 = 0;
            let mut ant_route: Vec<Path> = Vec::new();
            let mut visited: HashSet<Node> = HashSet::new();
            let mut route_len: f64 = 0.0;

            // Forward
            let mut cur_node = start_node.clone();
            let mut path_found = loop {
                if self.has_sight(self.node_to_world_pos(cur_node.clone()), end, grid_map) {
                    break true;
                }

                // Debug ======
                draw_temporary_dot(self.node_to_world_pos(cur_node.clone()), WHITE, 10.0, 1.0);

                try_count += 1;
                if try_count > self.max_ant_try {
                    break false;
                };

                let Some(next_node) = self.cal_next_node(cur_node.clone(), grid_map, &visited, &path_pheromone, &problem)
                else {
                    break false;
                };

                route_len += self
                    .node_to_world_pos(cur_node.clone())
                    .distance(self.node_to_world_pos(next_node.clone())) as f64;
                ant_route.push(Path::new(cur_node.clone(), next_node.clone()));
                cur_node = next_node;
            };

            if !path_found {
                continue;
            }

            // Backward
            ant_route.push(Path::new(cur_node.clone(), end_node.clone()));
            cur_node = end_node.clone();
            path_found = loop {
                if self.has_sight(self.node_to_world_pos(cur_node.clone()), start, grid_map) {
                    break true;
                }

                // Debug ======
                draw_temporary_dot(self.node_to_world_pos(cur_node.clone()), GRAY, 10.0, 1.0);

                try_count += 1;
                if try_count > self.max_ant_try {
                    break false;
                };

                let Some(next_node) = self.cal_next_node(cur_node.clone(), grid_map, &visited, &path_pheromone, &problem)
                else {
                    self.add_pheromone(&ant_route, -route_len, index, &mut path_pheromone);
                    break false;
                };

                route_len += self
                    .node_to_world_pos(cur_node.clone())
                    .distance(self.node_to_world_pos(next_node.clone())) as f64;
                ant_route.push(Path::new(cur_node.clone(), next_node.clone()));
                cur_node = next_node;
            };

            if path_found {
                self.evaporate_pheromone(&mut path_pheromone);
                self.add_pheromone(&ant_route, route_len, index, &mut path_pheromone);
                foundcnt += 1;
            }
        }
        std::println!("found {}", foundcnt);

        let mut aco_route: Vec<Vec2> = Vec::new();
        let mut aco_visited: HashSet<Node> = HashSet::new();
        let mut aco_cur_node = start_node.clone();
        aco_visited.insert(aco_cur_node.clone());
        while !self.has_sight(self.node_to_world_pos(aco_cur_node.clone()), end, grid_map) {
            let near_nodes = self.near_node_view(aco_cur_node.clone());
            let mut best = (near_nodes.first().unwrap().clone(), -1.0);
            for next_node in near_nodes {
                if aco_visited.contains(&next_node.clone()) {
                    continue;
                }
                if !self.has_sight(
                    self.node_to_world_pos(aco_cur_node.clone()),
                    self.node_to_world_pos(next_node.clone()),
                    grid_map,
                ) {
                    continue;
                }

                let pheromone = *path_pheromone
                    .get(&Path::new(aco_cur_node.clone(), next_node.clone()))
                    .unwrap_or(&0.0);
                if pheromone > best.1 {
                    best = (next_node.clone(), pheromone);
                }
            }
            let next_node = best.0;

            aco_route.push(self.node_to_world_pos(next_node.clone()));
            aco_visited.insert(next_node.clone());
            aco_cur_node = next_node;
        }

        let mut result: Vec<Vec2> = Vec::new();
        result.push(start);
        result.extend(aco_route);
        result.push(end);

        result
    }
}

impl AcoPsoStrategy {
    fn world_to_node_pos(&self, wpos: Vec2) -> Option<Node> {
        if wpos.is_nan() {
            None
        } else {
            Some(Node::new(
                (wpos.x / self.node_min_dist).round() as i32,
                (wpos.y / self.node_min_dist).round() as i32,
            ))
        }
    }
    fn node_to_world_pos(&self, npos: Node) -> Vec2 {
        Vec2::new(npos.pos.0 as f32 * self.node_min_dist, npos.pos.1 as f32 * self.node_min_dist)
    }

    fn near_node_view(&self, npos: Node) -> Vec<Node> {
        let mut v = Vec::new();
        v.push(Node::new(npos.pos.0 + 1, npos.pos.1 + 0));
        // v.push(Node::new(npos.pos.0 + 1, npos.pos.1 - 1));
        v.push(Node::new(npos.pos.0 + 0, npos.pos.1 - 1));
        // v.push(Node::new(npos.pos.0 - 1, npos.pos.1 - 1));
        v.push(Node::new(npos.pos.0 - 1, npos.pos.1 + 0));
        // v.push(Node::new(npos.pos.0 - 1, npos.pos.1 + 1));
        v.push(Node::new(npos.pos.0 + 0, npos.pos.1 + 1));
        // v.push(Node::new(npos.pos.0 + 1, npos.pos.1 + 1));
        v
    }

    fn has_sight(&self, from: Vec2, to: Vec2, grid_map: &GridMap) -> bool {
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
}

impl AcoPsoStrategy {
    fn cal_path_value(&self, path: Path, path_pheromones: &HashMap<Path, f64>, problem: &Problem) -> f64 {
        path_pheromones.get(&path).unwrap_or(&self.init_pheromone).powf(self.alpha)
            * (1.0 / self.node_to_world_pos(path.to).distance(problem.end) as f64).powf(self.beta)
    }

    fn cal_next_node(
        &self,
        from: Node,
        grid_map: &GridMap,
        visited: &HashSet<Node>,
        path_pheromones: &HashMap<Path, f64>,
        problem: &Problem,
    ) -> Option<Node> {
        let mut total_pheromone: f64 = 0.0;
        let mut node_pheromones: Vec<(Node, f64)> = Vec::new();
        node_pheromones.reserve(8);

        let near_nodes = self.near_node_view(from.clone());
        for next_node in near_nodes {
            if visited.contains(&next_node.clone()) {
                continue;
            }
            if !self.has_sight(
                self.node_to_world_pos(from.clone()),
                self.node_to_world_pos(next_node.clone()),
                grid_map,
            ) {
                continue;
            }

            let path = Path::new(from.clone(), next_node.clone());
            let pheromone = self.cal_path_value(path, &path_pheromones, &problem);
            node_pheromones.push((next_node.clone(), pheromone));
            total_pheromone += pheromone;
        }

        if node_pheromones.is_empty() {
            return None;
        }

        let random: f64 = rand::gen_range(0.0, total_pheromone);

        let mut accumulated_probability = 0.0;
        for (node, pheromone) in &node_pheromones {
            accumulated_probability += pheromone;
            if random <= accumulated_probability {
                return Some(node.clone());
            }
        }

        Some(node_pheromones.last().unwrap().0.clone())
    }

    fn evaporate_pheromone(&self, path_pheromone: &mut HashMap<Path, f64>) {
        let _ = path_pheromone.iter_mut().map(|x| *x.1 *= 1.0 - self.evaporation);
    }
    fn add_pheromone(&self, route: &[Path], route_len: f64, route_index: u32, path_pheromone: &mut HashMap<Path, f64>) {
        for path in route {
            if let Some(pheromone) = path_pheromone.get_mut(path) {
                *pheromone += self.deposit_constant / route_len;
            } else {
                path_pheromone.insert(
                    path.clone(),
                    self.init_pheromone * (1.0 - self.evaporation).powi(route_index as i32) + self.deposit_constant / route_len,
                );
            }
        }
    }
}
