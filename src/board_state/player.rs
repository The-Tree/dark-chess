#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Player {
    White,
    Black
}

// these tests are pretty trivial
// TODO remove?
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal() {
        assert_eq!(Player::White, Player::White);
    }

    #[test]
    fn not_equal() {
        assert_ne!(Player::White, Player::Black);
    }
}