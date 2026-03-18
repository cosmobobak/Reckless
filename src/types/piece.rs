use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use super::Color;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum Piece {
    WhitePawn,
    BlackPawn,
    WhiteKnight,
    BlackKnight,
    WhiteBishop,
    BlackBishop,
    WhiteRook,
    BlackRook,
    WhiteQueen,
    BlackQueen,
    WhiteKing,
    BlackKing,
}

impl Piece {
    pub const NUM: usize = 12;

    pub const ALL: [Piece; Self::NUM] = [
        Piece::WhitePawn,
        Piece::BlackPawn,
        Piece::WhiteKnight,
        Piece::BlackKnight,
        Piece::WhiteBishop,
        Piece::BlackBishop,
        Piece::WhiteRook,
        Piece::BlackRook,
        Piece::WhiteQueen,
        Piece::BlackQueen,
        Piece::WhiteKing,
        Piece::BlackKing,
    ];

    pub const fn new(color: Color, piece_type: PieceType) -> Self {
        unsafe { std::mem::transmute(((piece_type as u8) << 1) | color as u8) }
    }

    pub const fn from_index(index: usize) -> Self {
        debug_assert!(index < Self::NUM);

        unsafe { std::mem::transmute(index as u8) }
    }

    pub const fn piece_color(self) -> Color {
        unsafe { std::mem::transmute((self as u8) & 1) }
    }

    pub const fn piece_type(self) -> PieceType {
        unsafe { std::mem::transmute((self as u8) >> 1) }
    }
}

impl TryFrom<char> for Piece {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let index = "PpNnBbRrQqKk".find(value).ok_or(())?;
        Ok(Self::from_index(index))
    }
}

impl From<Piece> for char {
    fn from(piece: Piece) -> char {
        "PpNnBbRrQqKk".chars().nth(piece as usize).unwrap()
    }
}

impl<T> Index<Piece> for [T; Piece::NUM] {
    type Output = T;

    fn index(&self, index: Piece) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T> IndexMut<Piece> for [T; Piece::NUM] {
    fn index_mut(&mut self, index: Piece) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
    pub const NUM: usize = 6;

    pub const fn new(value: usize) -> Self {
        unsafe { std::mem::transmute(value as u8) }
    }
}

impl<T> Index<PieceType> for [T; PieceType::NUM] {
    type Output = T;

    fn index(&self, index: PieceType) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T> IndexMut<PieceType> for [T; PieceType::NUM] {
    fn index_mut(&mut self, index: PieceType) -> &mut Self::Output {
        &mut self[index as usize]
    }
}
