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

// TODO tests!
impl MoveGeneration {
    // checks that a dir is in bounds
    fn within_bounds(dir: &Direction, tile: &Tile) -> bool {
        // uses add_dir instead of doing it quicker manually
        // for easier readability
        let (x, y) = MoveGeneration::add_dir(dir, tile);

        x < 8 && x >= 0 && y < 8 && y >= 0
    }

    // Adds a direction to a tile
    fn add_dir(dir: &Direction, tile: &Tile) -> (usize, usize) {
        let (x, y) = tile.get_pos();

        // TODO conversion in rust is (rightfully) harder than i expected
        // for now I am using the as keyword because I know theoretically that
        // this should never fail, but in the future I should implement some
        // kind of function that does error handling in some way
        let x = *x as i32;
        let y = *y as i32;

        let x = x + dir.right;
        let y = y + dir.up;

        let x = x as usize;
        let y = y as usize;

        (x, y)
    }

    // checks a direction, and determines if there is a piece there,
    // or if it is out of bounds
    fn check_dir(dir: &Direction, tile: &Tile, board_state: &BoardState) -> bool {
        // first checks move is within bounds
        if !MoveGeneration::within_bounds(dir, tile) {
            return false;
        }

        let (x, y) = MoveGeneration::add_dir(dir, tile);

        let target_tile = board_state.get_tile_at_pos((x, y));

        // TODO use of unwrap should probably be removed
        match *target_tile.get_piece() {
            Some(piece) => piece.get_player() != tile.get_piece().unwrap().get_player(),
            None => true
        }
    }

    // TODO this and knight are nearly identical
    // i think i want them to be seperate functions though,
    // i think it makes my code more immediately readable and understandable
    // might be able to seperate more into functions
    fn gen_moves_king(tile: &Tile, board_state: &BoardState) -> Vec<StoredMove> {
        // panicking if piece is not of piecetype king, since it should always be so
        if tile.get_piece().unwrap().get_piece_type() != &PieceType::King {
            panic!("Given the wrong piece type. Piece type given was: {:?}", tile.get_piece().unwrap().get_piece_type())
        }
        let mut moves = Vec::new();

        for potential_dir in KING_QUEEN_DIRS {
            if MoveGeneration::check_dir(&potential_dir, tile, board_state) {
                let (x, y) = MoveGeneration::add_dir(&potential_dir, tile);

                moves.push(StoredMove{ 
                    start_pos: *tile.get_pos(),
                    end_pos: (x, y),
                    promotion: None,
                });
            }
        }
        moves
    }

    // TODO this is very similar to bishop and rook
    // TODO important! unfinished, should not work :-(
    fn gen_moves_queen(tile: &Tile, board_state: &BoardState) -> Vec<StoredMove> {
        // panicking if piece is not of piecetype king, since it should always be so
        if tile.get_piece().unwrap().get_piece_type() != &PieceType::Queen {
            panic!("Given the wrong piece type. Piece type given was: {:?}", tile.get_piece().unwrap().get_piece_type())
        }
        let mut moves = Vec::new();

        for potential_dir in KING_QUEEN_DIRS {
            if MoveGeneration::check_dir(&potential_dir, tile, board_state) {
                let (x, y) = MoveGeneration::add_dir(&potential_dir, tile);

                moves.push(StoredMove{ 
                    start_pos: *tile.get_pos(),
                    end_pos: (x, y),
                    promotion: None,
                });

                // continue checking & adding this dir if there is no piece, 
                // add no moves if there is piece
                let mut continued_moves = match board_state.get_tile_at_pos().get_piece() {
                    Some(piece) => Vec::new(),
                    None => continue_checking_dir(dir, tile) // TODO function not implemented yet
                }

                moves.append(continued_moves);
            }
        }
        moves
    }

