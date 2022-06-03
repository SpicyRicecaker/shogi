use std::f32::consts::PI;

use bevy::{math::const_vec3, prelude::*, text::Text2dBounds};

const SQUARE_LENGTH: f32 = 50.0;
const SQUARE_SIZE: Vec3 = const_vec3!([50.0, 50.0, 0.0]);
const SQUARE_BORDER: f32 = 2.0;

struct Colors {
    dark: Color,
    light: Color,
    red: Color,
    green: Color,
    yellow: Color,
    blue: Color,
    purple: Color,
    aqua: Color,
    orange: Color,
}

impl Default for Colors {
    fn default() -> Self {
        let hex = |s: &str| Color::hex(s).unwrap();
        Self {
            dark: hex("282828"),
            light: hex("fbf1c7"),
            red: hex("cc241d"),
            green: hex("98971a"),
            yellow: hex("d79921"),
            blue: hex("458588"),
            purple: hex("b16286"),
            aqua: hex("689d6a"),
            orange: hex("d65d0e"),
        }
    }
}

#[derive(Debug)]
struct Turn {
    player: Player,
}

#[derive(Component)]
struct SelectedPiece;

#[derive(Component)]
struct NegativeSelectedPiece;

#[derive(Component)]
struct Piece;

fn main() {
    App::new()
        .insert_resource(Colors::default())
        // default engine stuff begin
        .add_plugins(DefaultPlugins)
        .add_startup_system(camera)
        .insert_resource(ClearColor(Color::hex("282828").unwrap()))
        .insert_resource(Turn {
            player: Player::Challenging,
        })
        // .insert_resource(SelectedPiece { position: None })
        .add_event::<ClickEvent>()
        .add_event::<SelectedPieceEvent>()
        // default engine stuff end
        .add_startup_system(spawn_squares)
        .add_startup_system(spawn_pieces)
        .add_startup_system(spawn_debug)
        .add_system(mouse_system)
        .add_system(square_system)
        .add_system(available_square_system)
        .add_system(move_system)
        .add_system(detect_removals)
        .add_system(debug_system)
        .add_system_to_stage(CoreStage::PostUpdate, reset_square_system)
        .add_system_to_stage(CoreStage::Last, available_square_system)
        .run();
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Component, Clone, Copy)]
enum PieceType {
    King,
    Pawn,
    Lance,
    Knight,
    Silver,
    Gold,
    Bishop,
    Rook,
}

#[derive(Component)]
struct Square;

#[derive(Component)]
struct Selected;

#[derive(Component)]
struct Available;

impl From<char> for PieceType {
    fn from(c: char) -> Self {
        match c {
            'k' => PieceType::King,
            'p' => PieceType::Pawn,
            'l' => PieceType::Lance,
            'n' => PieceType::Knight,
            's' => PieceType::Silver,
            'g' => PieceType::Gold,
            'b' => PieceType::Bishop,
            'r' => PieceType::Rook,
            _ => panic!("doesnt work `{}`", c),
        }
    }
}

#[derive(Component)]
enum Rank {
    Regular,
    Promoted,
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Player {
    Challenging = 1,
    Residing = -1,
}

impl Player {
    fn swap(&self) -> Self {
        match self {
            Player::Challenging => Player::Residing,
            Player::Residing => Player::Challenging,
        }
    }
}

fn camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

// fn font(mut commands: Commands, asset_server: Res<AssetServer>) {
// mut commands: Commands
// }

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
                            (i as f32 - 4.5) * SQUARE_LENGTH,
                            (j as f32 - 4.5) * SQUARE_LENGTH,
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

fn get_kanji(piece_type: PieceType, rank: Rank, owner: Player) -> char {
    match piece_type {
        PieceType::King => match owner {
            Player::Challenging => '玉',
            Player::Residing => '王',
        },
        PieceType::Pawn => match rank {
            Rank::Regular => '歩',
            Rank::Promoted => 'と',
        },
        PieceType::Lance => match rank {
            Rank::Regular => '香',
            Rank::Promoted => '杏',
        },
        PieceType::Knight => match rank {
            Rank::Regular => '桂',
            Rank::Promoted => '今',
        },
        PieceType::Silver => match rank {
            Rank::Regular => '銀',
            Rank::Promoted => '全',
        },
        PieceType::Gold => '金',
        PieceType::Bishop => match rank {
            Rank::Regular => '角',
            Rank::Promoted => '馬',
        },
        PieceType::Rook => match rank {
            Rank::Regular => '飛',
            Rank::Promoted => '竜',
        },
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
            let player = if y <= 3 {
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
                            SQUARE_LENGTH - 4. * SQUARE_BORDER,
                            SQUARE_LENGTH - 4. * SQUARE_BORDER,
                        )),
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(
                            (x as f32 - 4.5) * SQUARE_LENGTH,
                            (y as f32 - 4.5) * SQUARE_LENGTH,
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
                                Player::Challenging => Vec3::new(
                                    0.0,
                                    2.0,
                                    2.0,
                                ),
                                Player::Residing => Vec3::new(
                                    0.0,
                                    -2.0,
                                    2.0,
                                ),
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

// it's hard to do a selected piece without a global resource, because sometimes we might not have a selected piece.
// regardless, I think that we should try out functional programming in full.

struct ClickEvent {
    position: Position,
}

#[derive(Debug, PartialEq, Eq)]
enum SelectedPieceEvent {
    Change,
    None,
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

fn translate_transform(x: f32, y: f32, owner: &Player) -> Vec3 {
    match owner {
        Player::Challenging => Vec3::new(
            (x as f32 - 4.5) * SQUARE_LENGTH,
            (y as f32 - 4.5) * SQUARE_LENGTH + 2.,
            2.0,
        ),
        Player::Residing => Vec3::new(
            (x as f32 - 4.5) * SQUARE_LENGTH,
            (y as f32 - 4.5) * SQUARE_LENGTH - 2.,
            2.0,
        ),
    }
}

fn move_system(
    mut ev_click: EventReader<ClickEvent>,
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

                    turn.player = turn.player.swap();
                }
            }
        }
    }
}

