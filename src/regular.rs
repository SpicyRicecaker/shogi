use std::collections::HashSet;

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

// ECS is so fucking useful by the way
// Is there anyway we can do this without making a struct like this?

enum XSelectedPiece {
    Board {
        p: Position,
        o: Player,
        r: Rank,
        t: PieceType,
    },
    Reserve {
        r: Reserve,
        o: Player,
        t: PieceType,
    },
}

fn available_squares_iter(
    sel_pc: XSelectedPiece,
    reserve: &[(&Reserve, &Player, &PieceType)],
    pcs: &[(&Position, &Player, &Rank, &PieceType)],
    squares: Vec<Position>,
) -> Vec<Position> {
    match sel_pc {
        XSelectedPiece::Board {
            p: p_sel,
            o: o_sel,
            r: r_sel,
            t: t_sel,
        } => {
            // ignore self pieces
            let pcs_set: HashSet<Position> = pcs
                .iter()
                .filter(|(_, o, _, _)| **o == o_sel)
                .map(|(p, _, _, _)| **p)
                .collect();

            squares
                .iter()
                .filter(|p| {
                    // remove the possibility of going to squares that are already owned
                    **p != p_sel && !pcs_set.contains(p)
                })
                .filter(|to_pos| {
                    let (dy, dx) = (
                        to_pos.y as i32 - p_sel.y as i32,
                        to_pos.x as i32 - p_sel.x as i32,
                    );

                    let matches = match r_sel {
                        Rank::Regular => match t_sel {
                            PieceType::King => (-1..=1).contains(&dx) && (-1..=1).contains(&dy),
                            PieceType::Pawn => dx == 0 && dy == o_sel as i32,
                            PieceType::Lance => match o_sel {
                                Player::Challenging => dx == 0 && dy >= 1,
                                Player::Residing => dx == 0 && dy <= -1,
                            },
                            PieceType::Knight => (dx == 1 || dx == -1) && dy == o_sel as i32 * 2,
                            PieceType::Silver => {
                                let dxdy = [[-1, 1], [0, 1], [1, 1], [-1, -1], [1, -1]];

                                dxdy.iter()
                                    .any(|&[ddx, ddy]| ddx == dx && ddy * o_sel as i32 == dy)
                            }
                            PieceType::Gold => {
                                let dxdy = [[-1, 1], [0, 1], [1, 1], [-1, 0], [1, 0]];

                                dxdy.iter()
                                    .any(|&[ddx, ddy]| ddx == dx && ddy * o_sel as i32 == dy)
                            }
                            PieceType::Bishop => dx == dy || -dx == dy,
                            PieceType::Rook => dx == 0 || dy == 0,
                        },
                        // make sure to trim
                        Rank::Promoted => match t_sel {
                            PieceType::King => unreachable!(),
                            // gold movement
                            PieceType::Pawn
                            | PieceType::Lance
                            | PieceType::Gold
                            | PieceType::Silver
                            | PieceType::Knight => {
                                let dxdy = [[-1, 1], [0, 1], [1, 1], [-1, 0], [1, 0]];

                                dxdy.iter()
                                    .any(|&[ddx, ddy]| ddx == dx && ddy * o_sel as i32 == dy)
                            }
                            // add corner squares
                            PieceType::Bishop => dx == dy || -dx == dy || {
                                let dxdy = [[1, 0], [-1, 0], [0, 1], [0, -1]];

                                dxdy.iter()
                                    .any(|&[ddx, ddy]| ddx == dx && ddy * o_sel as i32 == dy)
                            },
                            PieceType::Rook => dx == 0 || dy == 0 || {
                                let dxdy = [[1, 1], [1, -1], [-1, 1], [-1, -1]];

                                dxdy.iter()
                                    .any(|&[ddx, ddy]| ddx == dx && ddy * o_sel as i32 == dy)
                            },
                        },
                    };

                    let pcs: Vec<(&Position, &Player, &Rank, &PieceType)> =
                        pcs.iter().map(|tup| *tup).collect();
                    matches && is_path_clear(&p_sel, &to_pos, &pcs[..])
                    // {
                    // do this later, in the function with the query where we're
                    // calling this
                    // commands.entity(entity).insert(Available);
                    // sprite.color = colors.green;
                    // }
                })
                .map(|p| *p)
                .collect()
        }
        XSelectedPiece::Reserve {
            r: r_sel,
            t: t_sel,
            o: o_sel,
        } => {
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
            let free_squares: Vec<Position> = {
                let pcs: HashSet<Position> = pcs.iter().map(|(p, o, r, t)| **p).collect();
                squares.into_iter().filter(|p| !pcs.contains(&p)).collect()
            };

            // completely ignore all pieces
            match t_sel {
                PieceType::Pawn => {
                    let player_pawns: HashSet<usize> = pcs
                        .iter()
                        .filter(|(_, o, r, t)| {
                            **o == o_sel && **r == Rank::Regular && **t == PieceType::Pawn
                        })
                        .map(|(p, _, _, _)| p.x)
                        .collect();

                    free_squares
                        .into_iter()
                        .filter(|p| match o_sel {
                            Player::Challenging => p.y != 8,
                            Player::Residing => p.y != 0,
                        } && !player_pawns.contains(&p.x))
                        .collect()

                    // for pawn, we want to generate a hashset using the x values of the pawns (unpromoted) of the current player,
                    //
                    // then only allow the pawn to be placed on that square if
                    // 1. it is not on the last row, and
                    // - for challenging, ne 8, for residing, ne 0
                    // 2. that square is not within the hashmap
                }
                PieceType::Bishop | PieceType::Rook | PieceType::Gold | PieceType::Silver => {
                    free_squares
                }
                PieceType::Lance => {
                    free_squares
                        .into_iter()
                        // disallow last row
                        .filter(|p| match o_sel {
                            Player::Challenging => p.y != 8,
                            Player::Residing => p.y != 0,
                        })
                        .collect()
                }
                PieceType::Knight => {
                    free_squares
                        .into_iter()
                        // disallow last two rows
                        .filter(|p| match o_sel {
                            Player::Challenging => p.y <= 6,
                            Player::Residing => p.y >= 2,
                        })
                        .collect()
                }
                _ => unreachable!(),
            }
        }
    }
}

