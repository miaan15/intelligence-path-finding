//! Path representation and results

use crate::data_structures::Position;
use std::fmt;

/// Represents a path found by an algorithm
#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    pub positions: Vec<Position>,
    pub cost: f64,
}

impl Path {
    /// Create a new empty path
    pub fn new() -> Self {
        Self {
            positions: Vec::new(),
            cost: 0.0,
        }
    }

    /// Create a path with positions and calculate cost
    pub fn from_positions(mut positions: Vec<Position>) -> Self {
        let cost = Self::calculate_path_cost(&positions);
        positions.reverse(); // Reverse to get start-to-end order
        Self { positions, cost }
    }

    /// Calculate the total cost of a path (assuming unit cost between adjacent nodes)
    fn calculate_path_cost(positions: &[Position]) -> f64 {
        if positions.len() <= 1 {
            return 0.0;
        }

        let mut cost = 0.0;
        for window in positions.windows(2) {
            let pos1 = &window[0];
            let pos2 = &window[1];

            // Use Manhattan distance as default cost
            cost += pos1.manhattan_distance(pos2) as f64;
        }
        cost
    }

    /// Get the length of the path
    pub fn len(&self) -> usize {
        self.positions.len()
    }

    /// Check if the path is empty
    pub fn is_empty(&self) -> bool {
        self.positions.is_empty()
    }

    /// Get the start position of the path
    pub fn start(&self) -> Option<&Position> {
        self.positions.first()
    }

    /// Get the end position of the path
    pub fn end(&self) -> Option<&Position> {
        self.positions.last()
    }

    /// Check if a position is in the path
    pub fn contains(&self, position: &Position) -> bool {
        self.positions.contains(position)
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Path: {} steps, cost: {:.2}", self.len(), self.cost)?;
        if !self.is_empty() {
            write!(f, ", from {:?} to {:?}", self.start().unwrap(), self.end().unwrap())?;
        }
        Ok(())
    }
}

/// Result type for path-finding operations
#[derive(Debug, Clone, PartialEq)]
pub enum PathResult {
    /// Path found successfully
    Found(Path),
    /// No path exists between start and end
    NoPath,
    /// Start or end position is invalid
    InvalidPosition,
    /// Algorithm failed due to other reasons
    Failed(String),
}

impl PathResult {
    /// Check if a path was found
    pub fn is_found(&self) -> bool {
        matches!(self, PathResult::Found(_))
    }

    /// Get the path if found
    pub fn path(&self) -> Option<&Path> {
        match self {
            PathResult::Found(path) => Some(path),
            _ => None,
        }
    }

    /// Get the path length if found
    pub fn path_length(&self) -> Option<usize> {
        self.path().map(|p| p.len())
    }

    /// Get the path cost if found
    pub fn path_cost(&self) -> Option<f64> {
        self.path().map(|p| p.cost)
    }
}

impl fmt::Display for PathResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PathResult::Found(path) => write!(f, "✓ {}", path),
            PathResult::NoPath => write!(f, "✗ No path exists"),
            PathResult::InvalidPosition => write!(f, "✗ Invalid start or end position"),
            PathResult::Failed(msg) => write!(f, "✗ Algorithm failed: {}", msg),
        }
    }
}