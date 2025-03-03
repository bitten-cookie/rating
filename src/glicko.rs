use rust_decimal::Decimal;

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
pub struct GlickoStrategy {
    /// The default rating if the player is unrated
    initial_rating: Decimal,
    /// The default rating deviation if the player is unrated
    initial_rating_deviation: Decimal,
}

impl GlickoStrategy {
    /// Builds an Glicko calculator
    ///
    /// # Arguments
    /// * `k` - the k-factor to be used for the calculation
    pub fn new(initial_rating: u32, initial_rating_deviation: u32) -> Self {
        Self {
            initial_rating: Decimal::from(initial_rating),
            initial_rating_deviation: Decimal::from(initial_rating_deviation),
        }
    }
}

/// These are the default values proposed by Glicko
impl Default for GlickoStrategy {
    fn default() -> Self {
        GlickoStrategy::new(1500, 300)
    }
}
