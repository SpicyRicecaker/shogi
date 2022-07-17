use crate::*;

use crate::regular::DoneEvent;
use bevy::text::Text2dBounds;
use core::f32::consts::PI; // includes import of bevy's prelude

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

// it's hard to do a selected piece without a global resource, becaus sometimes we might not have a selected piece.
// regardless, I think that we should try out functional programming in full.

pub struct ClickEvent {
    pub position: Position,
}

#[derive(Debug)]
pub struct ReserveClickEvent {
    pub piece_type: PieceType,
    pub owner: Player,
}

pub struct MoveEvent;

pub struct TakeEvent {
    pub taker: Player,
    pub piece_type: PieceType,
}

// solution copied from https://bevy-cheatbook.github.io/cookbook/cursor2world.html?highlight=coordinate#convert-cursor-to-world-coordinates
pub fn window_to_world(
    screen_position: Vec2,
    window_size: Vec2,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Vec2 {
    // get the size of the window

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
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);
        let position = window_to_world(position, window_size, camera, camera_transform);
        if buttons.just_pressed(MouseButton::Left) {
            // try to match it to a square
            // from x to x + scale, cursor position
            // from y to y + scale, cursor position
            for (transform, square_position) in square_query.iter() {
                let size = transform.scale.x / 2.;

                let square_x = transform.translation.x;
                let square_y = transform.translation.y;

                let x = position.x;
                let y = position.y;

                if x >= square_x - size
                    && x <= square_x + size
                    && y >= square_y - size
                    && y <= square_y + size
                {
                    ev_click.send(ClickEvent {
                        position: *square_position,
                    });
                    break;
                }
            }
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
    mut selected_piece: Query<
        (
            Entity,
            &mut Transform,
            &Player,
            &mut Sprite,
            &PieceType,
            &Children,
            &mut Rank,
        ),
        With<SelectedPiece>,
    >,
    mut pos_q: Query<&mut Position, With<SelectedPiece>>,
    mut res_q: Query<&mut Reserve, With<SelectedPiece>>,
    mut q_txt: ParamSet<(Query<&mut Text, With<Counter>>, Query<&mut Text>)>,
    mut set: ParamSet<(
        // square query
        Query<(Entity, &Position), (With<Available>, Without<SelectedPiece>)>,
        // pieces query
        Query<(Entity, &Position, &PieceType), (With<Piece>, Without<SelectedPiece>)>,
    )>,
    colors: Res<Colors>,
    asset_server: Res<AssetServer>,
    board: Res<Board>,
) {
    for e in ev_click.iter() {
        if set
            .p0()
            .iter()
            .any(|(_, p)| p.y == e.position.y && p.x == e.position.x)
            // might be a bit of a redundant check, since there wouldn't be any
            // available squares if there was no selected piece in the first
            // place
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
                let x = e.position.x;
                let y = e.position.y;

                let (entity, mut transform, player, mut sprite, piece_type, children, mut rank) =
                    selected_piece.single_mut();

                if let Ok(mut position) = pos_q.get_mut(entity) {
                    // if the position we're moving to is within the enemy's backrank, promote if we haven't already
                    if match *player {
                        Player::Residing => y <= 2,
                        Player::Challenging => y >= 6,
                    } {
                        *rank = Rank::Promoted;

                        // update text
                        for child in children.iter() {
                            if let Ok(mut text) = q_txt.p1().get_mut(*child) {
                                let mut sec = text
                                    .sections
                                    .get_mut(0)
                                    .expect("error getting text field of reserve counter entity");
                                sec.value =
                                    get_kanji(*piece_type, Rank::Promoted, *player).to_string();
                                sec.style.color = colors.red;
                            }
                        }
                        // update color
                    }

                    // actually move the piece
                    position.x = x;
                    position.y = y;

                    transform.translation = translate_transform(
                        position.x as f32,
                        position.y as f32,
                        board.x_offset,
                        board.y_offset,
                        player,
                    );
                    sprite.color = colors.dark;
                } else {
                    let mut reserve = res_q.get_mut(entity).unwrap();

                    let font = asset_server.load("yujiboku.ttf");
                    let text_style = TextStyle {
                        font,
                        font_size: 50.0,
                        color: colors.light,
                    };
                    let text_alignment_center = TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    };
                    let box_size = Size::new(48.0, 48.0);

                    // decrement reserve
                    reserve.quantity -= 1;
                    if reserve.quantity == 0 {
                        sprite.color = {
                            let mut dark = colors.dark;
                            dark.set_a(0.2);
                            dark
                        };
                    } else {
                        sprite.color = colors.dark;
                    }
                    for &child in children.iter() {
                        if let Ok(mut text) = q_txt.p0().get_mut(child) {
                            text.sections
                                .get_mut(0)
                                .expect("error getting text field of reserve counter entity")
                                .value = format!("{}", reserve.quantity);
                        }
                    }

                    commands
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                color: colors.dark,
                                custom_size: Some(Vec2::new(
                                    SQUARE_LENGTH - SQUARE_BORDER,
                                    SQUARE_LENGTH - SQUARE_BORDER,
                                )),
                                ..Default::default()
                            },
                            transform: Transform {
                                translation: Vec3::new(
                                    x as f32 * SQUARE_LENGTH + board.x_offset,
                                    y as f32 * SQUARE_LENGTH + board.y_offset,
                                    1.0,
                                ),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(*piece_type)
                        .insert(*player)
                        .insert(Rank::Regular)
                        .insert(Position { x, y })
                        .insert(Piece)
                        .with_children(|parent| {
                            parent.spawn_bundle(Text2dBundle {
                                text: Text::with_section(
                                    get_kanji(*piece_type, Rank::Regular, *player),
                                    text_style.clone(),
                                    text_alignment_center,
                                ),
                                text_2d_bounds: Text2dBounds {
                                    // Wrap text in the rectangle
                                    size: box_size,
                                },
                                // We align text to the top-left, so this transform is the top-left corner of our text. The
                                // box is centered at box_position, so it is necessary to move by half of the box size to
                                // keep the text in the box.
                                transform: Transform {
                                    translation: match *player {
                                        Player::Challenging => Vec3::new(0.0, 2.0, 2.0),
                                        Player::Residing => Vec3::new(0.0, -2.0, 2.0),
                                    },
                                    rotation: match *player {
                                        Player::Challenging => Quat::from_rotation_z(0.),
                                        Player::Residing => Quat::from_rotation_z(PI),
                                    },
                                    ..default() // scale: todo!(),
                                },
                                ..default()
                            });
                        });
                    // generate a new piece at this specifc postion
                }

                commands.entity(entity).remove::<SelectedPiece>();
                ev_move.send(MoveEvent);
                turn.player = turn.player.swap();
            }
        }
    }
}

