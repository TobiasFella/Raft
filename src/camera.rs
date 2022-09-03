use crate::chunk::Chunk;
use crate::math::Vec3;
use glium::glutin::event::VirtualKeyCode;
use std::collections::HashSet;

const PLAYER_CAMERA_HEIGHT: f32 = 1.8;
const JUMP: f32 = 0.03;

pub struct Camera {
    acceleration: f32,
    max_speed: f32,
    speed: f32,
    pub position: Vec3,
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
            position: Vec3(1.0, 3.0, 1.0),
            angle: (0.0, 0.0),
            gravity: 0.0003,
            vertical_speed: 0.0,
        }
    }
}

impl Camera {
    pub fn get_direction(&self) -> Vec3 {
        Vec3(0.0, 0.0, 1.0).rotate(self.angle.0, self.angle.1, 0.0)
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

        let mut movement_dir = Vec3(0.0, 0.0, 0.0);
        let camera_dir = self.get_direction();
        if pressed_keys.contains(&VirtualKeyCode::W) {
            movement_dir += camera_dir;
        }
        if pressed_keys.contains(&VirtualKeyCode::S) {
            movement_dir -= camera_dir;
        }
        let right =
            Vec3(camera_dir.0, 0.0, camera_dir.2).rotate(0.5 * std::f32::consts::PI, 0.0, 0.0);

        if pressed_keys.contains(&VirtualKeyCode::D) {
            movement_dir += right;
        }
        if pressed_keys.contains(&VirtualKeyCode::A) {
            movement_dir -= right
        }
        movement_dir.1 = 0.0;
        if movement_dir != Vec3(0.0, 0.0, 0.0) {
            movement_dir = movement_dir.normalize();
            movement_dir = movement_dir.scale(self.speed);
            self.position += movement_dir;
        }
        self.position.1 += self.vertical_speed;
    }
}
