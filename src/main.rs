use bevy::prelude::*;

use shogi_rs::{
    debug::DebugPlugin, mouse::MousePlugin, regular::RegularPlugin, reserve::ReservePlugin, *,
};

fn main() {
    App::new()
        .insert_resource(Colors::default())
        // default engine stuff begin
        .add_plugins(DefaultPlugins)
        .add_plugin(ReservePlugin)
        .add_plugin(MousePlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(RegularPlugin)
        .add_startup_system(camera)
        .insert_resource(ClearColor(Color::hex("282828").unwrap()))
        .insert_resource(Turn {
            player: Player::Challenging,
        })
        // default engine stuff end
        .run();
}

fn camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
