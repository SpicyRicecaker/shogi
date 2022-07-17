#![deny(clippy::all)]

use bevy::prelude::*;

use shogi_rs::{
    debug::DebugPlugin, mouse::MousePlugin, regular::RegularPlugin, reserve::ReservePlugin, *,
};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::hex("282828").unwrap()))
        .add_startup_system(|mut commands: Commands| {
            commands.spawn_bundle(OrthographicCameraBundle::new_2d());
        })
        .init_resource::<Board>()
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
        .init_resource::<Colors>();

    // resize canvas thingy
    #[cfg(target_arch = "wasm32")]
    app.add_plugin(shogi_rs::bindgen::Plugin);

    app.run();
}
