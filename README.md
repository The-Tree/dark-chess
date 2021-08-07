# Dark Chess
This is a simple implementation of Dark Chess in Rust. Dark Chess is a chess variant where a player has incomplete information.
Specifically, in Dark Chess a player can only view tiles where one of their pieces are, or tiles where they can move. One consequence
of this, for example, is that a player can move their King into check since they may not necessarily be able to see that their King would
be attacked in a new position.

This implementation is not yet complete. The current goal is to bring it to a point where it is properly representing a 
game of Dark Chess. Once that is complete, I may create a server and client that together can be used to actually play the game.
Here are a list of features:
- [x] Representing board
  - [x] Representing pieces
  - [x] Representing players
  - [x] Representing tiles
  - [x] Representing moves
- [x] Move generation
  - [x] Piece move generation
    - [x] King move generation
    - [x] Queen move generation
    - [x] Rook move generation
    - [x] Knight move generation
    - [x] Bishop move generation
    - [x] Pawn move generation
  - [x] Player move generation
- [ ] More specific rules
  - [ ] Castling
  - [ ] En passant
  - [ ] 50 move draw
  - [ ] Three-fold repetition (?)
  - [ ] Pawn promotion
  - [ ] Pawn two step on first move
- [ ] Exhaustive tests  
  - Right now, some completed functionality is exhaustively tested with unit tests, and other supposedly completed functionality is not

Here are some more features that I think would be nice:
- [ ] Playable among friends
  - [ ] Simple server which hosts games, rooms, players, spectators, and maybe even a simple chat
  - [ ] Simple front end client to play
