use crate::game::RenderConfig;
use crate::world::grid::GridMap;
use macroquad::prelude::*;

pub struct GridRenderer {
    mesh: Option<Mesh>,
    config: RenderConfig,
}

impl GridRenderer {
    pub fn new(config: RenderConfig) -> Self { Self { mesh: None, config } }

    pub fn update_mesh(&mut self, map: &GridMap) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let cell_size = map.cell_size();

        for y in 0..map.height() {
            for x in 0..map.width() {
                if let Some(crate::world::grid::GridNodeValue::Obstacle) = map.grid().get(x, y) {
                    let world_x = x as f32 * cell_size;
                    let world_y = y as f32 * cell_size;

                    // Add vertices for this cell
                    let vertex_offset = vertices.len() as u16;
                    vertices.push(Vertex {
                        position: Vec3::new(world_x, world_y, 0.0),
                        normal: Vec4::new(0.0, 0.0, 1.0, 1.0),
                        uv: Vec2::new(0.0, 1.0),
                        color: self.config.obstacle_color.into(),
                    });
                    vertices.push(Vertex {
                        position: Vec3::new(world_x + cell_size, world_y, 0.0),
                        normal: Vec4::new(0.0, 0.0, 1.0, 1.0),
                        uv: Vec2::new(1.0, 1.0),
                        color: self.config.obstacle_color.into(),
                    });
                    vertices.push(Vertex {
                        position: Vec3::new(world_x + cell_size, world_y + cell_size, 0.0),
                        normal: Vec4::new(0.0, 0.0, 1.0, 1.0),
                        uv: Vec2::new(1.0, 0.0),
                        color: self.config.obstacle_color.into(),
                    });
                    vertices.push(Vertex {
                        position: Vec3::new(world_x, world_y + cell_size, 0.0),
                        normal: Vec4::new(0.0, 0.0, 1.0, 1.0),
                        uv: Vec2::new(0.0, 0.0),
                        color: self.config.obstacle_color.into(),
                    });

                    indices.push(vertex_offset);
                    indices.push(vertex_offset + 1);
                    indices.push(vertex_offset + 2);

                    indices.push(vertex_offset);
                    indices.push(vertex_offset + 2);
                    indices.push(vertex_offset + 3);
                }
            }
        }

        if !vertices.is_empty() {
            let mesh = Mesh {
                vertices,
                indices,
                texture: None,
            };
            self.mesh = Some(mesh);
        } else {
            self.mesh = None;
        }
    }

    pub fn draw(&self) {
        if let Some(mesh) = &self.mesh {
            draw_mesh(mesh);
        }
    }

    pub fn update_config(&mut self, config: RenderConfig) { self.config = config; }
}
