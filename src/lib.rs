use bevy::prelude::*;
use bevy::math::const_vec3;

pub const SQUARE_LENGTH: f32 = 50.0;
pub const SQUARE_SIZE: Vec3 = const_vec3!([50.0, 50.0, 0.0]);
pub const SQUARE_BORDER: f32 = 2.0;

pub mod mouse;

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

#[derive(Component, Clone, Copy)]
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