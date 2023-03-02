pub use crate::rating::Rating;
pub use elo::Elo;

mod elo;
mod rating;

#[derive(Debug, Copy, Clone)]
pub enum GameResult {
    Win,
    Loss,
    Draw,
}

pub trait ScoreStrategy {
    /// Calculates the resulting Rating of two players involved in a game
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

    /// Calculates the final resulting Rating of two players involved in multiple games
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
