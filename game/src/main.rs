use basis::{
    graphics::{glw, wavefront},
    prelude::*,
};
mod behaviours;
mod components;

use std::process::ExitCode;

use crate::components::PlayerCamera;

#[derive(Default)]
struct SystemDebugWireframe {
    is_wireframe: bool,
}
impl System for SystemDebugWireframe {
    fn run(&mut self, _world: &mut World, resources: &mut ResourcesManager) {
        //
        // debug helper
        //
        let event_handler = resources.get::<EventHandler>();
        if event_handler.on_key_press(graphics::glfw::Key::E, graphics::glfw::Modifiers::empty()) {
            self.is_wireframe = !self.is_wireframe;
            if self.is_wireframe {
                glw::polygon_mode(gl::FRONT_AND_BACK, gl::LINE);
            } else {
                glw::polygon_mode(gl::FRONT_AND_BACK, gl::FILL);
            }
        }
    }
}

#[derive(Default)]
struct SystemDebugCamera {}
impl System for SystemDebugCamera {
    fn run(&mut self, world: &mut World, resources: &mut ResourcesManager) {
        let event_handler = resources.get::<EventHandler>();
        let deltatime = resources.get::<Deltatime>();
        let shader = resources.get::<RenderShader>();

        for entity in world.entity_manager.active_entities() {
            world.with_components_mut_1::<DebugCamera, _>(entity, |debug_camera| {
                if let Some(debug_camera) = debug_camera {
                    if event_handler
                        .on_key_hold(graphics::glfw::Key::W, graphics::glfw::Modifiers::empty())
                    {
                        debug_camera.move_up(**deltatime)
                    }
                    if event_handler
                        .on_key_hold(graphics::glfw::Key::S, graphics::glfw::Modifiers::empty())
                    {
                        debug_camera.move_down(**deltatime)
                    }
                    if event_handler
                        .on_key_hold(graphics::glfw::Key::A, graphics::glfw::Modifiers::empty())
                    {
                        debug_camera.move_left(**deltatime)
                    }
                    if event_handler
                        .on_key_hold(graphics::glfw::Key::D, graphics::glfw::Modifiers::empty())
                    {
                        debug_camera.move_right(**deltatime)
                    }
                    if event_handler
                        .on_key_hold(graphics::glfw::Key::W, graphics::glfw::Modifiers::Control)
                    {
                        debug_camera.move_forward(**deltatime)
                    }
                    if event_handler
                        .on_key_hold(graphics::glfw::Key::S, graphics::glfw::Modifiers::Control)
                    {
                        debug_camera.move_backward(**deltatime)
                    }

                    shader.bind();
                    shader
                        .get_uniform_location("view")
                        .uniform_matrix4fv(&debug_camera.get_view_matrix());
                    shader.unbind();
                }
            });
        }
    }
}

#[derive(Default)]
struct SystemPlayerCamera {}
impl System for SystemPlayerCamera {
    fn run(&mut self, world: &mut World, resources: &mut ResourcesManager) {
        let event_handler = resources.get::<EventHandler>();
        let shader = resources.get::<RenderShader>();
        let deltatime = resources.get::<Deltatime>();

        for entity in world.entity_manager.active_entities() {
            world.with_components_mut_1::<PlayerCamera, _>(entity, |player_camera| {
                if let Some(player_camera) = player_camera {
                    if event_handler
                        .on_key_hold(graphics::glfw::Key::W, graphics::glfw::Modifiers::empty())
                    {
                        player_camera.move_up(**deltatime)
                    }
                    if event_handler
                        .on_key_hold(graphics::glfw::Key::S, graphics::glfw::Modifiers::empty())
                    {
                        player_camera.move_down(**deltatime)
                    }
                    if event_handler
                        .on_key_hold(graphics::glfw::Key::A, graphics::glfw::Modifiers::empty())
                    {
                        player_camera.move_left(**deltatime)
                    }
                    if event_handler
                        .on_key_hold(graphics::glfw::Key::D, graphics::glfw::Modifiers::empty())
                    {
                        player_camera.move_right(**deltatime)
                    }
                    if event_handler
                        .on_key_hold(graphics::glfw::Key::W, graphics::glfw::Modifiers::Control)
                    {
                        player_camera.move_forward(**deltatime)
                    }
                    if event_handler
                        .on_key_hold(graphics::glfw::Key::S, graphics::glfw::Modifiers::Control)
                    {
                        player_camera.move_backward(**deltatime)
                    }

                    shader.bind();
                    shader
                        .get_uniform_location("view")
                        .uniform_matrix4fv(&player_camera.get_view_matrix());
                    shader.unbind();
                }
            });
        }
    }
}

