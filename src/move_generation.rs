// use crate::board_state::{BoardState, Tile, StoredMove};

// pub struct MoveGeneration {}

// impl MoveGeneration {
//     // TODO - generate legal moves for a given player
//     // note, choosing a player is relevant since we want to know what they can see as well
//     // -----------

//     pub fn generate_moves_for_tile(board_state: &BoardState, tile: &Tile) -> Vec<StoredMove> {

//         // find pos function, given tile. returns an Option<(x, y)>

//         // find tile function, given pos. returns an Option<Tile>

//         let piece = tile.get_piece();

//         let pos = board_state.get_pos_of_tile(tile);

//         // match piece {
//         //     Some(PieceType::King) => MoveGeneration::get_legal_moves_king(board_state, tile),
//         //     Some(PieceType::Queen) => MoveGeneration::get_legal_moves_queen(board_state, tile),
//         //     Some(PieceType::Rook) => MoveGeneration::get_legal_moves_rook(board_state, tile),
//         //     Some(PieceType::Knight) => MoveGeneration::get_legal_moves_knight(board_state, tile),
//         //     Some(PieceType::Bishop) => MoveGeneration::get_legal_moves_bishop(board_state, tile),
//         //     Some(PieceType::Pawn) => MoveGeneration::get_legal_moves_pawn(board_state, tile),
//         //     None => Vec::new(),
//         // }

//         // temporarily here TODO redo this
//         Vec::new()
//     }

    
//     // TODO is there a way to do this with fewer functions?
//     fn get_legal_moves_king(board_state: &BoardState, tile: &Tile) -> Vec<StoredMove> {
//         // TODO
//         Vec::new()
//     }

//     fn get_legal_moves_queen(board_state: &BoardState, tile: &Tile) -> Vec<StoredMove> {
//         // TODO
//         Vec::new()
//     }

//     fn get_legal_moves_rook(board_state: &BoardState, tile: &Tile) -> Vec<StoredMove> {
//         // TODO
//         Vec::new()
//     }

//     fn get_legal_moves_knight(board_state: &BoardState, tile: &Tile) -> Vec<StoredMove> {
//         // TODO
//         Vec::new()
//     }

//     fn get_legal_moves_bishop(board_state: &BoardState, tile: &Tile) -> Vec<StoredMove> {
//         // TODO
//         Vec::new()
//     }

//     fn get_legal_moves_pawn(board_state: &BoardState, tile: &Tile) -> Vec<StoredMove> {
//         // TODO
//         Vec::new()
//     }
// }