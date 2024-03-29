#![allow(
  clippy::too_many_arguments,
  clippy::type_complexity,
)]

use bevy::math::const_vec3;
use bevy::prelude::*;

pub const SQUARE_LENGTH: f32 = 50.0;
pub const SQUARE_SIZE: Vec3 = const_vec3!([50.0, 50.0, 0.0]);
pub const SQUARE_BORDER: f32 = 8.0;

pub mod bindgen;
pub mod debug;
pub mod mouse;
pub mod regular;
pub mod reserve;
#[cfg(test)]
pub mod tests;

pub struct Board {
    pub x_offset: f32,
    pub y_offset: f32,
    pub x_reserve_offset: f32,
}

impl FromWorld for Board {
    fn from_world(world: &mut World) -> Self {
        let windows = world.get_resource::<Windows>().unwrap();
        let window = windows.get_primary().unwrap();

        // square_length / 2. because bevy spawns shapes at the center for no reason
        let x_offset =
            (window.width() - SQUARE_LENGTH * 9.) / 2. - window.width() / 2. + SQUARE_LENGTH / 2.;

        let y_offset =
            (window.height() - SQUARE_LENGTH * 9.) / 2. - window.height() / 2. + SQUARE_LENGTH / 2.;

        let x_reserve_offset = x_offset + SQUARE_LENGTH;

        Board {
            x_offset,
            y_offset,
            x_reserve_offset,
        }
    }
}

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

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq, Hash)]
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
#[derive(Debug, Component, Clone, Copy)]
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

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq)]
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

pub fn translate_transform(x: f32, y: f32, x_offset: f32, y_offset: f32, owner: &Player) -> Vec3 {
    match owner {
        Player::Challenging => Vec3::new(
            x * SQUARE_LENGTH + x_offset,
            y * SQUARE_LENGTH + y_offset,
            2.0,
        ),
        Player::Residing => Vec3::new(
            x * SQUARE_LENGTH + x_offset,
            y * SQUARE_LENGTH + y_offset,
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

pub fn is_path_clear(
    start: Position,
    end: Position,
    pieces: &[(Position, Player, Rank, PieceType)],
) -> bool {
    let polar_maker = |startx: f32, starty: f32, endx: f32, endy: f32| -> (f32, f32) {
        let dy = endy as f32 - starty as f32;
        let dx = endx as f32 - startx as f32;

        ((dy).atan2(dx), (dy.powi(2) + dx.powi(2)).sqrt())
    };

    let (trajectory_angle, trajectory_magnitude) =
        polar_maker(start.x as f32, start.y as f32, end.x as f32, end.y as f32);

    !pieces
        .iter()
        // do not include the piece itself in consideration
        .filter(|(p, _, _, _)| !(p.x == start.x && p.y == start.y))
        .any(|(piece, _, _, _)| {
            let (this_trajectory_angle, this_trajectory_magnitude) = polar_maker(
                start.x as f32,
                start.y as f32,
                piece.x as f32,
                piece.y as f32,
            );

            this_trajectory_angle == trajectory_angle
                && this_trajectory_magnitude < trajectory_magnitude
        })
}
