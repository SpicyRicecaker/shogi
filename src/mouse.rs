use crate::*;

// it's hard to do a selected piece without a global resource, because sometimes we might not have a selected piece.
// regardless, I think that we should try out functional programming in full.

pub struct ClickEvent {
    pub position: Position,
}

pub struct MoveEvent;

// solution copied from https://bevy-cheatbook.github.io/cookbook/cursor2world.html?highlight=coordinate#convert-cursor-to-world-coordinates
pub fn window_to_world(
    screen_position: Vec2,
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Vec2 {
    // get the size of the window
    let window_size = Vec2::new(window.width() as f32, window.height() as f32);

    // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
    let ndc = (screen_position / window_size) * 2.0 - Vec2::ONE;

    // matrix for undoing the projection and camera transform
    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();

    // use it to convert ndc to world-space coordinates
    let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

    // reduce it to a 2D value
    let world_pos: Vec2 = world_pos.truncate();

    world_pos
}

pub fn mouse_system(
    square_query: Query<(&Transform, &Position), With<Square>>,
    // selected_piece: Res<SelectedPiece>,
    mut ev_click: EventWriter<ClickEvent>,
    buttons: Res<Input<MouseButton>>,
    // we need the camera vector to normalize things
    camera: Query<(&Camera, &GlobalTransform)>,
    windows: Res<Windows>,
) {
    // there can be a selected piece, but there's no such thing as a selected square
    let window = windows.get_primary().unwrap();

    if let Some(position) = window.cursor_position() {
        let (camera, camera_transform) = camera.single();
        let position = window_to_world(position, window, camera, camera_transform);
        if buttons.just_pressed(MouseButton::Left) {
            // println!("mouse just got clicked at {:#?}", position);
            // try to match it to a square
            // from x to x + scale, cursor position
            // from y to y + scale, cursor position
            for (transform, square_position) in square_query.iter() {
                let size = transform.scale.x / 2.;

                let square_x = transform.translation.x;
                let square_y = transform.translation.y;

                let x = position.x;
                let y = position.y;

                // dbg!(transform, square_position);
                if x >= square_x - size
                    && x <= square_x + size
                    && y >= square_y - size
                    && y <= square_y + size
                {
                    // println!("square clicked!");
                    ev_click.send(ClickEvent {
                        position: *square_position,
                    });
                    // DEBUG
                    // println!("DBG: Square clicked at {:#?}", *square_position);
                    break;
                }
                // println!("{:#?}", transform.translation);
            }

            // DEBUG
            // println!("DBG: Square clicked at {:#?}", position);
        }
    }
}

// system that moves the selected piece to a square
pub fn move_system(
    mut ev_click: EventReader<ClickEvent>,
    mut ev_move: EventWriter<MoveEvent>,
    mut square_query: Query<(Entity, &Position), (With<Available>, Without<SelectedPiece>)>,
    mut selected_piece: Query<(&mut Position, &mut Transform, &Player), With<SelectedPiece>>,
    mut turn: ResMut<Turn>,
) {
    for e in ev_click.iter() {
        for (entity, position) in square_query.iter_mut() {
            if position.x == e.position.x && position.y == e.position.y {
                if let Ok((mut selected_piece_position, mut transform, player)) =
                    selected_piece.get_single_mut()
                {
                    selected_piece_position.x = e.position.x;
                    selected_piece_position.y = e.position.y;

                    transform.translation = translate_transform(
                        selected_piece_position.x as f32,
                        selected_piece_position.y as f32,
                        player,
                    );

                    ev_move.send(MoveEvent);
                    
                    turn.player = turn.player.swap();
                    // need to make sure to remove all available and selected pieces here as well
                    break;
                }
            }
        }
    }
}

pub fn cleanup_move_system(
    mut ev_move: EventReader<MoveEvent>,
    mut commands: Commands,
    mut available_squares: Query<(Entity, &mut Sprite), With<Available>>,
    mut selected_piece: Query<(Entity, &mut Sprite), (With<SelectedPiece>, Without<Available>)>,
    colors: Res<Colors>
) {
    for e in ev_move.iter() {
        if let Ok((entity, mut sprite)) = selected_piece.get_single_mut() {
            commands.entity(entity).remove::<SelectedPiece>();
            sprite.color = colors.dark;
        }
        for (entity, mut sprite) in available_squares.iter_mut() {
            commands.entity(entity).remove::<Available>();
            sprite.color = colors.light;
        }
    }
}