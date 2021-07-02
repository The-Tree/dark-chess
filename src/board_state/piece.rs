pub use crate::board_state::player::Player;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Piece {
    piece_type: PieceType,
    player: Player,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn
}

impl Piece {
    pub fn new(piece_type: PieceType, player: Player) -> Piece {
        Piece { piece_type, player }
    }

    // TODO both symbol functions - should these return chars or &chars?
    // I am not sure, i lean towards I should use &char?
    // TODO is there a way to not have two functions, one for symbol and one for symbol_utf?
    // these are the normal ascii values for each piece hardcoded. no differentiation between players
    pub fn symbol(&self) -> char {
        match self.piece_type {
            PieceType::King => 'K',
            PieceType::Queen => 'Q',
            PieceType::Rook => 'R',
            PieceType::Knight => 'N',
            PieceType::Bishop => 'B',
            PieceType::Pawn => 'p',
        }
    }

    // TODO is there a less verbose way to match this?
    // these are the utf-8 values for each piece harcoded
    // this is arguably harder to read
    pub fn symbol_utf(&self) -> char {
        if self.player == Player::White {
            return match self.piece_type {
                PieceType::King => '\u{2654}',
                PieceType::Queen => '\u{2655}',
                PieceType::Rook => '\u{2656}',
                PieceType::Knight => '\u{2657}',
                PieceType::Bishop => '\u{2658}',
                PieceType::Pawn => '\u{2659}',
            }
        }
        else {
            return match self.piece_type {
                PieceType::King => '\u{265A}',
                PieceType::Queen => '\u{265B}',
                PieceType::Rook => '\u{265C}',
                PieceType::Knight => '\u{265D}',
                PieceType::Bishop => '\u{265E}',
                PieceType::Pawn => '\u{265F}',
            }
        }
    }

    pub fn get_piece_type(&self) -> &PieceType {
        &self.piece_type
    }

    pub fn get_player(&self) -> &Player {
        &self.player
    }
}


// TODO these tests are also largely 
// relatively complete
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq_pieces() {
        let piece1 = Piece::new(PieceType::King, Player::White);
        let piece2 = Piece::new(PieceType::King, Player::White);

        assert_eq!(piece1, piece2);
    }

    #[test]
    fn ne_different_pieces() {
        let piece1 = Piece::new(PieceType::King, Player::White);
        let piece2 = Piece::new(PieceType::Queen, Player::White);

        assert_ne!(piece1, piece2);
    }

    #[test]
    fn ne_different_players() {
        let piece1 = Piece::new(PieceType::King, Player::White);
        let piece2 = Piece::new(PieceType::King, Player::Black);

        assert_ne!(piece1, piece2);
    }

    #[test]
    fn get_piece_type() {
        let piece = Piece::new(PieceType::King, Player::White);

        assert_eq!(piece.get_piece_type(), &PieceType::King);
    }

    #[test]
    fn get_player() {
        let piece = Piece::new(PieceType::King, Player::White);

        assert_eq!(piece.get_player(), &Player::White);
    }

    #[test]
    fn symbol_k_white() {
        let piece = Piece::new(PieceType::King, Player::White);

        assert_eq!(piece.symbol(), 'K');
    }

    #[test]
    fn symbol_k_black() {
        let piece = Piece::new(PieceType::Queen, Player::Black);

        assert_eq!(piece.symbol(), 'Q');
    }

    #[test]
    fn symbol_q() {
        let piece = Piece::new(PieceType::Queen, Player::White);

        assert_eq!(piece.symbol(), 'Q');
    }

    #[test]
    fn symbol_k_white_utf() {
        let piece = Piece::new(PieceType::King, Player::White);

        assert_eq!(piece.symbol_utf(), '\u{2654}');
    }

    #[test]
    fn symbol_k_black_utf() {
        let piece = Piece::new(PieceType::King, Player::Black);

        assert_eq!(piece.symbol_utf(), '\u{265A}');
    }
}