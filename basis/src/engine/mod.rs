use crate::prelude::{
    entity::Entity,
    graphics::{glw, window::Window},
    *,
};
mod behaviours;
mod components;
pub mod entity;
pub mod prelude;

#[derive(Debug, Default)]
pub struct EntitiesPending {
    to_add: Vec<Entity>,
    to_remove: Vec<Entity>,
    is_dirty: bool,
}
impl EntitiesPending {
    pub fn spawn(&mut self, entity: Box<dyn EntityLifetime>) {
        self.to_add.push(Entity::new(entity));
        self.is_dirty = true;
    }
    pub fn despawn(&mut self, entity: Box<dyn EntityLifetime>) {
        self.to_remove.push(Entity::new(entity));
        self.is_dirty = true;
    }
    fn mark_as_clean(&mut self) {
        self.is_dirty = false;
    }
}

#[derive(Debug, Default)]
pub struct EntitiesAlive {
    entities: Vec<Entity>,
}
impl EntitiesAlive {
    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.entities.iter()
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = &mut Entity> {
        self.entities.iter_mut()
    }

    fn update(&mut self, entities_pending: &mut EntitiesPending) {
        for entity in entities_pending.to_add.drain(..) {
            self.entities.push(entity);
        }
        self.entities
            .retain(|entity| !entities_pending.to_remove.contains(entity));
        entities_pending.to_remove.clear();
        entities_pending.mark_as_clean();
    }
}

#[derive(Debug, Default)]
pub struct Game {
    width: u32,
    height: u32,
    title: &'static str,
    entities_pending: EntitiesPending,
    entities_alive: EntitiesAlive,
}

impl Game {
    pub fn new(width: u32, height: u32, title: &'static str) -> Self {
        Self {
            width,
            height,
            title,
            entities_pending: EntitiesPending::default(),
            entities_alive: EntitiesAlive::default(),
        }
    }

    pub fn run(
        &mut self,
        callback_setup: impl Fn(&mut EntitiesAlive, &mut EntitiesPending),
        callback_before_entities_update: impl Fn(&mut EntitiesAlive, &mut EntitiesPending),
        callback_after_entities_update: impl Fn(&mut EntitiesAlive, &mut EntitiesPending),
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut window = Window::new(self.width, self.height, self.title);
        window.init_gl();

        glw::enable(gl::DEPTH_TEST);

        let shader = glw::Shader::default();
        shader
            .link_multiple(vec![
                glw::ShaderType::Vertex("basis/src/assets/shaders/vertex_perspective_shader.glsl"),
                glw::ShaderType::Fragment(
                    "basis/src/assets/shaders/fragment_perspective_shader.glsl",
                ),
            ])
            .expect("Shader to be found, compiled and linked");

        let mut is_wireframe = false;

        callback_setup(&mut self.entities_alive, &mut self.entities_pending);
        self.entities_alive.update(&mut self.entities_pending);

        //
        // setup entities
        //
        for entity in self.entities_alive.iter_mut() {
            entity.data.setup(&mut self.entities_pending);
        }
        self.entities_alive.update(&mut self.entities_pending);

        while !window.should_close() {
            window.compute_deltatime();

            glw::clear_color(0.2, 0.3, 0.3, 1.0);
            glw::clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            //
            // debug helper
            //
            if window.on_key_press(graphics::glfw::Key::E, graphics::glfw::Modifiers::empty()) {
                is_wireframe = !is_wireframe;
                if is_wireframe {
                    glw::polygon_mode(gl::FRONT_AND_BACK, gl::LINE);
                } else {
                    glw::polygon_mode(gl::FRONT_AND_BACK, gl::FILL);
                }
            }

            callback_before_entities_update(&mut self.entities_alive, &mut self.entities_pending);
            self.entities_alive.update(&mut self.entities_pending);
            //
            // run business logic
            //
            for entity in self.entities_alive.iter_mut() {
                entity.data.preupdate(&mut window, &shader);
                entity.data.update(&mut window);
                entity.data.postupdate(&mut window, &shader);
            }
            self.entities_alive.update(&mut self.entities_pending);
            callback_after_entities_update(&mut self.entities_alive, &mut self.entities_pending);
            self.entities_alive.update(&mut self.entities_pending);

            //
            // update screen
            //
            for entity in self.entities_alive.iter_mut() {
                match entity.data.get_object() {
                    None => {}
                    Some(object) => {
                        object.rotation = object.rotation.normalize();
                        draw(&shader, object, window.get_size());
                    }
                }
            }

            window.update(&mut |_event| {});
        }

        Ok(())
    }
}

pub fn draw(shader: &glw::Shader, obj: &Object, window_size: (i32, i32)) {
    shader.bind();
    let (window_width, window_height) = window_size;

    let mut model_mat = Mat4::identity();
    let projection_mat = Mat4::symmetric_perspective(
        45.0_f32.to_radians(),
        window_width as f32 / window_height as f32,
        0.1,
        1000.,
    );

    model_mat.scale(obj.scale);
    model_mat.rotate_around_center(obj.center().negate(), obj.rotation);
    model_mat.translate(obj.position);

    shader
        .get_uniform_location("projection")
        .uniform_matrix4fv(&projection_mat);
    shader
        .get_uniform_location("model")
        .uniform_matrix4fv(&model_mat);

    shader.get_uniform_location("object_texture").uniform1i(0);

    obj.draw();

    shader.unbind();
}

// fn load_model(
//     filepath: &str,
//     entities: &mut Vec<Box<dyn EntityLifetime>>,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     let mut obj = structs::Object::new(wavefront::obj::load(filepath)?);
//     obj.set_texture(helpers::load_custom_texture(
//         "game/src/resources/raw_texture.txt",
//     )?);
//
//     let objs_transformation = [(
//         Vec3::new(0.0, 0.0, 0.0),            // Position
//         Vec3::new(1.0, 1.0, 1.0),            // Scale
//         Quaternion::new(0.0, 0.0, 0.0, 1.0), // Rotation (identity quaternion)
//         Vec3::new(1.0, 0.0, 0.0),            // Color (Red)
//     )];
//
//     for (position, scale, rotation, rgb) in &objs_transformation {
//         let mut new = obj.clone();
//         new.scale(*scale);
//         new.color(*rgb);
//         new.translate(*position);
//         new.rotation = *rotation;
//         entities.push(Box::new(Cube { object: new }));
//     }
//
//     Ok(())
// }
