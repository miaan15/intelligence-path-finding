//! Generic graph data structure for path-finding

use crate::data_structures::{Node, Position};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// Edge in a weighted graph
#[derive(Debug, Clone)]
pub struct Edge<T> {
    pub from: T,
    pub to: T,
    pub weight: f64,
}

impl<T> Edge<T> {
    pub fn new(from: T, to: T, weight: f64) -> Self {
        Self { from, to, weight }
    }
}

/// Generic weighted graph
#[derive(Debug, Clone)]
pub struct Graph<T>
where
    T: Clone + Eq + Hash,
{
    nodes: HashMap<T, NodeData<T>>,
    adjacency_list: HashMap<T, Vec<Edge<T>>>,
}

#[derive(Debug, Clone)]
struct NodeData<T> {
    position: T,
    walkable: bool,
    data: Option<T>,
}

impl<T> Graph<T>
where
    T: Clone + Eq + Hash,
{
    /// Create a new empty graph
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            adjacency_list: HashMap::new(),
        }
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, node: T, walkable: bool) {
        self.nodes.insert(
            node.clone(),
            NodeData {
                position: node.clone(),
                walkable,
                data: None,
            },
        );
        self.adjacency_list.insert(node, Vec::new());
    }

    /// Add an edge between two nodes
    pub fn add_edge(&mut self, from: T, to: T, weight: f64) {
        if let Some(edges) = self.adjacency_list.get_mut(&from) {
            edges.push(Edge::new(from.clone(), to.clone(), weight));
        }
    }

    /// Add a bidirectional edge between two nodes
    pub fn add_bidirectional_edge(&mut self, node1: T, node2: T, weight: f64) {
        self.add_edge(node1.clone(), node2.clone(), weight);
        self.add_edge(node2, node1, weight);
    }

    /// Check if a node exists in the graph
    pub fn has_node(&self, node: &T) -> bool {
        self.nodes.contains_key(node)
    }

    /// Check if a node is walkable
    pub fn is_walkable(&self, node: &T) -> bool {
        self.nodes
            .get(node)
            .map(|data| data.walkable)
            .unwrap_or(false)
    }

    /// Get neighbors of a node
    pub fn get_neighbors(&self, node: &T) -> Vec<&Edge<T>> {
        self.adjacency_list
            .get(node)
            .map(|edges| edges.iter().collect())
            .unwrap_or_default()
    }

    /// Get all nodes in the graph
    pub fn nodes(&self) -> Vec<&T> {
        self.nodes.keys().collect()
    }

    /// Get all walkable nodes in the graph
    pub fn walkable_nodes(&self) -> Vec<&T> {
        self.nodes
            .iter()
            .filter(|(_, data)| data.walkable)
            .map(|(node, _)| node)
            .collect()
    }

    /// Set walkable status for a node
    pub fn set_walkable(&mut self, node: &T, walkable: bool) {
        if let Some(data) = self.nodes.get_mut(node) {
            data.walkable = walkable;
        }
    }

    /// Remove an edge from the graph
    pub fn remove_edge(&mut self, from: &T, to: &T) {
        if let Some(edges) = self.adjacency_list.get_mut(from) {
            edges.retain(|edge| edge.to != *to);
        }
    }

    /// Remove a node from the graph
    pub fn remove_node(&mut self, node: &T) {
        self.nodes.remove(node);
        self.adjacency_list.remove(node);

        // Remove all edges pointing to this node
        for edges in self.adjacency_list.values_mut() {
            edges.retain(|edge| edge.to != *node);
        }
    }

    /// Get the number of nodes in the graph
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Get the number of edges in the graph
    pub fn edge_count(&self) -> usize {
        self.adjacency_list.values().map(|edges| edges.len()).sum()
    }

    /// Check if the graph is empty
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Clear all nodes and edges from the graph
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.adjacency_list.clear();
    }
}

impl<T> Default for Graph<T>
where
    T: Clone + Eq + Hash,
{
    fn default() -> Self {
        Self::new()
    }
}

/// Create a graph from a grid
impl From<&crate::data_structures::Grid> for Graph<Position> {
    fn from(grid: &crate::data_structures::Grid) -> Self {
        let mut graph = Graph::new();
        let (width, height) = grid.dimensions();

        // Add all nodes
        for y in 0..height {
            for x in 0..width {
                let pos = Position::new(x, y);
                graph.add_node(pos, grid.is_walkable(&pos));
            }
        }

        // Add edges for adjacent nodes
        for y in 0..height {
            for x in 0..width {
                let pos = Position::new(x, y);
                if !grid.is_walkable(&pos) {
                    continue;
                }

                for neighbor_pos in grid.get_neighbors(&pos) {
                    if grid.is_walkable(&neighbor_pos) {
                        // Use Manhattan distance as weight
                        let weight = pos.manhattan_distance(&neighbor_pos) as f64;
                        graph.add_edge(pos, neighbor_pos, weight);
                    }
                }
            }
        }

        graph
    }
}