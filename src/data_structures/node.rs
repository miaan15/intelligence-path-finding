//! Node and position representations

use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::hash::{Hash, Hasher};

/// Represents a 2D position in the grid
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// Calculate Manhattan distance to another position
    pub fn manhattan_distance(&self, other: &Position) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    /// Calculate Euclidean distance to another position
    pub fn euclidean_distance(&self, other: &Position) -> f64 {
        let dx = self.x as f64 - other.x as f64;
        let dy = self.y as f64 - other.y as f64;
        (dx * dx + dy * dy).sqrt()
    }
}

/// Represents a node in the path-finding graph
#[derive(Debug, Clone)]
pub struct Node {
    pub position: Position,
    pub g_cost: f64,  // Cost from start
    pub h_cost: f64,  // Heuristic cost to end
    pub f_cost: f64,  // Total cost (g + h)
    pub parent: Option<Position>,
    pub walkable: bool,
}

impl Node {
    pub fn new(position: Position) -> Self {
        Self {
            position,
            g_cost: 0.0,
            h_cost: 0.0,
            f_cost: 0.0,
            parent: None,
            walkable: true,
        }
    }

    pub fn set_costs(&mut self, g_cost: f64, h_cost: f64) {
        self.g_cost = g_cost;
        self.h_cost = h_cost;
        self.f_cost = g_cost + h_cost;
    }

    pub fn is_walkable(&self) -> bool {
        self.walkable
    }

    pub fn set_walkable(&mut self, walkable: bool) {
        self.walkable = walkable;
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.position == other.position
    }
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        // Reverse order for min-heap behavior
        other.f_cost.partial_cmp(&self.f_cost).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.hash(state);
    }
}