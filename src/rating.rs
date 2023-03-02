use std::ops::Deref;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Rating(f64);

impl Rating {
    pub fn new(rating: f64) -> Self {
        Self(rating)
    }

    pub fn rounded(&self) -> i32 {
        self.0.round() as i32
    }
}

impl From<f64> for Rating {
    fn from(value: f64) -> Self {
        Rating::new(value)
    }
}

impl From<i32> for Rating {
    fn from(value: i32) -> Self {
        Rating::new(value as f64)
    }
}

impl From<i64> for Rating {
    fn from(value: i64) -> Self {
        Rating::new(value as f64)
    }
}

impl From<f32> for Rating {
    fn from(value: f32) -> Self {
        Rating::new(value as f64)
    }
}

impl Deref for Rating {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::Add<f64> for Rating {
    type Output = Rating;

    fn add(self, rhs: f64) -> Self::Output {
        Rating::new(*self + rhs)
    }
}

impl std::ops::Sub<f64> for Rating {
    type Output = Rating;

    fn sub(self, rhs: f64) -> Self::Output {
        Rating::new(*self - rhs)
    }
}
