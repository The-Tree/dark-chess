pub mod board_state;
pub mod move_generation;

// TODO - remove #[derive()] if possible (likely will be possible for debug)

// TODO implement a list of previous moves?

// TODO next thing I should do is just consider how my code should be organized

// these are the two overarching modules in the code:

// there is the board state, which contains the board and some metadata about
// whos turn it is, the 50 move draw, en passants
// anything related to board or board information STATICALLY is here
// this includes the board of Tiles
// the Tiles, which contain a piece or do not
// the Tiles should likely also have the position they are at.
// the Pieces, which have a player attached
// the Player, attached to pieces and whos turn it is

// there is the move generator, which is able to generate legal moves given the board state
// and generate legal moves for a specific tile given the board state and the tile, or the pos
// all this can do is generate moves. it sits without any information itself, it just has static functions

// TODO To this end, I will go through from player to board_state first, adding tests for each function and rethinking
// each function. i think all the functionality is more or less there, but i want to be able to hide things if possible
// and also maybe even optimize a bit