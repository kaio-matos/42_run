pub use crate::engine::prelude::*;

pub trait System {
    fn get_schedule(&self) -> Schedule {
        Schedule::Loop
    }

    fn run(&mut self, world: &mut World, resources: &mut ResourcesManager);
}
