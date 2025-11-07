//! 2D grid representation for path-finding

use crate::data_structures::{Node, Position};
use std::collections::HashMap;

/// Represents a 2D grid for path-finding
#[derive(Debug, Clone)]
pub struct Grid {
    width: usize,
    height: usize,
    nodes: HashMap<Position, Node>,
}

impl Grid {
    /// Create a new grid with specified dimensions
    pub fn new(width: usize, height: usize) -> Self {
        let mut nodes = HashMap::new();

        for y in 0..height {
            for x in 0..width {
                let position = Position::new(x, y);
                nodes.insert(position, Node::new(position));
            }
        }

        Self {
            width,
            height,
            nodes,
        }
    }

    /// Get grid dimensions
    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    /// Get a node at the specified position
    pub fn get_node(&self, position: &Position) -> Option<&Node> {
        self.nodes.get(position)
    }

    /// Get a mutable node at the specified position
    pub fn get_node_mut(&mut self, position: &Position) -> Option<&mut Node> {
        self.nodes.get_mut(position)
    }

    /// Check if a position is valid within the grid
    pub fn is_valid_position(&self, position: &Position) -> bool {
        position.x < self.width && position.y < self.height
    }

    /// Get all valid neighbors of a position (4-directional movement)
    pub fn get_neighbors(&self, position: &Position) -> Vec<Position> {
        let mut neighbors = Vec::new();
        let directions = [
            (0, 1),   // Down
            (1, 0),   // Right
            (0, -1),  // Up
            (-1, 0),  // Left
        ];

        for (dx, dy) in directions {
            let new_x = position.x as isize + dx;
            let new_y = position.y as isize + dy;

            if new_x >= 0 && new_y >= 0 {
                let new_pos = Position::new(new_x as usize, new_y as usize);
                if self.is_valid_position(&new_pos) {
                    neighbors.push(new_pos);
                }
            }
        }

        neighbors
    }

    /// Get all valid neighbors of a position (8-directional movement)
    pub fn get_neighbors_diagonal(&self, position: &Position) -> Vec<Position> {
        let mut neighbors = Vec::new();
        let directions = [
            (0, 1),   // Down
            (1, 0),   // Right
            (0, -1),  // Up
            (-1, 0),  // Left
            (1, 1),   // Down-Right
            (1, -1),  // Up-Right
            (-1, 1),  // Down-Left
            (-1, -1), // Up-Left
        ];

        for (dx, dy) in directions {
            let new_x = position.x as isize + dx;
            let new_y = position.y as isize + dy;

            if new_x >= 0 && new_y >= 0 {
                let new_pos = Position::new(new_x as usize, new_y as usize);
                if self.is_valid_position(&new_pos) {
                    neighbors.push(new_pos);
                }
            }
        }

        neighbors
    }

    /// Set walkable status for a position
    pub fn set_walkable(&mut self, position: &Position, walkable: bool) {
        if let Some(node) = self.nodes.get_mut(position) {
            node.set_walkable(walkable);
        }
    }

    /// Check if a position is walkable
    pub fn is_walkable(&self, position: &Position) -> bool {
        self.nodes
            .get(position)
            .map(|node| node.is_walkable())
            .unwrap_or(false)
    }

    /// Reset all nodes to their initial state
    pub fn reset(&mut self) {
        for node in self.nodes.values_mut() {
            node.g_cost = 0.0;
            node.h_cost = 0.0;
            node.f_cost = 0.0;
            node.parent = None;
        }
    }

    /// Create a grid from a 2D array of booleans (true = walkable)
    pub fn from_bool_array(data: &[Vec<bool>]) -> Option<Self> {
        if data.is_empty() || data[0].is_empty() {
            return None;
        }

        let height = data.len();
        let width = data[0].len();
        let mut grid = Self::new(width, height);

        for (y, row) in data.iter().enumerate() {
            for (x, &walkable) in row.iter().enumerate() {
                let position = Position::new(x, y);
                grid.set_walkable(&position, walkable);
            }
        }

        Some(grid)
    }

    /// Get all walkable positions in the grid
    pub fn walkable_positions(&self) -> Vec<Position> {
        self.nodes
            .iter()
            .filter(|(_, node)| node.is_walkable())
            .map(|(pos, _)| *pos)
            .collect()
    }
}