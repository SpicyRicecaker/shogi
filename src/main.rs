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

struct Turn(Player);

struct SelectedPiece {
    position: Option<Position>,
}

#[derive(Component)]
struct Piece;

fn main() {
    App::new()
        .insert_resource(Colors::default())
        // default engine stuff begin
        .add_plugins(DefaultPlugins)
        .add_startup_system(camera)
        .insert_resource(ClearColor(Color::hex("282828").unwrap()))
        .insert_resource(Turn(Player::Residing))
        .insert_resource(SelectedPiece { position: None })
        .add_event::<ClickEvent>()
        // default engine stuff end
        .add_startup_system(spawn_squares)
        .add_startup_system(spawn_pieces)
        .add_system(mouse_system)
        .add_system(square_system)
        .run();
}

#[derive(Debug, Component, Clone, Copy)]
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

#[derive(Component, Clone, Copy)]
enum Player {
    Challenging,
    Residing,
}

fn is_valid_move() {}

fn is_piece_between() {}

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

            commands.spawn_bundle(Text2dBundle {
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
                    translation: Vec3::new(
                        (x as f32 - 4.5) * SQUARE_LENGTH,
                        (y as f32 - 4.5) * SQUARE_LENGTH + 2.,
                        2.0,
                    ),
                    ..default()
                },
                ..default()
            });

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
                .insert(Piece);
        }
    }
}

// it's hard to do a selected piece without a global resource, because sometimes we might not have a selected piece.
// regardless, I think that we should try out functional programming in full.

struct ClickEvent(Position);

fn mouse_system(
    square_query: Query<(&Transform, &Position), With<Square>>,
    // selected_piece: Res<SelectedPiece>,
    mut ev_click: EventWriter<ClickEvent>,
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
) {
    println!("123");
    // there can be a selected piece, but there's no such thing as a selected square
    let window = windows.get_primary().unwrap();

    if let Some(position) = window.cursor_position() {
        if buttons.just_pressed(MouseButton::Left) {
            // try to match it to a square
            // from x to x + scale, cursor position
            // from y to y + scale, cursor position
            for (transform, square_position) in square_query.iter() {
                if position.x >= transform.translation.x
                    && position.x <= transform.scale.x + transform.translation.x
                    && position.y >= transform.translation.y
                    && position.y <= transform.scale.y
                {
                    ev_click.send(ClickEvent(*square_position));
                    break;
                }
                // println!("{:#?}", transform.translation);
            }

            // println!("{:#?}", position);
        }
    }
}

fn square_system(
    mut ev_click: EventReader<ClickEvent>,
    mut square_query: Query<(&mut Sprite, &Position), With<Piece>>,
    mut selected_piece: ResMut<SelectedPiece>,
    colors: Res<Colors>
) {
    for e in ev_click.iter() {
        for (mut sprite, position) in square_query.iter_mut() {
            if position.x == e.0.x && position.y == e.0.y {
                selected_piece.position = Some(*position);
                sprite.color = colors.blue;
            }
        }
    }
}
