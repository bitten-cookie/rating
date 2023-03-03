// extra lints
#![warn(unsafe_code)]
#![warn(clippy::all, clippy::panic, clippy::map_unwrap_or, clippy::unwrap_used)]
// Ignore unwraps in tests
#![cfg_attr(test, allow(clippy::unwrap_used))]

mod elo;
mod rating;

pub use crate::elo::EloStrategy;
pub use crate::rating::Rating;

#[derive(Debug, Copy, Clone)]
pub enum GameResult {
    Win,
    Loss,
    Draw,
}

pub trait CalculateRating {
    /// Calculates the resulting `Rating` of two players involved in a game
    ///
    /// # Arguments
    ///
    /// * `result` - the result of the game from the `player` perspective
    ///
    /// # Return
    ///
    /// * `(player, opponent)` - a tuple with the new ratings
    ///
    fn calculate(&self, player: Rating, opponent: Rating, result: GameResult) -> (Rating, Rating);

    /// Calculates the final resulting `Rating` of two players involved in multiple games
    ///
    /// # Return
    ///
    /// * `(player, opponent)` - a tuple with the new ratings
    ///
    fn calculate_multiple(
        &self,
        player: Rating,
        opponent: Rating,
        results: impl IntoIterator<Item = GameResult>,
    ) -> (Rating, Rating) {
        results
            .into_iter()
            .fold((player, opponent), |(first, second), result| {
                self.calculate(first, second, result)
            })
    }
}
