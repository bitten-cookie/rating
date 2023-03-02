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
        let expected_a = expected_result(*player_one, *player_two);
        let expected_b = expected_result(*player_two, *player_one);

        let (score_a, score_b) = match game_result {
            GameResult::Win => (dec!(1.0), dec!(0.0)),
            GameResult::Loss => (dec!(0.0), dec!(1.0)),
            GameResult::Draw => (dec!(0.5), dec!(0.5)),
        };

        let k_decimal = Decimal::from(self.k);

        let new_player_one_rating = player_one + k_decimal * (score_a - expected_a);
        let new_player_two_rating = player_two + k_decimal * (score_b - expected_b);

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
        k: i32,
        game_result: GameResult,
    ) -> (Rating, Rating) {
        let elo = Elo { k };

        elo.calculate(player_1_rating.into(), player_2_rating.into(), game_result)
    }

    #[test_case(2600, 2300, 16, Win => (2602.4, 2297.6))]
    #[test_case(2600, 2300, 16, Loss => (2586.4, 2313.6))]
    #[test_case(2600, 2300, 16, Draw => (2594.4, 2305.6))]
    #[test_case(2600, 2300, 40, Win => (2606.0, 2294.0))]
    #[test_case(2600, 2300, 40, Loss => (2566.0, 2334.0))]
    #[test_case(2600, 2300, 40, Draw => (2586.0, 2314.0))]
    fn calculates_elo_correctly(
        player_1_rating: i32,
        player_2_rating: i32,
        k: i32,
        game_result: GameResult,
    ) -> (f64, f64) {
        let (new_player_1, new_player_2) =
            calculates_elo(player_1_rating, player_2_rating, k, game_result);
        (new_player_1.value(), new_player_2.value())
    }

    #[test_case(2600, 2300, 16, Win => (2602, 2298))]
    #[test_case(2600, 2300, 16, Loss => (2586, 2314))]
    #[test_case(2600, 2300, 16, Draw => (2594, 2306))]
    #[test_case(2600, 2300, 40, Win => (2606, 2294))]
    #[test_case(2600, 2300, 40, Loss => (2566, 2334))]
    #[test_case(2600, 2300, 40, Draw => (2586, 2314))]
    fn calculates_elo_correctly_when_rounded(
        player_1_rating: i32,
        player_2_rating: i32,
        k: i32,
        game_result: GameResult,
    ) -> (i32, i32) {
        let (new_player_1, new_player_2) =
            calculates_elo(player_1_rating, player_2_rating, k, game_result);
        (new_player_1.round_to_i32(), new_player_2.round_to_i32())
    }

    #[test_case(2600, 2300, 16, vec![Win] => (2602.4, 2297.6))]
    #[test_case(2600, 2300, 16, vec![Win, Win] => (2604.8, 2295.2))]
    #[test_case(2600, 2300, 16, vec![Win, Win, Loss] => (2591.1, 2308.9))]
    fn calculates_multiple_correctly(
        player_1_rating: i32,
        player_2_rating: i32,
        k: i32,
        game_result: Vec<GameResult>,
    ) -> (f64, f64) {
        let elo = Elo { k };

        let (new_player_1, new_player_2) =
            elo.calculate_multiple(player_1_rating.into(), player_2_rating.into(), game_result);
        (new_player_1.value(), new_player_2.value())
    }
}
