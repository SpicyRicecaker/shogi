use std::collections::HashSet;
use std::f32::consts::PI;

use bevy::{prelude::*, text::Text2dBounds};

use crate::mouse::*;
use crate::*;

pub struct ReservePlugin;

impl Plugin for ReservePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_reserve)
            .add_system_to_stage(CoreStage::Last, reserve_system)
            .add_system_to_stage(CoreStage::Last, available_square_system);
    }
}

fn spawn_reserve(mut commands: Commands, colors: Res<Colors>, asset_server: Res<AssetServer>) {
    let font = asset_server.load("yujiboku.ttf");

    let box_size = Size::new(48.0, 48.0);

    for player in [Player::Challenging, Player::Residing].into_iter() {
        let start_y = match player {
            Player::Challenging => -SQUARE_LENGTH * 1.5,
            Player::Residing => SQUARE_LENGTH * 9.5,
        };

        for (idx, piece_type) in [
            PieceType::Pawn,
            PieceType::Silver,
            PieceType::Gold,
            PieceType::Knight,
            PieceType::Lance,
            PieceType::Bishop,
            PieceType::Rook,
        ]
        .into_iter()
        .enumerate()
        {
            // first spawn the square
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: colors.light,
                        ..Default::default()
                    },
                    transform: Transform {
                        scale: SQUARE_SIZE,
                        translation: Vec3::new(
                            RESERVE_X_OFFSET + SQUARE_LENGTH * idx as f32 + BOARD_X_OFFSET,
                            start_y + BOARD_Y_OFFSET,
                            0.,
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                // .insert(Reserve)
                // Not sure if we should include the square here, because then we'll need to change
                // literally every single query
                .insert(Square)
                .insert(piece_type)
                .insert(player);
            // then spawn the piece, with the wood, kanji, and count
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: {
                            let mut color = colors.dark;
                            color.set_a(0.2);
                            color
                        },
                        custom_size: Some(Vec2::new(
                            SQUARE_LENGTH - SQUARE_BORDER,
                            SQUARE_LENGTH - SQUARE_BORDER,
                        )),
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(
                            RESERVE_X_OFFSET + idx as f32 * SQUARE_LENGTH + BOARD_X_OFFSET,
                            start_y + BOARD_Y_OFFSET,
                            1.0,
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(piece_type)
                .insert(player)
                .insert(Rank::Regular)
                .insert(Reserve { quantity: 0 })
                // .insert(Piece)
                .with_children(|parent| {
                    parent.spawn_bundle(Text2dBundle {
                        text: Text::with_section(
                            get_kanji(piece_type, Rank::Regular, player),
                            TextStyle {
                                font: font.clone(),
                                font_size: 50.0,
                                color: colors.light,
                            },
                            TextAlignment {
                                vertical: VerticalAlign::Center,
                                horizontal: HorizontalAlign::Center,
                            },
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
                    parent
                        .spawn_bundle(Text2dBundle {
                            text: Text::with_section(
                                format!("{}", 0),
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 25.0,
                                    color: colors.light,
                                },
                                TextAlignment {
                                    vertical: VerticalAlign::Bottom,
                                    horizontal: HorizontalAlign::Right,
                                },
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
                                    Player::Challenging => Vec3::new(20.0, 3.0, 2.0),
                                    Player::Residing => Vec3::new(-20.0, -3.0, 2.0),
                                },
                                rotation: match player {
                                    Player::Challenging => Quat::from_rotation_z(0.),
                                    Player::Residing => Quat::from_rotation_z(PI),
                                },
                                ..default() // scale: todo!(),
                            },
                            ..default()
                        })
                        .insert(Counter);
                });
        }
    }
}

// we should actually just fully delete the position entity instead of trying to mutate it, and
// have every reserve be unique
// each reserve has
// - sprite
// - transform
// - count
// - piece_type
fn reserve_system(
    mut ev_take: EventReader<TakeEvent>,
    mut q_child: Query<(&mut Sprite, &mut Reserve, &PieceType, &Player, &Children)>,
    mut q_counter: Query<&mut Text, With<Counter>>,
) {
    for e in ev_take.iter() {
        if let Some((mut sprite, mut reserve, _, _, children)) = q_child
            .iter_mut()
            .find(|(_, _, &pt, &o, _)| pt == e.piece_type && o == e.taker)
        {
            reserve.quantity += 1;
            sprite.color.set_a(1.0);

            for &child in children.iter() {
                if let Ok(mut text) = q_counter.get_mut(child) {
                    text.sections
                        .get_mut(0)
                        .expect("error getting text field of reserve counter entity")
                        .value = format!("{}", reserve.quantity);
                }
            }
        }
    }
}

fn available_square_system(
    mut commands: Commands,
    mut ev_selected_piece: EventReader<SelectedPieceEvent>,
    mut square_query: Query<(Entity, &mut Sprite, &Position), With<Square>>,
    piece_query: Query<(&Position, &Player, &Rank, &PieceType), With<Piece>>,
    colors: Res<Colors>,
    selected_piece_query: Query<(&Reserve, &PieceType, &Player), With<SelectedPiece>>,
    turn: Res<Turn>,
) {
    for e in ev_selected_piece
        .iter()
        .filter(|e| **e != SelectedPieceEvent::None)
    {
        // DEBUG
        if let Ok((sel_pc_reserve, sel_pc_type, owner)) = selected_piece_query.get_single() {
            // two ways we can go about this
            //
            // 1.
            // turn squares into a hashmap
            // then iterate over pieces and remove keys from the hashmap
            //
            // 2.
            // turn pieces into a hashmap
            // iterate over squares, then filter them with pieces
            //
            // the end goal is to create a vector of squares, so 2 seems to make more sense since
            // we can collect at the end

            // create list of squares which are not occupied by pieces
            let free_squares: Vec<(Entity, Mut<Sprite>, &Position)> = {
                let pieces: HashSet<Position> = piece_query.iter().map(|(&p, _, _, _)| p).collect();
                square_query
                    .iter_mut()
                    .filter(|(_, _, p)| !pieces.contains(p))
                    .collect()
            };

            // completely ignore all pieces
            match sel_pc_type {
                PieceType::Pawn => {
                    let player_pawns: HashSet<usize> = piece_query
                        .iter()
                        .filter(|(_, o, r, t)| {
                            **o == turn.player && **r == Rank::Regular && **t == PieceType::Pawn
                        })
                        .map(|(p, _, _, _)| p.x)
                        .collect();

                    free_squares
                        .into_iter()
                        .filter(|(_, _, p)| match turn.player {
                            Player::Challenging => p.y != 8,
                            Player::Residing => p.y != 0,
                        } && !player_pawns.contains(&p.x))
                        .for_each(|(entity, mut sprite, position)| {
                            commands.entity(entity).insert(Available);
                            sprite.color = colors.green;
                        });

                    // for pawn, we want to generate a hashset using the x values of the pawns (unpromoted) of the current player,
                    //
                    // then only allow the pawn to be placed on that square if
                    // 1. it is not on the last row, and
                    // - for challenging, ne 8, for residing, ne 0
                    // 2. that square is not within the hashmap
                }
                PieceType::Bishop | PieceType::Rook | PieceType::Gold | PieceType::Silver => {
                    free_squares
                        .into_iter()
                        .for_each(|(entity, mut sprite, position)| {
                            commands.entity(entity).insert(Available);
                            sprite.color = colors.green;
                        });
                }
                PieceType::Lance => {
                    free_squares
                        .into_iter()
                        // disallow last row
                        .filter(|(_, _, p)| match turn.player {
                            Player::Challenging => p.y != 8,
                            Player::Residing => p.y != 0,
                        })
                        // I have a feeling that manually inserting this is terrible
                        .for_each(|(entity, mut sprite, position)| {
                            commands.entity(entity).insert(Available);
                            sprite.color = colors.green;
                        });
                }
                PieceType::Knight => {
                    free_squares
                        .into_iter()
                        // disallow last two rows
                        .filter(|(_, _, p)| match turn.player {
                            Player::Challenging => p.y <= 6,
                            Player::Residing => p.y >= 2,
                        })
                        // I have a feeling that manually inserting this is terrible
                        .for_each(|(entity, mut sprite, position)| {
                            commands.entity(entity).insert(Available);
                            sprite.color = colors.green;
                        });
                }
                _ => unreachable!(),
            }
        }
    }
}
