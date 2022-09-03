use crate::chunk::Chunk;
use crate::rotate;
use glium::glutin::event::VirtualKeyCode;
use std::collections::HashSet;

const PLAYER_CAMERA_HEIGHT: f32 = 1.8;
const JUMP: f32 = 0.03;

pub struct Camera {
    acceleration: f32,
    max_speed: f32,
    speed: f32,
    pub position: (f32, f32, f32),
    angle: (f32, f32),
    gravity: f32,
    vertical_speed: f32,
}

impl Default for Camera {
    fn default() -> Camera {
        Camera {
            acceleration: 0.00002,
            speed: 0.0,
            max_speed: 0.005,
            position: (1.0, 3.0, 1.0),
            angle: (0.0, 0.0),
            gravity: 0.0003,
            vertical_speed: 0.0,
        }
    }
}

fn vec_len((x, y, z): (f32, f32, f32)) -> f32 {
    (x * x + y * y + z * z).sqrt()
}

fn normalize((x, y, z): (f32, f32, f32)) -> (f32, f32, f32) {
    let len = vec_len((x, y, z));
    (x / len, y / len, z / len)
}

impl Camera {
    pub fn get_direction(&self) -> (f32, f32, f32) {
        let vec = rotate([0.0, 0.0, 1.0], self.angle.1, self.angle.0, 0.0);
        (vec[0], vec[1], vec[2])
    }
    pub fn rotate(&mut self, (mut y_axis, mut x_axis): (f64, f64)) {
        y_axis *= 0.001;
        x_axis *= 0.001;
        let mut x_angle = x_axis as f32 + self.angle.1;
        if x_angle < -1.5 {
            x_angle = -1.5;
        } else if x_angle > 1.5 {
            x_angle = 1.5;
        }
        self.angle = (self.angle.0 + y_axis as f32, x_angle);
    }
    pub fn handle_keys(&mut self, pressed_keys: &HashSet<VirtualKeyCode>, chunk: &Chunk) {
        if pressed_keys.contains(&VirtualKeyCode::Space) {
            let x = self.position.0.floor() as usize;
            let y = self.position.1;
            let y = y - PLAYER_CAMERA_HEIGHT;
            let y = y.ceil() as usize;
            let z = self.position.2.floor() as usize;
            if chunk.blocks[x][y][z] != 0 {
                self.vertical_speed = JUMP;
            }
        }
        if pressed_keys.contains(&VirtualKeyCode::LShift) {}

        let mut any_nonconflicting = false;
        if pressed_keys.contains(&VirtualKeyCode::W) != pressed_keys.contains(&VirtualKeyCode::S) {
            any_nonconflicting = true;
        }
        if pressed_keys.contains(&VirtualKeyCode::A) != pressed_keys.contains(&VirtualKeyCode::D) {
            any_nonconflicting = true;
        }

        if any_nonconflicting && self.speed < self.max_speed {
            self.speed += self.acceleration;
        }

        let x = self.position.0.floor() as usize;
        let y = self.position.1;
        let y = y - PLAYER_CAMERA_HEIGHT;
        let y = y.ceil() as usize;
        let z = self.position.2.floor() as usize;
        if chunk.blocks[x][y][z] == 0 {
            self.vertical_speed -= self.gravity;
        } else if self.vertical_speed <= 0.0 {
            self.vertical_speed = 0.0;
        }

        let mut movement_dir = (0.0, 0.0, 0.0);
        let camera_dir = self.get_direction();
        if pressed_keys.contains(&VirtualKeyCode::W) {
            movement_dir = (
                movement_dir.0 + camera_dir.0,
                0.0,
                movement_dir.2 + camera_dir.2,
            );
        }
        if pressed_keys.contains(&VirtualKeyCode::S) {
            movement_dir = (
                movement_dir.0 - camera_dir.0,
                0.0,
                movement_dir.2 - camera_dir.2,
            );
        }
        let right = crate::rotate(
            [camera_dir.0, 0.0, camera_dir.2],
            0.0,
            0.5 * std::f32::consts::PI,
            0.0,
        );
        if pressed_keys.contains(&VirtualKeyCode::D) {
            movement_dir = (movement_dir.0 + right[0], 0.0, movement_dir.2 + right[2]);
        }
        if pressed_keys.contains(&VirtualKeyCode::A) {
            movement_dir = (movement_dir.0 - right[0], 0.0, movement_dir.2 - right[2]);
        }
        if movement_dir != (0.0, 0.0, 0.0) {
            movement_dir = normalize(movement_dir);
            movement_dir = (
                movement_dir.0 * self.speed,
                0.0,
                movement_dir.2 * self.speed,
            );
            self.position = (
                self.position.0 + movement_dir.0,
                self.position.1 + movement_dir.1,
                self.position.2 + movement_dir.2,
            );
        }
        self.position.1 += self.vertical_speed;
    }
}