fn square_system(
    mut commands: Commands,
    mut ev_click: EventReader<ClickEvent>,
    mut piece_query: Query<(Entity, &mut Sprite, &Position, &Player), With<Piece>>,
    selected_piece: Query<(Entity, &Position), With<SelectedPiece>>,
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
                    selected_piece.get_single()
                {
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
                } else {
                    commands.entity(entity).insert(SelectedPiece);
                    ev_selected_piece.send(SelectedPieceEvent::Change);
                    sprite.color = colors.blue;
                    // dbg!("cool2");
                };

                break;
            }
        }
        // run system to check if an available square has been clicked
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

fn is_path_clear(start: &Position, end: &Position, pieces: &[&Position]) -> bool {
    let polar_maker = |startx: f32, starty: f32, endx: f32, endy: f32| -> (f32, f32) {
        let dy = endy as f32 - starty as f32;
        let dx = endx as f32 - startx as f32;

        ((dy).atan2(dx), (dy.powi(2) + dx.powi(2)).sqrt())
    };

    let (trajectory_angle, trajectory_magnitude) =
        polar_maker(start.x as f32, start.y as f32, end.x as f32, end.y as f32);

    // dbg!("runinng");

    !pieces
        .iter()
        // do not include the piece itself in consideration
        .filter(|p| !(p.x == start.x && p.y == start.y))
        .any(|piece| {
            let (this_trajectory_angle, this_trajectory_magnitude) = polar_maker(
                start.x as f32,
                start.y as f32,
                piece.x as f32,
                piece.y as f32,
            );

            // dbg!(this_trajectory_angle, trajectory_angle);
            // dbg!(this_trajectory_magnitude, trajectory_magnitude);

            this_trajectory_angle == trajectory_angle
                && this_trajectory_magnitude < trajectory_magnitude
        })
}

fn available_square_system(
    mut commands: Commands,
    mut ev_selected_piece: EventReader<SelectedPieceEvent>,
    mut square_query: Query<(Entity, &mut Sprite, &Position), With<Square>>,
    piece_query: Query<&Position, With<Piece>>,
    colors: Res<Colors>,
    selected_piece_query: Query<(&Position, &PieceType, &Player), With<SelectedPiece>>,
    // turn: Res<Turn>,
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
            let pieces: Vec<&Position> = piece_query.iter().collect();

            for (entity, mut sprite, to_position) in square_query.iter_mut() {
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

fn debug_system(windows: Res<Windows>) {
    // let window = windows.get_primary().unwrap();

    // dbg!(window.height(), window.width());
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

#[cfg(test)]
mod test {
    use crate::{is_path_clear, Position};

    #[test]
    fn test_is_path_clear() {
        let pieces = vec![&Position { x: 1, y: 1 }, &Position { x: 2, y: 2 }];
        assert!(!is_path_clear(
            &Position { x: 0, y: 0 },
            &Position { x: 3, y: 3 },
            &pieces
        ));
        assert!(is_path_clear(
            &Position { x: 0, y: 0 },
            &Position { x: 1, y: 1 },
            &pieces
        ));
        assert!(is_path_clear(
            &Position { x: 1, y: 1 },
            &Position { x: 0, y: 0 },
            &pieces
        ));
    }
    #[test]
    fn test_is_path_clear_rook() {
        let pieces = vec![];

        let start = &Position { x: 1, y: 7 };
        let end = &Position { x: 2, y: 7 };

        assert!(is_path_clear(start, end, &pieces));
    }
}
