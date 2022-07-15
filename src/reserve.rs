use std::f32::consts::PI;

use bevy::{prelude::*, text::Text2dBounds};

use crate::mouse::*;
use crate::regular::DoneEvent;
use crate::*;

pub struct ReservePlugin;

impl Plugin for ReservePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_reserve)
            .add_system_to_stage(CoreStage::Last, reserve_system);
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
