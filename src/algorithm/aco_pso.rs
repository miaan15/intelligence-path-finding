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
    pub node_dist: f32,

    pub alpha: f64,
    pub beta: f64,
    pub evaporation: f64,
    pub deposit_constant: f64,

    pub init_pheromone: f64,
    pub min_ant_count: u32,
    pub max_ant_try: u32,
}

impl Strategy for AcoPsoStrategy {
    fn path_finding(&self, problem: &Problem) -> Option<Vec<Vec2>> {
        let grid_map = problem.grid_map();
        let start = problem.start();
        let goal = problem.goal();

        let start_node = self.world_to_node_pos(start).unwrap();
        let goal_node = self.world_to_node_pos(goal).unwrap();

        let mut path_pheromones: HashMap<Path, f64> = HashMap::new();

        let mut best_aco_route: Option<Vec<Node>> = None;

        for _ in 0..self.min_ant_count {
            let mut route: Vec<Node> = Vec::new();
            let mut visited: HashSet<Node> = HashSet::new();

            let mut cur_node = start_node.clone();
            route.push(cur_node.clone());
            visited.insert(cur_node.clone());
            let mut try_count: i32 = self.max_ant_try as i32;
            let path_found = loop {
                let mut node_desires: Vec<(Node, f64)> = Vec::new();
                let mut total_desire = 0.0;
                for next_node in self.next_node_list(cur_node.clone()) {
                    let cur_pos = self.node_to_world_pos(cur_node.clone());
                    let next_pos = self.node_to_world_pos(next_node.clone());

                    if !self.has_sight(cur_pos.clone(), next_pos.clone(), &grid_map) {
                        continue;
                    }

                    let mut desire =
                        self.path_desire(Path::new(cur_node.clone(), next_node.clone()), &path_pheromones, goal.clone());

                    if visited.contains(&next_node) {
                        desire *= 0.001;
                    }

                    node_desires.push((next_node, desire));
                    total_desire += desire;
                }
                if node_desires.is_empty() {
                    break false;
                }
                let next_node = self.get_next_node(&node_desires, total_desire);
                cur_node = next_node;

                route.push(cur_node.clone());
                visited.insert(cur_node.clone());

                // Debug
                draw_temporary_dot(self.node_to_world_pos(cur_node.clone()), WHITE, 10.0, 1.0);
                //

                if cur_node == goal_node {
                    break true;
                }

                try_count -= 1;
                if try_count <= 0 {
                    break false;
                }
            };

            if path_found {
                self.update_pheromone(&route, route.len() as f64 * self.node_dist as f64, &mut path_pheromones);

                if best_aco_route.is_none() || best_aco_route.as_ref().unwrap().len() > route.len() {
                    best_aco_route = Some(route);
                }
            }
        }

        if let Some(best_route) = best_aco_route {
            Some(best_route.iter().map(|x| self.node_to_world_pos(x.clone())).collect())
        } else {
            None
        }
    }
}

impl AcoPsoStrategy {
    fn world_to_node_pos(&self, wpos: Vec2) -> Option<Node> {
        if wpos.is_nan() {
            None
        } else {
            Some(Node::new(
                ((wpos.x - self.node_dist / 2.0) / self.node_dist).round() as i32,
                ((wpos.y - self.node_dist / 2.0) / self.node_dist).round() as i32,
            ))
        }
    }
    fn node_to_world_pos(&self, npos: Node) -> Vec2 {
        Vec2::new(
            npos.pos.0 as f32 * self.node_dist + self.node_dist / 2.0,
            npos.pos.1 as f32 * self.node_dist + self.node_dist / 2.0,
        )
    }

    fn next_node_list(&self, npos: Node) -> Vec<Node> {
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
    fn path_desire(&self, path: Path, path_pheromones: &HashMap<Path, f64>, goal: Vec2) -> f64 {
        let pheromone = path_pheromones.get(&path).unwrap_or(&self.init_pheromone).powf(self.alpha);
        let heuristic = (1.0 / self.node_to_world_pos(path.to).distance(goal) as f64).powf(self.beta);
        pheromone * heuristic
    }

    fn get_next_node(&self, node_desires: &[(Node, f64)], total_desire: f64) -> Node {
        let random: f64 = rand::gen_range(0.0, total_desire);

        let mut accumulated_probability = 0.0;
        for (node, disire) in node_desires {
            accumulated_probability += disire;
            if random <= accumulated_probability {
                return node.clone();
            }
        }

        node_desires.last().unwrap().0.clone()
    }

    fn update_pheromone(&self, route: &[Node], route_len: f64, path_pheromones: &mut HashMap<Path, f64>) {
        path_pheromones.iter_mut().for_each(|(_, pheromone)| {
            *pheromone *= 1.0 - self.evaporation;
        });

        for nodes in route.windows(2) {
            let path = Path::new(nodes[0].clone(), nodes[1].clone());
            if let Some(pheromone) = path_pheromones.get_mut(&path) {
                *pheromone += self.deposit_constant / route_len;
            } else {
                path_pheromones.insert(path.clone(), self.init_pheromone + self.deposit_constant / route_len);
            }
        }
    }
}