    fn gen_moves_knight(tile: &Tile, board_state: &BoardState) -> Vec<StoredMove> {
        // panicking if piece is not of piecetype knight, since it should always be so
        if tile.get_piece().unwrap().get_piece_type() != &PieceType::Knight {
            panic!("Given the wrong piece type. Piece type given was: {:?}", tile.get_piece().unwrap().get_piece_type())
        }
        let mut moves = Vec::new();

        for potential_dir in KNIGHT_DIRS {
            if MoveGeneration::check_dir(&potential_dir, tile, board_state) {
                let (x, y) = MoveGeneration::add_dir(&potential_dir, tile);

                moves.push(StoredMove{ 
                    start_pos: *curr_pos,
                    end_pos: (x, y),
                    promotion: None,
                });
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
        let tile = Tile::new(None, pos);

        assert!(!MoveGeneration::within_bounds(&dir, &tile));
    }

    #[test]
    fn within_bounds_y_too_large() {
        let dir = Direction{ right: 0, up: 1 };
        let pos = (4, 7);
        let tile = Tile::new(None, pos);

        assert!(!MoveGeneration::within_bounds(&dir, &tile));
    }

    #[test]
    fn within_bounds_x_too_small() {
        let dir = Direction{ right: -1, up: 0 };
        let pos = (0, 4);
        let tile = Tile::new(None, pos);

        assert!(!MoveGeneration::within_bounds(&dir, &tile));
    }

    #[test]
    fn within_bounds_y_too_small() {
        let dir = Direction{ right: 0, up: -1 };
        let pos = (4, 0);
        let tile = Tile::new(None, pos);

        assert!(!MoveGeneration::within_bounds(&dir, &tile));
    }

    #[test]
    fn within_bounds_x_y_big() {
        let dir = Direction{ up: 1, right: 1 };
        let pos = (7, 7);
        let tile = Tile::new(None, pos);

        assert!(!MoveGeneration::within_bounds(&dir, &tile));
    }

    #[test]
    fn within_bounds_x_y_small() {
        let dir = Direction{ up: -1, right: -1 };
        let pos = (0, 0);
        let tile = Tile::new(None, pos);

        assert!(!MoveGeneration::within_bounds(&dir, &tile));
    }  
    
    #[test]
    fn within_bounds_x_y_big_small() {
        let dir = Direction{ right: 1, up: -1 };
        let pos = (7, 0);
        let tile = Tile::new(None, pos);

        assert!(!MoveGeneration::within_bounds(&dir, &tile));
    } 

    #[test]
    fn within_bounds_x_y_small_big() {
        let dir = Direction{ right: -1, up: 1 };
        let pos = (0, 7);
        let tile = Tile::new(None, pos);

        assert!(!MoveGeneration::within_bounds(&dir, &tile));
    } 

    #[test]
    fn within_bounds_x_close_big() {
        let dir = Direction{ right: 1, up: 0 };
        let pos = (6, 4);
        let tile = Tile::new(None, pos);

        assert!(MoveGeneration::within_bounds(&dir, &tile));
    } 

    #[test]
    fn within_bounds_x_close_small() {
        let dir = Direction{ up: 0, right: 0 };
        let pos = (0, 0);
        let tile = Tile::new(None, pos);

        assert!(MoveGeneration::within_bounds(&dir, &tile));
    } 

    #[test]
    fn within_bounds_y_close_big() {
        let dir = Direction{ up: 0, right: 0 };
        let pos = (0, 7);
        let tile = Tile::new(None, pos);

        assert!(MoveGeneration::within_bounds(&dir, &tile));
    } 

    #[test]
    fn within_bounds_y_close_small() {
        let dir = Direction{ right: 0, up: -1 };
        let pos = (0, 1);
        let tile = Tile::new(None, pos);

        assert!(MoveGeneration::within_bounds(&dir, &tile));
    } 

    #[test]
    fn within_bounds_x_y_middle() {
        let dir = Direction{ up: 1, right: 1 };
        let pos = (4, 4);
        let tile = Tile::new(None, pos);

        assert!(MoveGeneration::within_bounds(&dir, &tile));
    } 
}