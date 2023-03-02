use rust_decimal::prelude::ToPrimitive;
use rust_decimal::{Decimal, RoundingStrategy};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Rating(Decimal);

impl Rating {
    pub fn new(rating: Decimal) -> Self {
        Self(rating)
    }

    /// Rating rounded to the units
    ///
    /// Useful when we don't want to display `Rating` with decimal places
    pub fn round_to_i32(&self) -> i32 {
        self.round_dp_with_strategy(0, RoundingStrategy::MidpointAwayFromZero)
            .to_i32()
            .unwrap_or(0)
    }

    pub fn rounded(&self, decimal_places: u32) -> f64 {
        self.round_dp_with_strategy(decimal_places, RoundingStrategy::MidpointAwayFromZero)
            .to_f64()
            .unwrap_or(0.0)
    }

    pub fn value(&self) -> f64 {
        self.rounded(1)
    }
}

impl From<Decimal> for Rating {
    fn from(value: Decimal) -> Self {
        Rating::new(value)
    }
}

impl From<i32> for Rating {
    fn from(value: i32) -> Self {
        Rating::new(Decimal::from(value))
    }
}

impl From<i64> for Rating {
    fn from(value: i64) -> Self {
        Rating::new(Decimal::from(value))
    }
}

impl std::ops::Deref for Rating {
    type Target = Decimal;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::Add<Decimal> for Rating {
    type Output = Rating;

    fn add(self, rhs: Decimal) -> Self::Output {
        Rating::new(*self + rhs)
    }
}

impl std::ops::Sub<Decimal> for Rating {
    type Output = Rating;

    fn sub(self, rhs: Decimal) -> Self::Output {
        Rating::new(*self - rhs)
    }
}

impl std::fmt::Display for Rating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::prelude::FromPrimitive;
    use test_case::test_case;

    #[test_case(1.23 => 1.2)]
    #[test_case(1.25 => 1.3)]
    #[test_case(1.0005 => 1.0)]
    #[test_case(1.89 => 1.9)]
    #[test_case(1.849 => 1.8)]
    fn rounds_to_1_decimal_place_by_default(value: f64) -> f64 {
        Rating::new(Decimal::from_f64(value).unwrap()).value()
    }
}
