use std::collections::HashSet;

use bevy::text::Text2dBounds;

use crate::*;
use core::f32::consts::PI;

pub struct RegularPlugin;

pub struct DoneEvent;

impl Plugin for RegularPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SelectedPieceEvent>()
            .add_event::<DoneEvent>()
            .add_startup_system(spawn_squares)
            .add_startup_system(spawn_pieces)
            .add_system(detect_removals)
            .add_system_to_stage(CoreStage::PostUpdate, reset_square_system)
            .add_system_to_stage(CoreStage::Last, available_square_system)
            .add_system_to_stage(CoreStage::Last, check_win_system);
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

    // let pieces = r##"
    // lnsgkgsnl
    // .r.....b.
    // ppppppppp
    // .........
    // .........
    // .........
    // ppppppppp
    // .b.....r.
    // lnsgkgsnl
    // "##;
    let pieces = r##"
    r...k....
    .........
    .........
    .........
    .........
    .........
    p........
    r........
    r...k...r
    "##;

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

// Is there anyway we can do this without making a struct like this?

#[derive(Debug, Clone, Copy)]
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

fn is_in_check(
    sel_pc: XSelectedPiece,
    _reserve: Vec<(Reserve, Player, PieceType)>,
    pcs: Vec<(Position, Player, Rank, PieceType)>,
    squares: Vec<Position>,
) -> bool {
    if let XSelectedPiece::Board {
        p: p_sel,
        o: o_sel,
        r: r_sel,
        t: t_sel,
    } = sel_pc
    {
        // run available squares algorithm
        // the problem is this function does not even require the reserve
        // vector, but available square does, so we're going to have to use it

        // find the available squares of every piece that is not the same as the
        // selected piece

        pcs.iter()
            .filter(|&&(_, o, _, _)| o != o_sel)
            // who cares about 4 clones? not me
            .flat_map(|&(p, o, r, t)| {
                let curr_sel_pc = XSelectedPiece::Board { p, o, r, t };
                available_squares_iter(curr_sel_pc, _reserve.clone(), pcs.clone(), squares.clone())
                    .into_iter()
            })
            .any(|p| p == p_sel)
    } else {
        unreachable!();
    }
}

// literal clone of the version that mutates. I don't like ecs
fn move_to_position(
    sel_pc: XSelectedPiece,
    // can't actually do a simple Position, because we chose to use separate
    // arrays for reserve and pcs. So we're gonna use another piece as the to
    // position, and just filter it. Very efficient, I know
    to_position: Position,
    reserve: &mut Vec<(Reserve, Player, PieceType)>,
    pcs: &mut Vec<(Position, Player, Rank, PieceType)>,
    // realistically squares should be a hashset of position...
    squares: Vec<Position>,
) {
    match sel_pc {
        XSelectedPiece::Board {
            p: p_sel,
            o: o_sel,
            r: mut r_sel,
            t: t_sel,
        } => {
            let mut i = None;
            // if the position we're moving to is occupied, take the piece
            if let Some((idx, &(p, o, r, t))) = pcs
                .iter()
                .enumerate()
                .find(|(idx, &(p, _, _, _))| p == to_position)
            {
                // find the piece in reserve which matches the taken piece and increment it
                // double `&&` might error here
                if let Some((mut r_res, _, _)) = reserve
                    .iter_mut()
                    .find(|(_, o_res, t_res)| *t_res == t && *o_res == o_sel)
                {
                    r_res.quantity += 1;
                    // remove the piece from the board lol
                    // this is a reason why having pcs separate from reserve is
                    // not beneficial, technically pieces are never created nor
                    // destroyed in shogi, nor chess
                }
                i = Some(idx);
            }
            if let Some(i) = i {
                pcs.remove(i);
            }

            // then move the selected piece
            let x = to_position.x;
            let y = to_position.y;

            // if the position we're moving to is within the enemy's backrank, promote if we haven't already
            if match o_sel {
                Player::Residing => y <= 2,
                Player::Challenging => y >= 6,
            } {
                r_sel = Rank::Promoted;
            }

            // p_sel.x = x;
            // p_sel.y = y;

            // add the selected piece to the board (VERY BAD HACK, BECAUSE
            // LITERALLY EVERY OTHER FUNCTION ASSUMES THAT THE SELECTED PIECE IS
            // NOT ACTUALLY PART OF THE BOARD SO CARE)
            if let Some((p, _, r, _)) = pcs
                .iter_mut()
                .find(|(p, o, _, t)| *p == p_sel && *o == o_sel && *t == t_sel)
            {
                p.x = x;
                p.y = y;

                *r = r_sel;
            } else {
                panic!("hello world 123123123")
            }
        }
        XSelectedPiece::Reserve {
            r: mut r_sel,
            o: o_sel,
            t: t_sel,
        } => {
            if let Some((r, o, t)) = reserve
                .iter_mut()
                .find(|(r, o, t)| r.quantity > 0 && *o == o_sel && *t == t_sel)
            {
                r_sel.quantity -= 1;
                *r = r_sel;

                // decrement reserve
                pcs.push((to_position, o_sel, Rank::Regular, t_sel));
            } else {
                panic!();
            }
        }
    }
}

