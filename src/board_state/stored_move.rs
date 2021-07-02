use crate::board_state::Piece;

// Since there is no mutability here, it is fine to have public data
// We only make new Stored moves and get the data from them
pub struct StoredMove {
    pub start_pos: (usize, usize),
    pub end_pos: (usize, usize),
    pub promotion: Option<Piece>,
}