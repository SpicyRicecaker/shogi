use std::f32::consts::PI;

use bevy::{prelude::*, text::Text2dBounds};

use crate::mouse::*;
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

    for player in [Player::Challenging, Player::Residing].into_iter() {
        let start_y = match player {
            Player::Challenging => -50.0 * 1.5,
            Player::Residing => 50.0 * 9.5,
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
                            0.
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                });
                // .insert(Reserve)
                // Not sure if we should include the square here, because then we'll need to change
                // literally every single query
                // .insert(Square);
            // then spawn the piece, with the wood, kanji, and count
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

            // then spawn the piece as a child of the square
            
            // then spawn the count as a child of the piece
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
    mut reserve_query: Query<(&mut Reserve, &PieceType, &Player, &Children)>,
    reserve_query_child: Query<&mut Sprite>,
) {
    for e in ev_take.iter() {
        if let Some((mut reserve, _, _, child)) = reserve_query
            .iter_mut()
            .find(|(_, &pt, &o, _)| pt == e.piece_type && o == e.taker)
        {
            reserve.quantity += 1;
            // update sprite of child
        }
    }
}