fn available_squares_iter_parent(
    sel_pc: XSelectedPiece,
    reserve: Vec<(Reserve, Player, PieceType)>,
    pcs: Vec<(Position, Player, Rank, PieceType)>,
    squares: Vec<Position>,
) -> Vec<Position> {
    let o_sel = match sel_pc {
        XSelectedPiece::Board { o, .. } => o,
        XSelectedPiece::Reserve { o, .. } => o,
    };
    // dbg!("123123123", pcs
    //     .iter()
    //     .filter(|&(_, o, _, t)| *o == o_sel && *t == PieceType::King)
    //     .copied()
    //     .collect::<Vec<(Position, Player, Rank, PieceType)>>());
    // for each naive square, create a new board where those pieces are moved, then check if the king is not under check
    // dbg!(
    //     "start",
    //     available_squares_iter(sel_pc, reserve.clone(), pcs.clone(), squares.clone()),
    //     "end"
    // );
    available_squares_iter(sel_pc, reserve.clone(), pcs.clone(), squares.clone())
        .into_iter()
        .filter(|p| {
            // dbg!(pcs
            //     .iter()
            //     .filter(|&(_, o, _, t)| *o == o_sel && *t == PieceType::King)
            //     .copied()
            //     .collect::<Vec<(Position, Player, Rank, PieceType)>>());
            // dbg!(p);
            let (mut reserve, mut pcs, squares) = (reserve.clone(), pcs.clone(), squares.clone());
            // dbg!(&reserve);
            move_to_position(sel_pc, *p, &mut reserve, &mut pcs, squares.clone());
            // check if the kingpiece of the owner of `sel_pc` (which is now invalidated btw) is under check
            // find king
            let king_pc = XSelectedPiece::Board {
                p: pcs
                    .iter()
                    .find(|&(_, o, _, t)| *o == o_sel && *t == PieceType::King)
                    .unwrap()
                    .0,
                o: o_sel,
                // why do I have to fill this out lol, I made a terrible system
                r: Rank::Regular,
                t: PieceType::King,
            };

            // dbg!(pcs
            //     .iter()
            //     .filter(|&(_, o, _, t)| *o == o_sel && *t == PieceType::King)
            //     .copied()
            //     .collect::<Vec<(Position, Player, Rank, PieceType)>>());

            // if pcs
            //     .iter()
            //     .filter(|&(_, o, _, t)| *o == o_sel && *t == PieceType::King)
            //     .copied()
            //     .collect::<Vec<(Position, Player, Rank, PieceType)>>().len() > 1 {
            //         println!("12312312313123p12312312313123123123123131231231231231312312312312313123");
            //     }
            // let idx = pcs
            //         .iter()
            //         .enumerate()
            //         .find(|(i, &(_, o, _, t))| o == o_sel && t == PieceType::King)
            //         .map(|(i, _)| i)
            //         .unwrap();
            // pcs.remove(idx);

            !is_in_check(king_pc, reserve, pcs, squares)
        })
        .collect()
}

