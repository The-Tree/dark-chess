use crate::board_state::{ BoardState, Tile, StoredMove, PieceType };

// this is a struct to make parsing all possible moves easier 
struct Direction {
    pub up: i32,
    pub right: i32,
}

const UP: Direction = Direction{ up: 1, right: 0 };
const UP_RIGHT: Direction = Direction{ up: 1, right: 1 };
const RIGHT: Direction = Direction{ up: 0, right: 1 };
const DOWN_RIGHT: Direction = Direction{ up: -1, right: 1 };
const DOWN: Direction = Direction{ up: -1, right: 0 };
const DOWN_LEFT: Direction = Direction{ up: -1, right: -1 };
const LEFT: Direction = Direction{ up: 0, right: -1 };
const UP_LEFT: Direction = Direction{ up: 1, right: -1 };

const KING_QUEEN_DIRS: [Direction; 8] = [UP, UP_RIGHT, RIGHT, DOWN_RIGHT, DOWN, DOWN_LEFT, LEFT, UP_LEFT];
const BISHOP_DIRS: [Direction; 4] = [UP_RIGHT, DOWN_RIGHT, DOWN_LEFT, UP_LEFT];
const ROOK_DIRS: [Direction; 4] = [UP, RIGHT, DOWN, LEFT];

const KNIGHT_DIRS: [Direction; 8] = [
    Direction{up: 2, right: 1},
    Direction{up: 1, right: 2},
    Direction{up: -1, right: 2},
    Direction{up: -2, right: 1},
    Direction{up: -2, right: -1},
    Direction{up: -1, right: -2},
    Direction{up: 1, right: -2},
    Direction{up: 2, right: -1}
];

const B_PAWN_DIRS: [Direction; 3] = [UP, UP_RIGHT, UP_LEFT];

const W_PAWN_DIRS: [Direction; 3] = [DOWN, DOWN_RIGHT, DOWN_LEFT];

pub struct MoveGeneration {}

impl MoveGeneration {
    fn within_bounds(dir: &Direction, pos: &(usize, usize)) -> bool {
        let (x, y) = pos;

        // TODO conversion in rust is (rightfully) harder than i expected
        // for now I am using the as keyword because I know theoretically that
        // this should never fail, but in the future I should implement some
        // kind of function that does error handling in some way
        let x = *x as i32;
        let y = *y as i32;

        x + dir.right < 8 && x + dir.right >= 0 && y + dir.up < 8 && y + dir.up >= 0
    }

    // TODO currently assumes is correct piece
    fn gen_moves_king(tile: &Tile, board_state: &BoardState) -> Vec<StoredMove> {
        // panicking if piece is not of piecetype king, since it should always be so
        if tile.get_piece().unwrap().get_piece_type() != &PieceType::King {
            panic!("Given the wrong piece type. Piece type given was: {:?}", tile.get_piece().unwrap().get_piece_type())
        }

        let curr_pos = tile.get_pos();

        let moves = Vec::new();

        for potential_dir in KING_QUEEN_DIRS {
            if MoveGeneration::within_bounds(&potential_dir, curr_pos) {
                println!(":)");
            }
        }

        moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board_state::{PieceType, Player};

    #[test]
    fn within_bounds_x_too_large() {
        let dir = Direction{ right: 1, up: 0 };
        let pos = (7, 4);

        assert!(!MoveGeneration::within_bounds(&dir, &pos));
    }

    #[test]
    fn within_bounds_y_too_large() {
        let dir = Direction{ right: 0, up: 1 };
        let pos = (4, 7);

        assert!(!MoveGeneration::within_bounds(&dir, &pos));
    }

    #[test]
    fn within_bounds_x_too_small() {
        let dir = Direction{ right: -1, up: 0 };
        let pos = (0, 4);

        assert!(!MoveGeneration::within_bounds(&dir, &pos));
    }

    #[test]
    fn within_bounds_y_too_small() {
        let dir = Direction{ right: 0, up: -1 };
        let pos = (4, 0);

        assert!(!MoveGeneration::within_bounds(&dir, &pos));
    }

    #[test]
    fn within_bounds_x_y_big() {
        let dir = Direction{ up: 1, right: 1 };
        let pos = (7, 7);

        assert!(!MoveGeneration::within_bounds(&dir, &pos));
    }

    #[test]
    fn within_bounds_x_y_small() {
        let dir = Direction{ up: -1, right: -1 };
        let pos = (0, 0);

        assert!(!MoveGeneration::within_bounds(&dir, &pos));
    }  
    
    #[test]
    fn within_bounds_x_y_big_small() {
        let dir = Direction{ right: 1, up: -1 };
        let pos = (7, 0);

        assert!(!MoveGeneration::within_bounds(&dir, &pos));
    } 

    #[test]
    fn within_bounds_x_y_small_big() {
        let dir = Direction{ right: -1, up: 1 };
        let pos = (0, 7);

        assert!(!MoveGeneration::within_bounds(&dir, &pos));
    } 

    #[test]
    fn within_bounds_x_close_big() {
        let dir = Direction{ right: 1, up: 0 };
        let pos = (6, 4);

        assert!(MoveGeneration::within_bounds(&dir, &pos));
    } 

    #[test]
    fn within_bounds_x_close_small() {
        let dir = Direction{ up: 0, right: 0 };
        let pos = (0, 0);

        assert!(MoveGeneration::within_bounds(&dir, &pos));
    } 

    #[test]
    fn within_bounds_y_close_big() {
        let dir = Direction{ up: 0, right: 0 };
        let pos = (0, 7);

        assert!(MoveGeneration::within_bounds(&dir, &pos));
    } 

    #[test]
    fn within_bounds_y_close_small() {
        let dir = Direction{ right: 0, up: -1 };
        let pos = (0, 1);

        assert!(MoveGeneration::within_bounds(&dir, &pos));
    } 

    #[test]
    fn within_bounds_x_y_middle() {
        let dir = Direction{ up: 1, right: 1 };
        let pos = (4, 4);

        assert!(MoveGeneration::within_bounds(&dir, &pos));
    } 
}