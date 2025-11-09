use crate::game::game::RenderConfig;
use crate::world::grid::GridMap;
use crate::world::grid::GridNodeValue;
use macroquad::prelude::*;

pub struct MapRenderer {
    config: RenderConfig,
    mesh: Option<Mesh>,

    max_bound: Vec2,
}

impl MapRenderer {
    pub fn new(config: RenderConfig) -> Self {
        Self {
            config,
            mesh: None,
            max_bound: Vec2::ZERO,
        }
    }

    pub fn reset_mesh(&mut self, map: &GridMap) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for y in 0..map.height() {
            for x in 0..map.width() {
                if let Some(GridNodeValue::Obstacle) = map.grid().get(x, y) {
                    let world_x = x as f32;
                    let world_y = y as f32;

                    self.max_bound.x = self.max_bound.x.max(world_x + 1.0);
                    self.max_bound.y = self.max_bound.y.max(world_y + 1.0);

                    let vertex_offset = vertices.len() as u16;
                    vertices.push(Vertex {
                        position: Vec3::new(world_x, world_y, 0.0),
                        normal: Vec4::new(0.0, 0.0, 1.0, 1.0),
                        uv: Vec2::new(0.0, 1.0),
                        color: self.config.obstacle_color.into(),
                    });
                    vertices.push(Vertex {
                        position: Vec3::new(world_x + 1.0, world_y, 0.0),
                        normal: Vec4::new(0.0, 0.0, 1.0, 1.0),
                        uv: Vec2::new(1.0, 1.0),
                        color: self.config.obstacle_color.into(),
                    });
                    vertices.push(Vertex {
                        position: Vec3::new(world_x + 1.0, world_y + 1.0, 0.0),
                        normal: Vec4::new(0.0, 0.0, 1.0, 1.0),
                        uv: Vec2::new(1.0, 0.0),
                        color: self.config.obstacle_color.into(),
                    });
                    vertices.push(Vertex {
                        position: Vec3::new(world_x, world_y + 1.0, 0.0),
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

    pub fn mesh(&self) -> Option<&Mesh> { self.mesh.as_ref() }
    pub fn max_bound(&self) -> Vec2 { self.max_bound }
}