// Ensures that after a move, the piece that was moved is desselected. We need
// this, because otherwise, on the next players turn they might have their
// opponent's piece selected
fn cleanup_move_system(
    mut ev_move: EventReader<MoveEvent>,
    mut ev_win: EventWriter<DoneEvent>,
    mut commands: Commands,
    mut available_squares: Query<(Entity, &mut Sprite), With<Available>>,
    colors: Res<Colors>,
) {
    ev_move.iter().for_each(|_| {
        // if let Ok((entity, mut sprite)) = selected_piece.get_single_mut() {
        //     commands.entity(entity).remove::<SelectedPiece>();
        //     sprite.color = colors.dark;
        // }
        for (entity, mut sprite) in available_squares.iter_mut() {
            commands.entity(entity).remove::<Available>();
            sprite.color = colors.light;
        }
        ev_win.send(DoneEvent);
    });
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
                if *owner != turn.player {
                    break;
                }
                // remove other query

                // set the entity's colors and whatnot,
                // but only if it isn't the same thing
                if let Ok((old_selected_piece, old_selected_piece_position)) =
                    selected_piece.p0().get_single()
                {
                    let mut entity_command = commands.entity(old_selected_piece);
                    entity_command.remove::<SelectedPiece>();
                    entity_command.insert(NegativeSelectedPiece);

                    if *position != *old_selected_piece_position {
                        commands.entity(entity).insert(SelectedPiece);
                        ev_selected_piece.send(SelectedPieceEvent::Change);
                        sprite.color = colors.blue;
                    } else {
                        ev_selected_piece.send(SelectedPieceEvent::None);
                    }
                } else if let Ok((old_sel_pc_e, _)) = selected_piece.p1().get_single() {
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
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);
        let position = window_to_world(position, window_size, camera, camera_transform);
        if buttons.just_pressed(MouseButton::Left) {
            // try to match it to a square
            // from x to x + scale, cursor position
            // from y to y + scale, cursor position
            if let Some((_, &owner, &piece_type)) = square_query.iter().find(|(transform, _, _)| {
                let size = transform.scale.x / 2.;

                let square_x = transform.translation.x;
                let square_y = transform.translation.y;

                let x = position.x;
                let y = position.y;

                x >= square_x - size
                    && x <= square_x + size
                    && y >= square_y - size
                    && y <= square_y + size
            }) {
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
    // e is the clicked on square
    for e in ev_click.iter() {
        // entity, sprite, piece_type, owner, reserve : every reserve holds these traits
        if let Some((entity, mut sprite, _, _, _)) =
            piece_query.iter_mut().find(|(_, _, &t, &o, &r)| {
                t == e.piece_type && o == turn.player && o == e.owner && r.quantity > 0
            })
        {
            if let Ok((sel_pc_e, _, mut sprite)) = sel_pc_q.get_single_mut() {
                if pos_q.get(entity).is_ok() {
                    let mut entity_command = commands.entity(sel_pc_e);
                    entity_command.remove::<SelectedPiece>();
                    entity_command.insert(NegativeSelectedPiece);

                    ev_selected_piece.send(SelectedPieceEvent::Change);
                    sprite.color = colors.blue;
                } else {
                    commands.entity(sel_pc_e).remove::<SelectedPiece>();
                    commands.entity(sel_pc_e).insert(NegativeSelectedPiece);

                    commands.entity(entity).insert(SelectedPiece);
                    ev_selected_piece.send(SelectedPieceEvent::Change);
                    sprite.color = colors.blue;
                }
            } else {
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