#[derive(Default)]
struct SystemCubeSetup {}
impl System for SystemCubeSetup {
    fn get_schedule(&self) -> Schedule {
        Schedule::Setup
    }

    fn run(&mut self, world: &mut World, _resources: &mut ResourcesManager) {
        world.add_component(DebugCamera::new(
            Vec3::new(0.0, 0.0, 10.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            30.,
        ));

        //
        // Create Cube
        //
        let entity = world.spawn();
        let mut transform = Transform::default();
        let mut obj = Object::new(
            wavefront::obj::load("basis/src/assets/models/cube.obj")
                .expect("Cube model is expected to exist."),
        );

        transform.scale(Vec3::splat(1.));
        transform.translate(Vec3::splat(0.0));
        transform.rotation = Quaternion::new(0.0, 0.0, 0.0, 1.0);
        obj.color(Vec3::new(1.0, 0.0, 0.0));

        world.add_entity_component(entity, Cube);
        world.add_entity_component(entity, obj);
        world.add_entity_component(entity, transform);
        //
        // /Create Cube
        //
    }
}

#[derive(Default)]
struct SystemCubeMovement {}
impl System for SystemCubeMovement {
    fn run(&mut self, world: &mut World, resources: &mut ResourcesManager) {
        let event_handler = resources.get::<EventHandler>();
        let deltatime = resources.get::<Deltatime>();

        for entity in world.entity_manager.active_entities() {
            world.with_components_mut_2::<Cube, Transform, _>(entity, |cube, transform| {
                if let (Some(_cube), Some(transform)) = (cube, transform) {
                    if event_handler
                        .on_key_hold(graphics::glfw::Key::Up, graphics::glfw::Modifiers::empty())
                    {
                        transform.move_up(**deltatime)
                    }
                    if event_handler.on_key_hold(
                        graphics::glfw::Key::Down,
                        graphics::glfw::Modifiers::empty(),
                    ) {
                        transform.move_down(**deltatime)
                    }
                    if event_handler.on_key_hold(
                        graphics::glfw::Key::Left,
                        graphics::glfw::Modifiers::empty(),
                    ) {
                        transform.move_left(**deltatime)
                    }
                    if event_handler.on_key_hold(
                        graphics::glfw::Key::Right,
                        graphics::glfw::Modifiers::empty(),
                    ) {
                        transform.move_right(**deltatime)
                    }
                    if event_handler.on_key_hold(
                        graphics::glfw::Key::PageUp,
                        graphics::glfw::Modifiers::empty(),
                    ) {
                        transform.move_forward(**deltatime)
                    }
                    if event_handler.on_key_hold(
                        graphics::glfw::Key::PageDown,
                        graphics::glfw::Modifiers::empty(),
                    ) {
                        transform.move_backward(**deltatime)
                    }

                    if event_handler
                        .on_key_hold(graphics::glfw::Key::Up, graphics::glfw::Modifiers::Control)
                    {
                        transform.rotateq(
                            **deltatime,
                            Quaternion::from_euler_angles(
                                Vec3::new(0.1, 0.0, 0.0),
                                5_f32.to_radians(),
                            ),
                        );
                    }

                    if event_handler.on_key_hold(
                        graphics::glfw::Key::Down,
                        graphics::glfw::Modifiers::Control,
                    ) {
                        transform.rotateq(
                            **deltatime,
                            Quaternion::from_euler_angles(
                                Vec3::new(-0.1, 0.0, 0.0),
                                5_f32.to_radians(),
                            ),
                        );
                    }
                    if event_handler.on_key_hold(
                        graphics::glfw::Key::Left,
                        graphics::glfw::Modifiers::Control,
                    ) {
                        transform.rotateq(
                            **deltatime,
                            Quaternion::from_euler_angles(
                                Vec3::new(0.0, -0.1, 0.0),
                                5_f32.to_radians(),
                            ),
                        );
                    }
                    if event_handler.on_key_hold(
                        graphics::glfw::Key::Right,
                        graphics::glfw::Modifiers::Control,
                    ) {
                        transform.rotateq(
                            **deltatime,
                            Quaternion::from_euler_angles(
                                Vec3::new(0.0, 0.1, 0.0),
                                5_f32.to_radians(),
                            ),
                        );
                    }
                }
            });
        }
    }
}

fn main() -> ExitCode {
    let mut engine = Engine::new(800, 800, "42run");
    let mut systems: Vec<Box<dyn System>> = vec![
        Box::new(SystemCubeSetup::default()),
        Box::new(SystemDebugWireframe::default()),
        Box::new(SystemDebugCamera::default()),
        Box::new(SystemCubeMovement::default()),
        // Box::new(SystemPlayerCamera::default()),
    ];

    let result = engine.run(&mut systems);

    if let Err(error) = result {
        eprintln!("Error: {}", error);
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
