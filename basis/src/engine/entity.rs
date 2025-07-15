use std::fmt::Debug;

use crate::{
    engine::EntitiesPending,
    graphics::{self, glw::Shader},
    prelude::Object,
};

#[derive(Debug)]
pub struct Entity {
    pub data: Box<dyn EntityLifetime>,
    pub id: u32,
}
impl Entity {
    pub fn new(entity: Box<dyn EntityLifetime>) -> Self {
        Self {
            id: rand::random(),
            data: entity,
        }
    }
}

impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub trait EntityLifetime: Debug {
    fn get_object(&mut self) -> Option<&mut Object> {
        None
    }
    fn setup(&mut self, _entities_pending: &mut EntitiesPending) {}
    fn preupdate(&mut self, _window: &mut graphics::window::Window, _shader: &Shader) {}
    fn update(&mut self, _window: &mut graphics::window::Window) {}
    fn postupdate(&mut self, _window: &mut graphics::window::Window, _shader: &Shader) {}
}
