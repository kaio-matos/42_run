use basis::prelude::*;

#[derive(Debug)]
pub struct PlayerCamera {
    pub position: Vec3,
    front: Vec3,
    up: Vec3,
    speed: f32,
}
impl Component for PlayerCamera {}

impl PlayerCamera {
    pub fn new(position: Vec3, front: Vec3, up: Vec3, speed: f32) -> Self {
        Self {
            position,
            front,
            up,
            speed,
        }
    }
}

impl Camerable for PlayerCamera {
    fn get_view_matrix(&self) -> Mat4 {
        Mat4::look_at(self.position, self.position + self.front, self.up)
    }
}

impl Controllable for PlayerCamera {
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
