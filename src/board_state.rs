pub mod player;
pub mod piece;
pub mod tile;
pub mod stored_move;

pub use crate::board_state::player::Player;
pub use crate::board_state::piece::{ Piece, PieceType };
pub use crate::board_state::tile::Tile;
pub use crate::board_state::stored_move::StoredMove;

const WHITE: Player = Player::White;
const BLACK: Player = Player::Black;

const KING: PieceType = PieceType::King;
const QUEEN: PieceType = PieceType::Queen;
const ROOK: PieceType = PieceType::Rook;
const KNIGHT: PieceType = PieceType::Knight;
const BISHOP: PieceType = PieceType::Bishop;
const PAWN: PieceType = PieceType::Pawn;

#[derive(Debug)]
pub struct BoardState {
    player_turn: Player,
    fifty_move_rule_count: usize,
    board: [[Tile; 8]; 8],
}

// TODO track en passants and whether castling is legal for each player
impl BoardState {
    pub fn new() -> BoardState {
        BoardState {
            player_turn: Player::White,
            fifty_move_rule_count: 0,
            board: BoardState::place_pieces()
        }
    }

    pub fn get_pos_of_tile<'a>(&self, tile: &'a Tile) -> &'a (usize, usize) {
        &tile.get_pos()
    }

    pub fn get_tile_at_pos(&self, pos: (usize, usize)) -> &Tile {
        let (x, y) = pos;
        &self.board[x][y]
    }

    pub fn display_full_board(&self) -> String {
        let mut result = String::from("");

        for x in self.board.iter() {
            for y in x.iter() {
                result.push(y.symbol());
                result.push(' ');
            }
            result.pop(); // To remove extra space
            result.push('\n');
        }
        result.pop(); // To remove extra new line

        result
    }

    pub fn display_full_board_utf(&self) -> String {
        let mut result = String::from("");

        for x in self.board.iter() {
            for y in x.iter() {
                result.push(y.symbol_utf());
                result.push(' ');
            }
            result.pop(); // To remove extra space
            result.push('\n');
        }
        result.pop(); // To remove extra new line

        result
    }

    // TODO commented out player and seperated the lines just because the warning was annoying me
    pub fn move_piece(&mut self, 
        // player: Player, 
        planned_move: StoredMove) {
        let (start_y, start_x) = planned_move.start_pos;
        let (end_y, end_x) = planned_move.end_pos;
        let promotion = planned_move.promotion;

        // TODO ensure that move is within the bounds of the board
        // TODO ensure that a move is legal

        // move is legal if - 
        // player's turn
        // grabbing a piece in start
        // grabbing a piece of the players type in start
        // moving to a valid position per rules (knights, bishops, etc) in end
        // note! since dark chess, checking self is allowed
        // end is either a none or an enemy

        let start_tile = self.board[start_x][start_y];
        let start_piece = start_tile.get_piece();

        let end_tile = self.board[end_x][end_y];

        // TODO is there a way to not have to rewrite the Some(piece)
        // TODO should I really be dereferencing here?
        self.board[end_x][end_y] = match promotion {
            Some(piece) => Tile::new(Some(piece), (end_x, end_y)),
            None => Tile::new(*start_piece, planned_move.end_pos),
        };
        self.board[start_x][start_y] = Tile::new(None, (start_x, start_y));

        println!();
        println!("start piece: {:?}, end piece: {:?}", start_tile, end_tile);
        println!("move_piece()!");
        println!();
    }

    // TODO this function is super ugly, and doing a lot
    fn place_pieces() -> [[Tile; 8]; 8] {
        // Putting these declarations up top instead of in the array makes the code more readable, imo

        // There is a crate that allows me to input a closure to init the array,
        // but I am trying make this project without crates, so I can better learn rust

        // declaring pieces
        let w_king = Piece::new(KING, WHITE);
        let w_queen =Piece::new(QUEEN, WHITE);
        let w_rook = Piece::new(ROOK, WHITE);
        let w_knight = Piece::new(KNIGHT, WHITE);
        let w_bishop = Piece::new(BISHOP, WHITE);
        let w_pawn = Piece::new(PAWN, WHITE);

        let b_king = Piece::new(KING, BLACK);
        let b_queen = Piece::new(QUEEN, BLACK);
        let b_rook = Piece::new(ROOK, BLACK);
        let b_knight = Piece::new(KNIGHT, BLACK);
        let b_bishop = Piece::new(BISHOP, BLACK);
        let b_pawn = Piece::new(PAWN, BLACK);

        // declaring rows
        let row_8 = [
            Tile::new(Some(b_rook), (0,0)),
            Tile::new(Some(b_knight), (0,1)),
            Tile::new(Some(b_bishop), (0,2)),
            Tile::new(Some(b_queen), (0,3)),
            Tile::new(Some(b_king), (0,4)),
            Tile::new(Some(b_bishop), (0,5)),
            Tile::new(Some(b_knight), (0,6)),
            Tile::new(Some(b_rook), (0,7)),
        ];
        let row_7 = BoardState::init_consistent_row(1, Some(b_pawn));

        let row_6 = BoardState::init_consistent_row(2, None);
        let row_5 = BoardState::init_consistent_row(3, None);
        let row_4 = BoardState::init_consistent_row(4, None);
        let row_3 = BoardState::init_consistent_row(5, None);

        let row_2 = BoardState::init_consistent_row(6, Some(w_pawn));
        let row_1 = [
            Tile::new(Some(w_rook), (7,0)),
            Tile::new(Some(w_knight), (7,1)),
            Tile::new(Some(w_bishop), (7,2)),
            Tile::new(Some(w_queen), (7,3)),
            Tile::new(Some(w_king), (7,4)),
            Tile::new(Some(w_bishop), (7,5)),
            Tile::new(Some(w_knight), (7,6)),
            Tile::new(Some(w_rook), (7,7)),
        ];

        [ row_8, row_7, row_6, row_5, row_4, row_3, row_2, row_1 ]
    }
    
    fn init_consistent_row(row: usize, piece: Option<Piece>) -> [Tile; 8] {
        let mut row_arr = [Tile::new(piece, (0, 0)); 8];
        for (index, mut _val) in row_arr.iter_mut().enumerate() {
            *_val = Tile::new(piece, (row, index));
        }
        row_arr
    }
}

// TODO test suite?