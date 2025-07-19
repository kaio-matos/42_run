use std::error::Error;

use crate::graphics::{glw, window::Window};
mod behaviours;
mod components;
mod ecs;
pub mod prelude;
mod resources;
mod world;
use crate::prelude::*;

pub struct Engine {
    width: u32,
    height: u32,
    title: &'static str,
    world: World,
}

impl Engine {
    pub fn new(width: u32, height: u32, title: &'static str) -> Self {
        Self {
            world: World::default(),
            width,
            height,
            title,
        }
    }

    pub fn run(&mut self, systems: &mut Vec<Box<dyn System>>) -> Result<(), Box<dyn Error>> {
        let mut window = Window::new(self.width, self.height, self.title);
        let mut resources = ResourcesManager::default();

        window.init_gl();
        glw::enable(gl::DEPTH_TEST);

        resources.add::<Deltatime>(Deltatime(window.deltatime));
        resources.add::<EventHandler>(EventHandler::new());
        resources.add::<RenderShader>(RenderShader::default());

        // TODO: Improve this:
        systems
            .iter_mut()
            .filter(|system| Schedule::Setup == system.get_schedule())
            .for_each(|system| system.run(&mut self.world, &mut resources));

        while !window.should_close() {
            window.compute_deltatime();
            let deltatime = resources.get_mut::<Deltatime>();
            *deltatime = Deltatime(window.deltatime);

            // TODO: Improve this:
            systems
                .iter_mut()
                .filter(|system| Schedule::Loop == system.get_schedule())
                .for_each(|system| system.run(&mut self.world, &mut resources));

            render(&mut self.world, &mut resources, &window);

            let events = window.update();
            let event_handler = resources.get_mut::<EventHandler>();
            event_handler.update(events);
            event_handler.process();
        }

        Ok(())
    }
}

fn render(world: &mut World, resources: &mut ResourcesManager, window: &Window) {
    glw::clear_color(0.2, 0.3, 0.3, 1.0);
    glw::clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

    let shader = resources.get::<RenderShader>();
    let (window_width, window_height) = window.get_size();

    shader.bind();
    for entity in world.entity_manager.active_entities() {
        world.with_components_mut_2::<Object, Transform, _>(entity, |object, transform| {
            if let (Some(obj), Some(transform)) = (object, transform) {
                transform.rotation = transform.rotation.normalize();

                let mut model_mat = Mat4::identity();
                let projection_mat = Mat4::symmetric_perspective(
                    45.0_f32.to_radians(),
                    window_width as f32 / window_height as f32,
                    0.1,
                    1000.,
                );

                model_mat.scale(transform.scale);
                model_mat.rotate_around_center(
                    transform.center(obj.center()).negate(),
                    transform.rotation,
                );
                model_mat.translate(transform.position);

                shader
                    .get_uniform_location("projection")
                    .uniform_matrix4fv(&projection_mat);
                shader
                    .get_uniform_location("model")
                    .uniform_matrix4fv(&model_mat);

                shader.get_uniform_location("object_texture").uniform1i(0);

                obj.draw();
            }
        });
    }
    shader.unbind();
}
