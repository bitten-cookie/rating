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
    fn calculate(
        &self,
        player_one: Rating,
        player_two: Rating,
        result: GameResult,
    ) -> (Rating, Rating);
}