// returns the valid available moves for a piece, given a board with all the pieces and a piece
// if we choose to use the recursion method, how do we know when to stop recursing? A move is only valid if after the selected piece is moved to a square, the king is no longer in check.
fn available_squares_iter(
    sel_pc: XSelectedPiece,
    reserve: Vec<(Reserve, Player, PieceType)>,
    pcs: Vec<(Position, Player, Rank, PieceType)>,
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
                .filter(|&&(_, o, _, _)| o == o_sel)
                .map(|&(p, _, _, _)| p)
                .collect();

            squares
                .into_iter()
                .filter(|&p| {
                    // remove the possibility of going to squares that are already owned
                    p != p_sel && !pcs_set.contains(&p)
                })
                .filter(|&to_pos| {
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
                            PieceType::Bishop => {
                                dx == dy || -dx == dy || {
                                    let dxdy = [[1, 0], [-1, 0], [0, 1], [0, -1]];

                                    dxdy.iter()
                                        .any(|&[ddx, ddy]| ddx == dx && ddy * o_sel as i32 == dy)
                                }
                            }
                            PieceType::Rook => {
                                dx == 0 || dy == 0 || {
                                    let dxdy = [[1, 1], [1, -1], [-1, 1], [-1, -1]];

                                    dxdy.iter()
                                        .any(|&[ddx, ddy]| ddx == dx && ddy * o_sel as i32 == dy)
                                }
                            }
                        },
                    };

                    let pcs: Vec<(Position, Player, Rank, PieceType)> =
                        pcs.iter().copied().collect();
                    matches && is_path_clear(p_sel, to_pos, pcs)
                    // {
                    // do this later, in the function with the query where we're
                    // calling this
                    // commands.entity(entity).insert(Available);
                    // sprite.color = colors.green;
                    // }
                })
                .collect()
            // .map(|p| *p)
            // for each available square of the piece, move it, then check if the king is still in check
            // .filter(|p_available| {
            //     // move the selected piece to the square
            //     // we *do not* have to promote the square at this point, because the rank of the piece shouldn't affect if it blocks any checks or not. We *DO* have to take pieces in this phase, though. I think functions in ECS by default should definitely accept normal Rust constructs. ECS is only a way to init into these functions easier
            //     let selected_piece = XSelectedPiece::Board {
            //         p: p_available,
            //         o,
            //         r,
            //         t
            //     };
            // })
        }
        XSelectedPiece::Reserve {
            r: r_sel,
            o: o_sel,
            t: t_sel,
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
                let pcs: HashSet<Position> = pcs.iter().map(|(p, o, r, t)| *p).collect();
                squares.into_iter().filter(|p| !pcs.contains(p)).collect()
            };

            // completely ignore all pieces
            match t_sel {
                PieceType::Pawn => {
                    let player_pawns: HashSet<usize> = pcs
                        .into_iter()
                        .filter(|&(_, o, r, t)| {
                            o == o_sel && r == Rank::Regular && t == PieceType::Pawn
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
    reserve_q: Query<(&Reserve, &Player, &PieceType)>,
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
            // dbg!("hello world kappa", piece_query
            //     .iter()
            //     .filter(|&(_, o, _, t)| *o == turn.player && *t == PieceType::King)
            //     .collect::<Vec<(&Position, &Player, &Rank, &PieceType)>>());

            // never fails, since we checked earlier
            let (e, o, r, t) = sel_pc_q.single();

            let squares: Vec<Position> = square_query.p0().iter().copied().collect();

            let available_squares = if let Ok(p) = pos_q.get(e) {
                let sel_pc = XSelectedPiece::Board {
                    p: *p,
                    o: *o,
                    r: *r,
                    t: *t,
                };
                let mut reserve: Vec<(Reserve, Player, PieceType)> =
                    reserve_q.iter().map(|(r, o, t)| (*r, *o, *t)).collect();
                let mut pcs: Vec<(Position, Player, Rank, PieceType)> = piece_query
                    .iter()
                    .map(|(p, o, r, t)| (*p, *o, *r, *t))
                    .collect();
                // pcs.push((*p, *o, *r, *t));

                // pcs includes the selected_piece by default, even though you'd think it wouldn't
                available_squares_iter_parent(sel_pc, reserve, pcs, squares)
            } else if let Ok(r) = res_q.get(e) {
                let sel_pc = XSelectedPiece::Reserve {
                    r: *r,
                    o: *o,
                    t: *t,
                };
                let mut reserve: Vec<(Reserve, Player, PieceType)> =
                    reserve_q.iter().map(|(r, o, t)| (*r, *o, *t)).collect();
                // reserve.push((*r, *o, *t));
                let mut pcs: Vec<(Position, Player, Rank, PieceType)> = piece_query
                    .iter()
                    .map(|(p, o, r, t)| (*p, *o, *r, *t))
                    .collect();

                // dbg!("123123123123123", &reserve);
                available_squares_iter_parent(sel_pc, reserve, pcs, squares)
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

/// Function that checks if game is won or not. If the current player's king is in check, checks if the player has any possible moves to get them out of check. If not, lets the other player win
/// Feels like this function can recurse...
fn check_win_system(
    mut ev_done: EventReader<DoneEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    colors: Res<Colors>,
    pcs_q: Query<(&Position, &Player, &Rank, &PieceType), With<Piece>>,
    res_q: Query<(&Reserve, &Player, &PieceType)>,
    squares: Query<&Position, With<Square>>,
    turn: Res<Turn>,
) {
    for e in ev_done.iter() {
        // query for the king of the current player
        if let Some((&p, &o, &r, &t)) = pcs_q
            .iter()
            .find(|&(_, o, _, t)| *o == turn.player && *t == PieceType::King)
        {
            let king_pc = XSelectedPiece::Board { p, o, r, t };
            // check if the king is in check
            let pcs: Vec<(Position, Player, Rank, PieceType)> =
                pcs_q.iter().map(|(p, o, r, t)| (*p, *o, *r, *t)).collect();

            let reserve: Vec<(Reserve, Player, PieceType)> =
                res_q.iter().map(|(r, o, t)| (*r, *o, *t)).collect();

            let squares: Vec<Position> = squares.iter().copied().collect();
            if is_in_check(king_pc, reserve.clone(), pcs.clone(), squares.clone()) {
                // dbg!("king is in check");
                // check if there is a single move the player can make to get themselves out of the situation
                if !pcs
                    .iter()
                    .filter(|&&(_, o, _, _)| o == turn.player)
                    .any(|&(p, o, r, t)| {
                        let sel_pc = XSelectedPiece::Board { p, o, r, t };

                        available_squares_iter(
                            sel_pc,
                            reserve.clone(),
                            pcs.clone(),
                            squares.clone(),
                        )
                        .iter()
                        .filter_map(|&p| {
                            let mut pcs = pcs.clone();
                            let mut reserve = reserve.clone();
                            let squares = squares.clone();

                            // dbg!("1231231313123123");
                            move_to_position(sel_pc, p, &mut reserve, &mut pcs, squares.clone());

                            // find king piece again
                            let king_pc = if let Some(&(p, o, r, t)) = pcs
                                .iter()
                                .find(|&&(_, o, _, t)| o == turn.player && t == PieceType::King)
                            {
                                XSelectedPiece::Board { p, o, r, t }
                            } else {
                                unreachable!()
                            };

                            if !is_in_check(king_pc, reserve.clone(), pcs.clone(), squares) {
                                Some(3)
                            } else {
                                None
                            }
                        }).count() != 0
                    })
                    && !reserve
                        .iter()
                        .filter(|&&(r, o, _)| o == turn.player && r.quantity > 0)
                        .any(|&(r, o, t)| {
                            let sel_pc = XSelectedPiece::Reserve { r, o, t };

                            available_squares_iter(
                                sel_pc,
                                reserve.clone(),
                                pcs.clone(),
                                squares.clone(),
                            )
                            .iter()
                            .filter_map(|&p| {
                                let mut pcs = pcs.clone();
                                let mut reserve = reserve.clone();
                                let squares = squares.clone();

                                // dbg!("123123123");
                                move_to_position(
                                    sel_pc,
                                    p,
                                    &mut reserve,
                                    &mut pcs,
                                    squares.clone(),
                                );

                                // find king piece again
                                let king_pc = if let Some(&(p, o, r, t)) = pcs
                                    .iter()
                                    .find(|&&(_, o, _, t)| o == turn.player && t == PieceType::King)
                                {
                                    XSelectedPiece::Board { p, o, r, t }
                                } else {
                                    unreachable!()
                                };

                                if !is_in_check(king_pc, reserve.clone(), pcs.clone(), squares) {
                                    Some(3)
                                } else {
                                    None
                                }
                            }).count() != 0
                        })
                // also add check for things in bench
                {
                    // iterate over every single piece here and make it all black or something
                    let font = asset_server.load("yujiboku.ttf");
                    let text_style = TextStyle {
                        font,
                        font_size: 100.0,
                        color: colors.purple,
                    };
                    let text_alignment_center = TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    };
                    let box_size = Size::new(400.0, 400.0);

                    commands.spawn_bundle(Text2dBundle {
                        text: Text::with_section(
                            match turn.player {
                                Player::Residing => {
                                    "challenging player lost, better luck next time"
                                }
                                Player::Challenging => {
                                    "residing player lost, oh how the might have fallen"
                                }
                            },
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
                            translation: match turn.player {
                                Player::Challenging => Vec3::new(0.0, 2.0, 5.0),
                                Player::Residing => Vec3::new(0.0, -2.0, 5.0),
                            },
                            // rotation: match turn.player {
                            //     Player::Challenging => Quat::from_rotation_z(0.),
                            //     Player::Residing => Quat::from_rotation_z(PI),
                            // },
                            ..default() // scale: todo!(),
                        },
                        ..default()
                    });
                }
            }
        }
    }
}
