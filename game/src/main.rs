use basis::prelude::*;
mod behaviours;
mod components;

use std::process::ExitCode;

fn callback_setup(_entities_alive: &mut EntitiesAlive, entities_pending: &mut EntitiesPending) {
    entities_pending.spawn(Box::new(DebugCamera::new(
        Vec3::new(0.0, 0.0, 10.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        30.,
    )));
    entities_pending.spawn(Box::new(Cube::default()));
}

fn callback_before_entities_update(
    _entities_alive: &mut EntitiesAlive,
    _entities_pending: &mut EntitiesPending,
) {
}

fn callback_after_entities_update(
    _entities_alive: &mut EntitiesAlive,
    _entities_pending: &mut EntitiesPending,
) {
}

fn main() -> ExitCode {
    let mut game = Game::new(800, 800, "42run");

    let result = game.run(
        callback_setup,
        callback_before_entities_update,
        callback_after_entities_update,
    );
    if let Err(error) = result {
        eprintln!("Error: {}", error);
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
