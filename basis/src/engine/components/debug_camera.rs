use crate::prelude::*;
use crate::{engine::behaviours::Controllable, graphics};

#[derive(Debug)]
pub struct DebugCamera {
    pub position: Vec3,
    front: Vec3,
    up: Vec3,
    speed: f32,
}

impl DebugCamera {
    pub fn new(position: Vec3, front: Vec3, up: Vec3, speed: f32) -> Self {
        Self {
            position,
            front,
            up,
            speed,
        }
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        Mat4::look_at(self.position, self.position + self.front, self.up)
    }
}

impl Controllable for DebugCamera {
    fn get_speed(&self, deltatime: f32) -> f32 {
        self.speed * deltatime
    }

    fn move_forward(&mut self, deltatime: f32) {
        self.position = self.position + self.front.scale(self.get_speed(deltatime));
    }

    fn move_backward(&mut self, deltatime: f32) {
        self.position = self.position - self.front.scale(self.get_speed(deltatime));
    }

    fn move_left(&mut self, deltatime: f32) {
        self.position = self.position
            - self
                .front
                .cross(self.up)
                .normalize()
                .scale(self.get_speed(deltatime));
    }

    fn move_right(&mut self, deltatime: f32) {
        self.position = self.position
            + self
                .front
                .cross(self.up)
                .normalize()
                .scale(self.get_speed(deltatime));
    }

    fn move_up(&mut self, deltatime: f32) {
        self.position = self.position + self.up.scale(self.get_speed(deltatime));
    }

    fn move_down(&mut self, deltatime: f32) {
        self.position = self.position - self.up.scale(self.get_speed(deltatime));
    }

    #[warn(dead_code)]
    fn rotate(&mut self, _deltatime: f32, yaw: f32, pitch: f32) {
        let yawr = yaw.to_radians();
        let pitchr = pitch.to_radians();
        let mut direction = Vec3::splat(0.);
        direction.x = yawr.cos() * pitchr.sin();
        direction.y = pitchr.sin();
        direction.z = yawr.sin() * pitchr.sin();
    }

    fn rotateq(&mut self, _deltatime: f32, _quaternion: Quaternion) {}
}

impl EntityLifetime for DebugCamera {
    fn update(&mut self, window: &mut graphics::window::Window) {
        if window.on_key_hold(graphics::glfw::Key::W, graphics::glfw::Modifiers::empty()) {
            self.move_up(window.deltatime)
        }
        if window.on_key_hold(graphics::glfw::Key::S, graphics::glfw::Modifiers::empty()) {
            self.move_down(window.deltatime)
        }
        if window.on_key_hold(graphics::glfw::Key::A, graphics::glfw::Modifiers::empty()) {
            self.move_left(window.deltatime)
        }
        if window.on_key_hold(graphics::glfw::Key::D, graphics::glfw::Modifiers::empty()) {
            self.move_right(window.deltatime)
        }
        if window.on_key_hold(graphics::glfw::Key::W, graphics::glfw::Modifiers::Control) {
            self.move_forward(window.deltatime)
        }
        if window.on_key_hold(graphics::glfw::Key::S, graphics::glfw::Modifiers::Control) {
            self.move_backward(window.deltatime)
        }
    }

    fn postupdate(
        &mut self,
        _window: &mut graphics::window::Window,
        shader: &graphics::glw::Shader,
    ) {
        shader.bind();
        shader
            .get_uniform_location("view")
            .uniform_matrix4fv(&self.get_view_matrix());
        shader.unbind();
    }
}
