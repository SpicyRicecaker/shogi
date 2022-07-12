use crate::*; // includes import of bevy's prelude

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ClickEvent>()
            .add_event::<ReserveClickEvent>()
            .add_event::<MoveEvent>()
            .add_event::<TakeEvent>()
            // detects a click event, converts it into world coords
            .add_system(mouse_system)
            .add_system(reserve_mouse_system)
            // detects a mouse click, tries to map mouse click to specific square
            .add_system(square_system)
            .add_system(reserve_square_system)
            .add_system_to_stage(CoreStage::PostUpdate, move_system)
            .add_system_to_stage(CoreStage::Last, cleanup_move_system);
    }
}

// it's hard to do a selected piece without a global resource, because sometimes we might not have a selected piece.
// regardless, I think that we should try out functional programming in full.

pub struct ClickEvent {
    pub position: Position,
}

#[derive(Debug)]
pub struct ReserveClickEvent {
    pub piece_type: PieceType,
    pub owner: Player,
}

pub struct MoveEvent {
    // player that moved the piece
    player: Player,
    // position piece was moved to
    position: Position,
}

pub struct TakeEvent {
    pub taker: Player,
    pub piece_type: PieceType,
}

// solution copied from https://bevy-cheatbook.github.io/cookbook/cursor2world.html?highlight=coordinate#convert-cursor-to-world-coordinates
fn window_to_world(
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

fn mouse_system(
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

// queue move -> take piece -> actually move piece

// system that moves the selected piece to a square
fn move_system(
    mut commands: Commands,
    mut ev_click: EventReader<ClickEvent>,
    mut ev_move: EventWriter<MoveEvent>,
    mut ev_take: EventWriter<TakeEvent>,
    mut turn: ResMut<Turn>,
    mut selected_piece: Query<(&mut Position, &mut Transform, &Player), With<SelectedPiece>>,
    mut set: ParamSet<(
        // square query
        Query<(Entity, &Position), (With<Available>, Without<SelectedPiece>)>,
        // pieces query
        Query<(Entity, &Position, &PieceType), (With<Piece>, Without<SelectedPiece>)>,
    )>,
) {
    for e in ev_click.iter() {
        if set
            .p0()
            .iter()
            .any(|(_, p)| p.y == e.position.y && p.x == e.position.x)
            && selected_piece.get_single().is_ok()
        {
            // decide to take the piece
            {
                if let Some((entity, _, piece_type)) =
                    set.p1().iter_mut().find(|(_, &p, _)| p == e.position)
                {
                    // copy over *some* properties, including
                    // - who the owner is
                    // - the piece
                    // add_to_reserve(&mut commands, turn.player, piece_type);
                    ev_take.send(TakeEvent {
                        taker: turn.player,
                        piece_type: *piece_type,
                    });

                    // then despawn the entity
                    commands.entity(entity).despawn_recursive();

                    // add it to the reserve
                    // commands.entity(entity).remove::<Position>();
                    // commands.entity(entity).remove::<Piece>();
                    //
                    // commands.entity(entity).insert(Reserve);
                    // *owner = turn.player;

                    // also update the sprite of the piece
                    // transform.translation = translate_transform(
                    //     // position.x as f32,
                    //     // position.y as f32,
                    //     0.0,
                    //     0.0,
                    //     &owner,
                    // );
                }
            }

            // move the piece
            {
                let (mut position, mut transform, owner) = selected_piece.single_mut();

                // actually move the piece
                position.x = e.position.x;
                position.y = e.position.y;

                transform.translation =
                    translate_transform(position.x as f32, position.y as f32, owner);

                ev_move.send(MoveEvent {
                    player: turn.player,
                    position: e.position,
                });
                turn.player = turn.player.swap();
            }
        }
    }
}

fn cleanup_move_system(
    mut ev_move: EventReader<MoveEvent>,
    mut commands: Commands,
    mut available_squares: Query<(Entity, &mut Sprite), With<Available>>,
    mut selected_piece: Query<(Entity, &mut Sprite), (With<SelectedPiece>, Without<Available>)>,
    colors: Res<Colors>,
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

fn square_system(
    mut commands: Commands,
    mut ev_click: EventReader<ClickEvent>,
    mut piece_query: Query<(Entity, &mut Sprite, &Position, &Player), With<Piece>>,
    mut selected_piece: ParamSet<(
        Query<(Entity, &Position), (With<SelectedPiece>, Without<Available>)>,
        Query<(Entity, &Reserve), (With<SelectedPiece>, Without<Available>)>,
    )>,
    colors: Res<Colors>,
    mut ev_selected_piece: EventWriter<SelectedPieceEvent>,
    turn: Res<Turn>,
) {
    for e in ev_click.iter() {
        for (entity, mut sprite, position, owner) in piece_query.iter_mut() {
            if position.x == e.position.x && position.y == e.position.y {
                // Prevent other player from clicking.
                // Turn off for debugging
                // dbg!("turn player", &turn);
                if *owner != turn.player {
                    break;
                }
                // remove other query

                // set the entity's colors and whatnot,
                // but only if it isn't the same thing
                if let Ok((old_selected_piece, old_selected_piece_position)) =
                    selected_piece.p0().get_single()
                {
                    println!("old was a square");
                    let mut entity_command = commands.entity(old_selected_piece);
                    entity_command.remove::<SelectedPiece>();
                    entity_command.insert(NegativeSelectedPiece);

                    if *position != *old_selected_piece_position {
                        commands.entity(entity).insert(SelectedPiece);
                        ev_selected_piece.send(SelectedPieceEvent::Change);
                        sprite.color = colors.blue;
                        // dbg!("cool");
                    } else {
                        // dbg!("nothing");
                        ev_selected_piece.send(SelectedPieceEvent::None);
                    }
                } else if let Ok((old_sel_pc_e, reserve)) = selected_piece.p1().get_single() {
                    println!("old was a square");
                    commands.entity(old_sel_pc_e).remove::<SelectedPiece>();
                    commands.entity(old_sel_pc_e).insert(NegativeSelectedPiece);

                    commands.entity(entity).insert(SelectedPiece);
                    ev_selected_piece.send(SelectedPieceEvent::Change);
                    sprite.color = colors.blue;
                } else {
                    commands.entity(entity).insert(SelectedPiece);
                    ev_selected_piece.send(SelectedPieceEvent::Change);
                    sprite.color = colors.blue;
                };

                break;
            }
        }
        // run system to check if an available square has been clicked
    }
}

fn reserve_mouse_system(
    square_query: Query<(&Transform, &Player, &PieceType), With<Square>>,
    mut ev_click: EventWriter<ReserveClickEvent>,
    buttons: Res<Input<MouseButton>>,
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
            if let Some((transform, &owner, &piece_type)) =
                square_query.iter().find(|(transform, _, _)| {
                    let size = transform.scale.x / 2.;

                    let square_x = transform.translation.x;
                    let square_y = transform.translation.y;

                    let x = position.x;
                    let y = position.y;

                    // dbg!(transform, square_position);
                    x >= square_x - size
                        && x <= square_x + size
                        && y >= square_y - size
                        && y <= square_y + size
                })
            {
                // Send a taker 2 event
                ev_click.send(ReserveClickEvent { piece_type, owner });
            }

            // DEBUG
            // println!("DBG: Square clicked at {:#?}", position);
        }
    }
}

fn reserve_square_system(
    mut commands: Commands,
    mut ev_click: EventReader<ReserveClickEvent>,
    mut piece_query: Query<
        (Entity, &mut Sprite, &PieceType, &Player, &Reserve),
        Without<SelectedPiece>,
    >,
    mut sel_pc_q: Query<(Entity, &PieceType, &mut Sprite), With<SelectedPiece>>,
    pos_q: Query<&Position>,
    mut ev_selected_piece: EventWriter<SelectedPieceEvent>,
    colors: Res<Colors>,
    turn: Res<Turn>,
) {
    for e in ev_click.iter() {
        if let Some((entity, mut sprite, piece_type, owner, reserve)) = piece_query
            .iter_mut()
            .filter(|(_, _, _, &o, _)| o == turn.player && o == e.owner)
            .find(|(entity, sprite, &piece_type, owner, _)| piece_type == e.piece_type)
        {
            // println!("cool shit");
            if let Ok((sel_pc_e, piece_type, mut sprite)) = sel_pc_q.get_single_mut() {
                if reserve.quantity > 0 {
                    if pos_q.get(entity).is_ok() {
                        println!("previous selected has a position on the board");
                        let mut entity_command = commands.entity(sel_pc_e);
                        entity_command.remove::<SelectedPiece>();
                        entity_command.insert(NegativeSelectedPiece);

                        ev_selected_piece.send(SelectedPieceEvent::Change);
                        sprite.color = colors.blue;
                    } else {
                        println!("previous selected was cool");
                        commands.entity(sel_pc_e).remove::<SelectedPiece>();
                        commands.entity(sel_pc_e).insert(NegativeSelectedPiece);

                        commands.entity(entity).insert(SelectedPiece);
                        ev_selected_piece.send(SelectedPieceEvent::Change);
                        sprite.color = colors.blue;
                    }
                }
            } else {
                println!("previous selected does not exist");
                commands.entity(entity).insert(SelectedPiece);
                ev_selected_piece.send(SelectedPieceEvent::Change);
                sprite.color = colors.blue;
            }
            // // set the entity's colors and whatnot,
            // // but only if it isn't the same thing
        }
        // run system to check if an available square has been clicked
    }
}