fn available_square_system(
    mut commands: Commands,
    mut ev_selected_piece: EventReader<SelectedPieceEvent>,
    mut square_query: ParamSet<(
        Query<&Position, With<Square>>,
        Query<(Entity, &mut Sprite, &Position), With<Square>>,
    )>,
    piece_query: Query<(&Position, &Player, &Rank, &PieceType), With<Piece>>,
    reserve_q: Query<(&Reserve, &Player, &PieceType), With<Piece>>,
    colors: Res<Colors>,
    sel_pc_q: Query<(Entity, &Player, &Rank, &PieceType), With<SelectedPiece>>,
    pos_q: Query<&Position, With<SelectedPiece>>,
    res_q: Query<&Reserve, With<SelectedPiece>>,
    turn: Res<Turn>,
) {
    ev_selected_piece
        .iter()
        // if there is no selected piece don't populate anything
        .filter(|e| **e != SelectedPieceEvent::None)
        .for_each(|e| {
            // never fails, since we checked earlier
            let (e, o, r, t) = sel_pc_q.single();

            let squares: Vec<Position> = square_query.p0().iter().map(|p| *p).collect();

            let available_squares = if let Ok(p) = pos_q.get(e) {
                let sel_pc = XSelectedPiece::Board {
                    p: *p,
                    o: *o,
                    r: *r,
                    t: *t,
                };
                let reserve: Vec<(&Reserve, &Player, &PieceType)> = reserve_q.iter().collect();
                let mut pcs: Vec<(&Position, &Player, &Rank, &PieceType)> =
                    piece_query.iter().collect();
                pcs.push((p, o, r, t));

                available_squares_iter(sel_pc, &reserve, &pcs, squares)
            } else if let Ok(r) = res_q.get(e) {
                let sel_pc = XSelectedPiece::Reserve {
                    r: *r,
                    o: *o,
                    t: *t,
                };
                let mut reserve: Vec<(&Reserve, &Player, &PieceType)> = reserve_q.iter().collect();
                reserve.push((r, o, t));
                let pcs: Vec<(&Position, &Player, &Rank, &PieceType)> =
                    piece_query.iter().collect();

                available_squares_iter(sel_pc, &reserve, &pcs, squares)
            } else {
                unreachable!();
            };

            if available_squares.is_empty() {
                // // declare game state as won, exit process, probably
                // // DEBUG
                // println!(
                //     "{:?} won, gg.",
                //     match turn.player {
                //         Player::Residing => "challenging",
                //         Player::Challenging => "residing",
                //     }
                // );
                // std::process::exit(0);
            } else {
                let pos_set: HashSet<Position> = available_squares.into_iter().collect();

                // match the squares with sprites against the hashmap we generate here
                square_query
                    .p1()
                    .iter_mut()
                    .filter(|(_, _, p)| pos_set.contains(p))
                    .for_each(|(e, mut s, _)| {
                        commands.entity(e).insert(Available);
                        s.color = colors.green;
                    });
            }
        });
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

// FIXME need to disallow taking the king

// need a way to limit the amount of available squares based off of if moving
// that square will lead to the king being still in check currently or becoming
// checked (because there's a pin in place)
// this is actually pretty difficult, since we will have to check the available squares of a bord in the future
// Find available squares needs to
// 1. Take in a vector of pieces and a vector of reserves, instead of querying things
// 2. Needs to return an iterator (or vector, or whatever) of available pieces instead of actually mutating things
// We 100 % need to make this a recursive function, since because of the dynamicity of shogi, there's an untold number of moves that can be made with the reserve
