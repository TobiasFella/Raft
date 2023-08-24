use crate::chunk::Chunk;
use crate::object::Object;
use crate::world_gen::WorldGen;
use std::collections::HashMap;

pub struct World {
    pub chunks: HashMap<(i32, i32), Chunk>,
    block_types: HashMap<u8, Object>,
}

impl World {
    /// Returns a reference to the chunk at position (x, _, z) in world coordinates. Creates it if it doesn't exist yet
    pub fn chunk_at_pos_create(&mut self, x: i32, z: i32) -> &mut Chunk {
        if !self.chunk_exists(x / 16, z / 16) {
            self.create_empty_chunk(x / 16, z / 16);
        }
        self.chunks.get_mut(&(x / 16, z / 16)).unwrap()
    }
    /// Returns a reference to the chunk at position (x, _, z) in world coordinates, if it exists
    pub fn chunk_at_pos(&self, x: i32, z: i32) -> Option<&Chunk> {
        self.chunks.get(&(x / 16, z / 16))
    }
    /// Returns a mutable reference to the chunk at position (x, _, z) in world coordinates, if it exists
    // pub fn chunk_at_mut(&mut self, x: i32, z: i32) -> Option<&mut Chunk> {
    //     self.chunks.get_mut(&(x / 16, z / 16))
    // }
    /// Returns a reference to the object type at position (x, y, z) in world coordinates, if it exists
    pub fn block_at(&mut self, mut x: i32, y: i32, mut z: i32) -> Option<&Object> {
        let chunk_x = x / 16 - if x < 0 { 1 } else { 0 };
        let chunk_z = z / 16 - if z < 0 { 1 } else { 0 };
        //println!("chunk: {}, {}", chunk_x, chunk_z);
        if !self.chunk_exists(chunk_x, chunk_z) {
            println!("chunk doesn't exist");
            self.create_empty_chunk(chunk_x, chunk_z);
        }
        if let Some(chunk) = self.chunks.get(&(chunk_x, chunk_z)) {
            while x < 0 {
                //TODO improve
                x += 16
            }
            while z < 0 {
                z += 16
            }
            let id = chunk.blocks[(x % 16) as usize][y as usize][(z % 16) as usize];
            self.block_types.get(&id)
        } else {
            None
        }
    }
    /// Creates an empty chunk at **chunk position** (x, z)
    pub fn create_empty_chunk(&mut self, chunk_x: i32, chunk_z: i32) {
        println!("creating empty chunk");
        self.chunks.insert((chunk_x, chunk_z), Chunk::new());
    }
    /// Returns whether this chunk exists
    pub fn chunk_exists(&self, chunk_x: i32, chunk_z: i32) -> bool {
        self.chunks.contains_key(&(chunk_x, chunk_z))
    }

    pub fn new() -> Self {
        let mut world = World {
            chunks: HashMap::new(),
            block_types: HashMap::new(),
        };
        world.block_types.insert(1, Object {});
        world
    }
    pub fn generate_chunk(&mut self, chunk_x: i32, chunk_z: i32) {
        self.chunks
            .insert((chunk_x, chunk_z), WorldGen::generate_chunk());
    }
}
