use crate::{GameResult, Rating, ScoreStrategy};
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use std::ops::Sub;

pub struct Elo {
    pub k: i32,
}

impl ScoreStrategy for Elo {
    fn calculate(
        &self,
        player_one: Rating,
        player_two: Rating,
        game_result: GameResult,
    ) -> (Rating, Rating) {
        let player_1_rating = Decimal::from_f64(*player_one).unwrap();
        let player_2_rating = Decimal::from_f64(*player_two).unwrap();

        let expected_a = expected_result(player_1_rating, player_2_rating);
        let expected_b = expected_result(player_2_rating, player_1_rating);

        let (score_a, score_b) = match game_result {
            GameResult::Win => (dec!(1.0), dec!(0.0)),
            GameResult::Loss => (dec!(0.0), dec!(1.0)),
            GameResult::Draw => (dec!(0.5), dec!(0.5)),
        };

        let k_decimal = Decimal::from(self.k);

        let new_player_one_rating = player_1_rating + k_decimal * (score_a - expected_a);
        let new_player_two_rating = player_2_rating + k_decimal * (score_b - expected_b);

        (
            Rating::new(
                new_player_one_rating
                    .round_dp_with_strategy(1, RoundingStrategy::MidpointAwayFromZero)
                    .to_f64()
                    .unwrap(),
            ),
            Rating::new(
                new_player_two_rating
                    .round_dp_with_strategy(1, RoundingStrategy::MidpointAwayFromZero)
                    .to_f64()
                    .unwrap(),
            ),
        )
    }
}

fn expected_result(player: Decimal, opponent: Decimal) -> Decimal {
    let diff = opponent.sub(player);
    let p = dec!(1.0) + dec!(10).powd((diff) / dec!(400.0));

    dec!(1.0) / p
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    fn calculates_elo(
        player_1_rating: i32,
        player_2_rating: i32,
        k: i32,
        game_result: GameResult,
    ) -> (Rating, Rating) {
        let elo = Elo { k };

        elo.calculate(player_1_rating.into(), player_2_rating.into(), game_result)
    }

    #[test_case(2600, 2300, 16, GameResult::Win => (2602.4, 2297.6))]
    #[test_case(2600, 2300, 16, GameResult::Loss => (2586.4, 2313.6))]
    #[test_case(2600, 2300, 16, GameResult::Draw => (2594.4, 2305.6))]
    #[test_case(2600, 2300, 40, GameResult::Win => (2606.0, 2294.0))]
    #[test_case(2600, 2300, 40, GameResult::Loss => (2566.0, 2334.0))]
    #[test_case(2600, 2300, 40, GameResult::Draw => (2586.0, 2314.0))]
    fn calculates_elo_correctly(
        player_1_rating: i32,
        player_2_rating: i32,
        k: i32,
        game_result: GameResult,
    ) -> (f64, f64) {
        let (new_player_1, new_player_2) =
            calculates_elo(player_1_rating, player_2_rating, k, game_result);
        (*new_player_1, *new_player_2)
    }

    #[test_case(2600, 2300, 16, GameResult::Win => (2602, 2298))]
    #[test_case(2600, 2300, 16, GameResult::Loss => (2586, 2314))]
    #[test_case(2600, 2300, 16, GameResult::Draw => (2594, 2306))]
    #[test_case(2600, 2300, 40, GameResult::Win => (2606, 2294))]
    #[test_case(2600, 2300, 40, GameResult::Loss => (2566, 2334))]
    #[test_case(2600, 2300, 40, GameResult::Draw => (2586, 2314))]
    fn calculates_elo_correctly_when_rounded(
        player_1_rating: i32,
        player_2_rating: i32,
        k: i32,
        game_result: GameResult,
    ) -> (i32, i32) {
        let (new_player_1, new_player_2) =
            calculates_elo(player_1_rating, player_2_rating, k, game_result);
        (new_player_1.rounded(), new_player_2.rounded())
    }
}
