use crate::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_debug).add_system(debug_system);
    }
}

fn spawn_debug(mut commands: Commands, colors: Res<Colors>) {
    // commands
    //     .spawn_bundle(SpriteBundle {
    //         sprite: Sprite {
    //             color: colors.green,
    //             // custom_size: Some(Vec2::new(
    //             //     square_length - square_border,
    //             // )),
    //             ..Default::default()
    //         },
    //         transform: Transform {
    //             translation: Vec3::new(
    //                 -225.,
    //                 300.,
    //                 0.0,
    //             ),
    //             scale: SQUARE_SIZE,
    //             ..Default::default()
    //         },
    //         ..Default::default()
    //     });
    //     // .insert(Position { x: i, y: j })
    //     // .insert(Square);
}

fn debug_system(windows: Res<Windows>) {
    // let window = windows.get_primary().unwrap();

    // dbg!(window.height(), window.width());
}
