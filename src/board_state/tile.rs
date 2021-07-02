
pub use crate::board_state::piece::Piece;

#[derive(Debug, Copy, Clone)]
pub struct Tile {
    piece: Option<Piece>,
    pos: (usize, usize),
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        match (&self.piece, &other.piece) {
            (&Some(ref lhs), &Some(ref rhs)) if lhs == rhs => true,
            (&None, &None) => true,
            _ => false
        }
    }
}

impl Tile {
    pub fn new(piece: Option<Piece>, pos: (usize, usize)) -> Tile {
        Tile { piece, pos }
    }

    pub fn get_piece(&self) -> &Option<Piece> {
        &self.piece
    }

    pub fn get_pos(&self) -> &(usize, usize) {
        &self.pos
    }

    pub fn symbol(&self) -> char {
        match self.piece {
            Some(piece) => piece.symbol(),
            None => '.',
        }
    }

    pub fn symbol_utf(&self) -> char {
        match self.piece {
            Some(piece) => piece.symbol_utf(),
            None => '.',
        }
    }
}