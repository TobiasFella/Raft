use crate::cube::Cube;
use crate::vertex::Vertex;
use glium::Display;
use glium::IndexBuffer;

pub struct Chunk {
    pub blocks: [[[u8; 16]; 256]; 16],
}

impl Default for Chunk {
    fn default() -> Chunk {
        Chunk {
            blocks: [[[0; 16]; 256]; 16],
        }
    }
}

impl Chunk {
    pub fn prepare(
        &self,
        display: &Display,
    ) -> (glium::vertex::VertexBuffer<Vertex>, IndexBuffer<u32>) {
        let mut shape = vec![];
        let mut indices: Vec<u32> = vec![];
        let mut current_index = 0u32;
        for (x, x_slice) in self.blocks.iter().enumerate() {
            for (y, y_slice) in x_slice.iter().enumerate() {
                for (z, z_slice) in y_slice.iter().enumerate() {
                    if z_slice == &0 {
                        continue;
                    }
                    if x == 0 || self.blocks[x - 1][y][z] == 0 {
                        let left = Cube::left();
                        for mut vertex in left {
                            vertex.position[0] += x as f32;
                            vertex.position[1] += y as f32;
                            vertex.position[2] += z as f32;
                            shape.push(vertex);
                        }
                        indices.push(current_index);
                        indices.push(current_index + 1);
                        indices.push(current_index + 2);
                        indices.push(current_index + 1);
                        indices.push(current_index + 3);
                        indices.push(current_index + 2);
                        current_index += 4;
                    }
                    if x == 15 || self.blocks[x + 1][y][z] == 0 {
                        let right = Cube::right();
                        for mut vertex in right {
                            vertex.position[0] += x as f32;
                            vertex.position[1] += y as f32;
                            vertex.position[2] += z as f32;
                            shape.push(vertex);
                        }
                        indices.push(current_index);
                        indices.push(current_index + 1);
                        indices.push(current_index + 2);
                        indices.push(current_index + 1);
                        indices.push(current_index + 3);
                        indices.push(current_index + 2);
                        current_index += 4;
                    }
                    if y == 0 || self.blocks[x][y - 1][z] == 0 {
                        let bottom = Cube::bottom();
                        for mut vertex in bottom {
                            vertex.position[0] += x as f32;
                            vertex.position[1] += y as f32;
                            vertex.position[2] += z as f32;
                            shape.push(vertex);
                        }
                        indices.push(current_index);
                        indices.push(current_index + 1);
                        indices.push(current_index + 2);
                        indices.push(current_index + 1);
                        indices.push(current_index + 3);
                        indices.push(current_index + 2);
                        current_index += 4;
                    }
                    if y == 255 || self.blocks[x][y + 1][z] == 0 {
                        let top = Cube::top();
                        for mut vertex in top {
                            vertex.position[0] += x as f32;
                            vertex.position[1] += y as f32;
                            vertex.position[2] += z as f32;
                            shape.push(vertex);
                        }
                        indices.push(current_index);
                        indices.push(current_index + 1);
                        indices.push(current_index + 2);
                        indices.push(current_index + 1);
                        indices.push(current_index + 3);
                        indices.push(current_index + 2);
                        current_index += 4;
                    }
                    if z == 0 || self.blocks[x][y][z - 1] == 0 {
                        let front = Cube::front();
                        for mut vertex in front {
                            vertex.position[0] += x as f32;
                            vertex.position[1] += y as f32;
                            vertex.position[2] += z as f32;
                            shape.push(vertex);
                        }
                        indices.push(current_index);
                        indices.push(current_index + 1);
                        indices.push(current_index + 2);
                        indices.push(current_index + 1);
                        indices.push(current_index + 3);
                        indices.push(current_index + 2);
                        current_index += 4;
                    }
                    if z == 15 || self.blocks[x][y][z + 1] == 0 {
                        let back = Cube::back();
                        for mut vertex in back {
                            vertex.position[0] += x as f32;
                            vertex.position[1] += y as f32;
                            vertex.position[2] += z as f32;
                            shape.push(vertex);
                        }
                        indices.push(current_index);
                        indices.push(current_index + 1);
                        indices.push(current_index + 2);
                        indices.push(current_index + 1);
                        indices.push(current_index + 3);
                        indices.push(current_index + 2);
                        current_index += 4;
                    }
                }
            }
        }
        let shape = glium::vertex::VertexBuffer::new(display, &shape).unwrap();
        let indices = glium::index::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &indices,
        )
        .unwrap();
        (shape, indices)
    }
}
