use crate::{CalculateRating, GameResult, Rating};
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use std::ops::Sub;

/// A classical Elo calculation
///
/// This calculation is used for head-to-head games, when two players
/// faces each other.
///
/// The `k`, k-factor, determines how strongly a result affects the rating change.
/// The higher the value, the more drastic the difference is.
///
/// This calculation method is used in a variety of games, with chess being
/// the most well-known game that uses this system.
#[derive(Clone, Debug)]
pub struct EloStrategy {
    k: Decimal,
}

impl EloStrategy {
    /// Builds an Elo calculator
    ///
    /// # Arguments
    /// * `k` - the k-factor to be used for the calculation
    pub fn new(k: u32) -> Self {
        Self {
            k: Decimal::from(k),
        }
    }
}

impl CalculateRating for EloStrategy {
    fn calculate(
        &self,
        player_one: Rating,
        player_two: Rating,
        game_result: GameResult,
    ) -> (Rating, Rating) {
        let expected_a = expected_result(*player_one, *player_two);
        let expected_b = expected_result(*player_two, *player_one);

        let (score_a, score_b) = match game_result {
            GameResult::Win => (dec!(1.0), dec!(0.0)),
            GameResult::Loss => (dec!(0.0), dec!(1.0)),
            GameResult::Draw => (dec!(0.5), dec!(0.5)),
        };

        let new_player_one_rating = player_one + self.k * (score_a - expected_a);
        let new_player_two_rating = player_two + self.k * (score_b - expected_b);

        (new_player_one_rating, new_player_two_rating)
    }
}

fn expected_result(player: Decimal, opponent: Decimal) -> Decimal {
    let diff = opponent.sub(player);
    let p = dec!(1.0) + dec!(10).powd((diff) / dec!(400.0));

    dec!(1.0) / p
}

#[cfg(test)]
mod tests {
    use super::GameResult::*;
    use super::*;
    use test_case::test_case;

    fn calculates_elo(
        player_1_rating: i32,
        player_2_rating: i32,
        k: u32,
        game_result: GameResult,
    ) -> (Rating, Rating) {
        let elo = EloStrategy::new(k);

        elo.calculate(player_1_rating.into(), player_2_rating.into(), game_result)
    }

    #[test_case(1000, 2000, 16, Win => (1015.9, 1984.1))]
    #[test_case(2600, 2300, 16, Win => (2602.4, 2297.6))]
    #[test_case(2600, 2300, 16, Loss => (2586.4, 2313.6))]
    #[test_case(2600, 2300, 16, Draw => (2594.4, 2305.6))]
    #[test_case(2600, 2300, 40, Win => (2606.0, 2294.0))]
    #[test_case(2600, 2300, 40, Loss => (2566.0, 2334.0))]
    #[test_case(2600, 2300, 40, Draw => (2586.0, 2314.0))]
    fn calculates_elo_correctly(
        player_1_rating: i32,
        player_2_rating: i32,
        k: u32,
        game_result: GameResult,
    ) -> (f64, f64) {
        let (new_player_1, new_player_2) =
            calculates_elo(player_1_rating, player_2_rating, k, game_result);
        (new_player_1.value(), new_player_2.value())
    }

    #[test_case(1000, 2000, 16, Win => (1016, 1984))]
    #[test_case(2600, 2300, 16, Win => (2602, 2298))]
    #[test_case(2600, 2300, 16, Loss => (2586, 2314))]
    #[test_case(2600, 2300, 16, Draw => (2594, 2306))]
    #[test_case(2600, 2300, 40, Win => (2606, 2294))]
    #[test_case(2600, 2300, 40, Loss => (2566, 2334))]
    #[test_case(2600, 2300, 40, Draw => (2586, 2314))]
    fn calculates_elo_correctly_when_rounded(
        player_1_rating: i32,
        player_2_rating: i32,
        k: u32,
        game_result: GameResult,
    ) -> (i32, i32) {
        let (new_player_1, new_player_2) =
            calculates_elo(player_1_rating, player_2_rating, k, game_result);
        (new_player_1.round_to_i32(), new_player_2.round_to_i32())
    }

    #[test_case(2600, 2300, 16, vec![Win] => (2602.4, 2297.6))]
    #[test_case(2600, 2300, 16, vec![Win, Win] => (2604.8, 2295.2))]
    #[test_case(2600, 2300, 16, vec![Win, Win, Loss] => (2591.1, 2308.9))]
    #[test_case(2600, 2300, 16, vec![Win, Win, Loss, Loss] => (2577.7, 2322.3))]
    #[test_case(2600, 2300, 16, vec![Win, Win, Loss, Loss, Loss] => (2564.7, 2335.3))]
    #[test_case(2600, 2300, 16, vec![Win, Win, Loss, Loss, Loss, Loss] => (2552.1, 2347.9))]
    #[test_case(2600, 2300, 16, vec![Win, Win, Loss, Loss, Loss, Loss, Draw] => (2547.9, 2352.1))]
    fn calculates_multiple_correctly(
        player_1_rating: i32,
        player_2_rating: i32,
        k: u32,
        game_result: Vec<GameResult>,
    ) -> (f64, f64) {
        let elo = EloStrategy::new(k);

        let (new_player_1, new_player_2) =
            elo.calculate_multiple(player_1_rating.into(), player_2_rating.into(), game_result);
        (new_player_1.value(), new_player_2.value())
    }
}
