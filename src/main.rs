use dark_chess_server::board_state::BoardState;
// use dark_chess_server::board_state::Player;
// use dark_chess_server::board_state::Tile;
// use dark_chess_server::board_state::{ Piece, PieceType };
// use dark_chess_server::board_state::StoredMove;

// use dark_chess_server::move_generation::MoveGeneration;

fn main() {
    let board = BoardState::new();

    // println!("Board before move:\n{}", board.display_full_board_utf());

    // board.move_piece(Player::White, StoredMove {
    //     start_pos: (4, 6),
    //     end_pos: (4, 4),
    //     promotion: None
    // } );

    // println!("Board after move:\n{}", board.display_full_board_utf());

    println!("Move generator for board:\n{}", board.display_full_board_utf());

    println!("TEST:\n {:?}", board);

    // let tile = board.get_tile_at_pos((6,4));

    // MoveGeneration::generate_moves_for_tile(&board, tile);
}

