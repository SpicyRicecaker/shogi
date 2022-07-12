use bevy::prelude::*;

use shogi_rs::{
    debug::DebugPlugin, mouse::MousePlugin, regular::RegularPlugin, reserve::ReservePlugin, *,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::hex("282828").unwrap()))
        .add_startup_system(|mut commands: Commands| {
            commands.spawn_bundle(OrthographicCameraBundle::new_2d());
        })
        .add_plugin(DebugPlugin)
        // turns mouse clicks into world coordinates, then emits an event
        .add_plugin(MousePlugin)
        // handles squares on the 9x9 grid
        .add_plugin(RegularPlugin)
        // handles "taken" pieces and their squares
        .add_plugin(ReservePlugin)
        .insert_resource(Turn {
            player: Player::Challenging,
        })
        .insert_resource(Colors::default())
        .run();
}
