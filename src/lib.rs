use bevy::math::const_vec3;
use bevy::prelude::*;

pub const SQUARE_LENGTH: f32 = 50.0;
pub const SQUARE_SIZE: Vec3 = const_vec3!([50.0, 50.0, 0.0]);
pub const SQUARE_BORDER: f32 = 8.0;

pub const BOARD_X_OFFSET: f32 = -250.0;
pub const BOARD_Y_OFFSET: f32 = -200.0;
pub const RESERVE_X_OFFSET: f32 = 50.0;

pub mod debug;
pub mod mouse;
pub mod reserve;

pub struct Colors {
    pub dark: Color,
    pub light: Color,
    pub red: Color,
    pub green: Color,
    pub yellow: Color,
    pub blue: Color,
    pub purple: Color,
    pub aqua: Color,
    pub orange: Color,
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
pub struct Turn {
    pub player: Player,
}

#[derive(Component)]
pub struct SelectedPiece;

#[derive(Component)]
pub struct NegativeSelectedPiece;

#[derive(Component)]
pub struct Piece;

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
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
pub struct Square;

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct Available;

/// If a piece is in reserve, it can be played wheneve
#[derive(Component)]
pub struct Reserve {
    quantity: u8,
}

#[derive(Component)]
pub struct Counter;

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
pub enum Rank {
    Regular,
    Promoted,
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Player {
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

#[derive(Debug, PartialEq, Eq)]
pub enum SelectedPieceEvent {
    Change,
    None,
}

pub fn translate_transform(x: f32, y: f32, owner: &Player) -> Vec3 {
    match owner {
        Player::Challenging => Vec3::new(
            x * SQUARE_LENGTH + BOARD_X_OFFSET,
            y * SQUARE_LENGTH + BOARD_Y_OFFSET,
            2.0,
        ),
        Player::Residing => Vec3::new(
            x * SQUARE_LENGTH + BOARD_X_OFFSET,
            y * SQUARE_LENGTH + BOARD_Y_OFFSET,
            2.0,
        ),
    }
}

pub fn get_kanji(piece_type: PieceType, rank: Rank, owner: Player) -> char {
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
