use crate::vertex::Vertex;

pub struct Cube;

impl Cube {
    pub fn left() -> [Vertex; 4] {
        [
            Vertex {
                position: [0.0, 1.0, 1.0],
                normal: [-1.0, 0.0, 0.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [0.0, 0.0, 1.0],
                normal: [-1.0, 0.0, 0.0],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [0.0, 1.0, 0.0],
                normal: [-1.0, 0.0, 0.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [0.0, 0.0, 0.0],
                normal: [-1.0, 0.0, 0.0],
                tex_coords: [1.0, 1.0],
            },
        ]
    }

    pub fn right() -> [Vertex; 4] {
        [
            Vertex {
                position: [1.0, 1.0, 0.0],
                normal: [1.0, 0.0, 0.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [1.0, 0.0, 0.0],
                normal: [1.0, 0.0, 0.0],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [1.0, 1.0, 1.0],
                normal: [1.0, 0.0, 0.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [1.0, 0.0, 1.0],
                normal: [1.0, 0.0, 0.0],
                tex_coords: [1.0, 1.0],
            },
        ]
    }

    pub fn top() -> [Vertex; 4] {
        [
            Vertex {
                position: [0.0, 1.0, 1.0],
                normal: [0.0, 1.0, 0.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [0.0, 1.0, 0.0],
                normal: [0.0, 1.0, 0.0],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [1.0, 1.0, 1.0],
                normal: [0.0, 1.0, 0.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [1.0, 1.0, 0.0],
                normal: [0.0, 1.0, 0.0],
                tex_coords: [1.0, 1.0],
            },
        ]
    }

    pub fn bottom() -> [Vertex; 4] {
        [
            Vertex {
                position: [0.0, 0.0, 0.0],
                normal: [0.0, -1.0, 0.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [0.0, 0.0, 1.0],
                normal: [0.0, -1.0, 0.0],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [1.0, 0.0, 0.0],
                normal: [0.0, -1.0, 0.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [1.0, 0.0, 1.0],
                normal: [0.0, -1.0, 0.0],
                tex_coords: [1.0, 1.0],
            },
        ]
    }

    pub fn front() -> [Vertex; 4] {
        [
            Vertex {
                position: [0.0, 1.0, 0.0],
                normal: [0.0, 0.0, -1.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [0.0, 0.0, 0.0],
                normal: [0.0, 0.0, -1.0],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [1.0, 1.0, 0.0],
                normal: [0.0, 0.0, -1.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [1.0, 0.0, 0.0],
                normal: [0.0, 0.0, -1.0],
                tex_coords: [1.0, 1.0],
            },
        ]
    }

    pub fn back() -> [Vertex; 4] {
        [
            Vertex {
                position: [1.0, 1.0, 1.0],
                normal: [0.0, 0.0, 1.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [1.0, 0.0, 1.0],
                normal: [0.0, 0.0, 1.0],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [0.0, 1.0, 1.0],
                normal: [0.0, 0.0, 1.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [0.0, 0.0, 1.0],
                normal: [0.0, 0.0, 1.0],
                tex_coords: [1.0, 1.0],
            },
        ]
    }
}
