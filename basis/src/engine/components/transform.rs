use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quaternion,
    pub scale: Vec3,
}
impl Component for Transform {}

impl Transform {
    pub fn translate(&mut self, new_pos: Vec3) {
        self.position = new_pos;
    }

    pub fn scale(&mut self, scale: Vec3) {
        self.scale = scale;
    }

    pub fn center(&self, object_center: Vec3) -> Vec3 {
        object_center * self.scale // scale by the object's scale
    }
}

impl Controllable for Transform {
    fn get_speed(&self, deltatime: f32) -> f32 {
        30. * deltatime
    }

    fn move_forward(&mut self, deltatime: f32) {
        self.position.z -= self.get_speed(deltatime);
    }

    fn move_backward(&mut self, deltatime: f32) {
        self.position.z += self.get_speed(deltatime);
    }

    fn move_left(&mut self, deltatime: f32) {
        self.position.x -= self.get_speed(deltatime);
    }

    fn move_right(&mut self, deltatime: f32) {
        self.position.x += self.get_speed(deltatime);
    }

    fn move_up(&mut self, deltatime: f32) {
        self.position.y += self.get_speed(deltatime);
    }

    fn move_down(&mut self, deltatime: f32) {
        self.position.y -= self.get_speed(deltatime);
    }

    fn rotate(&mut self, _deltatime: f32, _yaw: f32, _pitch: f32) {}

    fn rotateq(&mut self, deltatime: f32, quaternion: Quaternion) {
        self.rotation
            .rotate_mut(quaternion * self.get_speed(deltatime));
    }
}
