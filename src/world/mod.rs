pub mod grid;
pub mod types;

#[derive(Debug, Clone)]
pub struct WorldConfig {
    pub grid_size: (usize, usize),
    pub cell_size: f32,
}
