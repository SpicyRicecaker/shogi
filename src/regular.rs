use bevy::text::Text2dBounds;

use crate::*;
use core::f32::consts::PI;

pub struct RegularPlugin;

impl Plugin for RegularPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SelectedPieceEvent>()
            .add_startup_system(spawn_squares)
            .add_startup_system(spawn_pieces)
            .add_system(detect_removals)
            .add_system_to_stage(CoreStage::PostUpdate, reset_square_system)
            .add_system_to_stage(CoreStage::Last, available_square_system);
    }
}

// separating squares from pieces is genius, because then we can actually give the piece a unique sprite
fn spawn_squares(mut commands: Commands, colors: Res<Colors>) {
    for i in 0..9 {
        for j in 0..9 {
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: colors.light,
                        // custom_size: Some(Vec2::new(
                        //     square_length - square_border,
                        // )),
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(
                            i as f32 * SQUARE_LENGTH + BOARD_X_OFFSET,
                            j as f32 * SQUARE_LENGTH + BOARD_Y_OFFSET,
                            0.0,
                        ),
                        scale: SQUARE_SIZE,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Position { x: i, y: j })
                .insert(Square);
        }
    }
}

fn spawn_pieces(mut commands: Commands, colors: Res<Colors>, asset_server: Res<AssetServer>) {
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
    // each piece has a PieceType, position, rank, and owner
    // the sprite of the piece changes with respect to its name and rank

    let box_size = Size::new(48.0, 48.0);

    let pieces = r##"
    lnsgkgsnl
    .r.....b.
    ppppppppp
    .........
    .........
    .........
    ppppppppp
    .b.....r.
    lnsgkgsnl
    "##;
    // let pieces = r##"
    // .........
    // .r.......
    // .........
    // .........
    // .........
    // .........
    // .........
    // .........
    // .........
    // "##;

    for (y, line) in pieces.trim().lines().rev().enumerate() {
        for (x, char) in line.trim().chars().enumerate() {
            if char == '.' {
                continue;
            }
            let player = if y <= 2 {
                Player::Challenging
            } else {
                Player::Residing
            };

            let piece_type = PieceType::from(char);

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
                            x as f32 * SQUARE_LENGTH + BOARD_X_OFFSET,
                            y as f32 * SQUARE_LENGTH + BOARD_Y_OFFSET,
                            1.0,
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(piece_type)
                .insert(player)
                .insert(Rank::Regular)
                .insert(Position { x, y })
                .insert(Piece)
                .with_children(|parent| {
                    parent.spawn_bundle(Text2dBundle {
                        text: Text::with_section(
                            get_kanji(piece_type, Rank::Regular, player),
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
                            translation: match player {
                                Player::Challenging => Vec3::new(0.0, 2.0, 2.0),
                                Player::Residing => Vec3::new(0.0, -2.0, 2.0),
                            },
                            rotation: match player {
                                Player::Challenging => Quat::from_rotation_z(0.),
                                Player::Residing => Quat::from_rotation_z(PI),
                            },
                            ..default() // scale: todo!(),
                        },
                        ..default()
                    });
                });
        }
    }
}

fn reset_square_system(
    mut commands: Commands,
    mut ev_selected_piece: EventReader<SelectedPieceEvent>,
    mut square_query: Query<(Entity, &mut Sprite), With<Available>>,
    colors: Res<Colors>,
) {
    for _ in ev_selected_piece.iter() {
        // dbg!("ran reset square system");
        for (entity, mut sprite) in square_query.iter_mut() {
            sprite.color = colors.light;
            commands.entity(entity).remove::<Available>();
        }
    }
}

fn available_square_system(
    mut commands: Commands,
    mut ev_selected_piece: EventReader<SelectedPieceEvent>,
    mut square_query: Query<(Entity, &mut Sprite, &Position), With<Square>>,
    piece_query: Query<(&Position, &Player), With<Piece>>,
    colors: Res<Colors>,
    selected_piece_query: Query<(&Position, &PieceType, &Player), With<SelectedPiece>>,
    turn: Res<Turn>,
) {
    for e in ev_selected_piece.iter() {
        // if there is no selected piece don't populate anything
        if *e == SelectedPieceEvent::None {
            // dbg!("123123123");
            break;
        }
        // dbg!("ran available square system");

        // DEBUG
        if let Ok((selected_piece_position, selected_piece_type, owner)) =
            selected_piece_query.get_single()
        {
            // dbg!("owner of piece is", *owner);

            // create vector of all pieces
            let pieces: Vec<(&Position, &Player)> = piece_query.iter().collect();

            for (entity, mut sprite, to_position) in square_query.iter_mut().filter(|(_, _, to)| {
                !pieces
                    .iter()
                    .any(|(p, &o)| p.x == to.x && p.y == to.y && o == turn.player)
            }) {
                let (dy, dx) = (
                    to_position.y as i32 - selected_piece_position.y as i32,
                    to_position.x as i32 - selected_piece_position.x as i32,
                );

                let matches = match selected_piece_type {
                    PieceType::King => (-1..=1).contains(&dx) && (-1..=1).contains(&dy),
                    PieceType::Pawn => dx == 0 && dy == *owner as i32,
                    PieceType::Lance => match owner {
                        Player::Challenging => dx == 0 && dy >= 1,
                        Player::Residing => dx == 0 && dy <= -1,
                    },
                    PieceType::Knight => (dx == 1 || dx == -1) && dy == *owner as i32 * 2,
                    PieceType::Silver => {
                        let dxdy = [[-1, 1], [0, 1], [1, 1], [-1, -1], [1, -1]];

                        dxdy.iter()
                            .any(|&[ddx, ddy]| ddx == dx && ddy * *owner as i32 == dy)
                    }
                    PieceType::Gold => {
                        let dxdy = [[-1, 1], [0, 1], [1, 1], [-1, 0], [1, 0]];

                        dxdy.iter()
                            .any(|&[ddx, ddy]| ddx == dx && ddy * *owner as i32 == dy)
                    }
                    PieceType::Bishop => dx == dy,
                    PieceType::Rook => dx == 0 || dy == 0,
                };

                if matches
                    && !(dy == 0 && dx == 0)
                    && is_path_clear(selected_piece_position, to_position, &pieces)
                {
                    commands.entity(entity).insert(Available);
                    sprite.color = colors.green;
                }
            }
        }
    }
}

fn detect_removals(
    mut commands: Commands,
    mut removals: Query<(Entity, &mut Sprite), With<NegativeSelectedPiece>>,
    colors: Res<Colors>,
) {
    for (entity, mut sprite) in removals.iter_mut() {
        // dbg!("ran removal system");
        commands.entity(entity).remove::<NegativeSelectedPiece>();
        sprite.color = colors.dark;
    }
}