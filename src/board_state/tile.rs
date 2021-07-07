
pub use crate::board_state::piece::Piece;

#[derive(Debug, Copy, Clone)]
pub struct Tile {
    piece: Option<Piece>,
    pos: (usize, usize),
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        if self.pos != other.pos {
            return false
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board_state::{PieceType, Player};

    #[test]
    fn equal_none() {
        let tile1 = Tile::new(None, (0, 0));
        let tile2 = Tile::new(None, (0, 0));

        assert_eq!(tile1, tile2);
    }

    #[test]
    fn equal_w_king() {
        let tile1 = Tile::new(Some(Piece::new(PieceType::King, Player::White)), (0, 0));
        let tile2 = Tile::new(Some(Piece::new(PieceType::King, Player::White)), (0, 0));

        assert_eq!(tile1, tile2);
    }

    #[test]
    fn not_equal_diff_player() {
        let tile1 = Tile::new(Some(Piece::new(PieceType::King, Player::Black)), (0, 0));
        let tile2 = Tile::new(Some(Piece::new(PieceType::King, Player::White)), (0, 0));

        assert_ne!(tile1, tile2);
    }

    #[test]
    fn not_equal_diff_piece() {
        let tile1 = Tile::new(None, (0, 0));
        let tile2 = Tile::new(Some(Piece::new(PieceType::King, Player::White)), (0, 0));

        assert_ne!(tile1, tile2);
    }

    #[test]
    fn not_equal_diff_pos() {
        let tile1 = Tile::new(None, (0, 0));
        let tile2 = Tile::new(None, (0, 1));

        assert_ne!(tile1, tile2);
    }

    #[test]
    fn get_piece_none() {
        let tile = Tile::new(None, (0, 0));

        assert_eq!(tile.get_piece(), &None);
    }

    #[test]
    fn get_piece_w_king() {
        let tile = Tile::new(Some(Piece::new(PieceType::King, Player::White)), (0, 0));

        assert_eq!(tile.get_piece(), &Some(Piece::new(PieceType::King, Player::White)));
    }

    #[test]
    fn get_pos() {
        let tile = Tile::new(None, (0, 0));

        assert_eq!(tile.get_pos(), &(0, 0));
    }

    #[test]
    fn symbol_w_king() {
        let tile = Tile::new(Some(Piece::new(PieceType::King, Player::White)), (0, 0));

        assert_eq!(tile.symbol(), 'K');
    }

    #[test]
    fn symbol_none() {
        let tile = Tile::new(None, (0,0));

        assert_eq!(tile.symbol(), '.');
    }

    #[test]
    fn symbol_utf_w_king() {
        let tile = Tile::new(Some(Piece::new(PieceType::King, Player::White)), (0,0));

        assert_eq!(tile.symbol_utf(), '\u{2654}');
    }

    #[test]
    fn symbol_utf_none() {
        let tile = Tile::new(None, (0,0));

        assert_eq!(tile.symbol_utf(), '.');
    }
}