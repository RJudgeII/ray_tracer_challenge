use crate::EPSILON;

pub trait FuzzyEq<T> {
  fn fuzzy_eq(&self, other: &T) -> bool;

  fn fuzzy_ne(&self, other: &T) -> bool {
    !self.fuzzy_eq(other)
  }
}

impl FuzzyEq<f64> for f64 {
  fn fuzzy_eq(&self, other: &f64) -> bool {
    (*self - other).abs() < EPSILON
  }
}

#[macro_export]
macro_rules! assert_feq {
  ($left:expr, $right:expr $(,)?) => {
    match (&$left, $right) {
      (left_val, right_val) => {
        if left_val.fuzzy_ne(&right_val) {
          panic!(
            "
            Attempted to assert fuzzy equality.
            The following are not fuzzy equal:
            LEFT VALUE---{:?}
            RIGHT VALUE--{:?}
            ",
            left_val, right_val
          )
        }
      }
    }
  };
}

#[macro_export]
macro_rules! assert_fne {
  ($left:expr, $right:expr $(,)?) => {
    match (&$left, $right) {
      (left_val, right_val) => {
        if left_val.fuzzy_eq(&right_val) {
          panic!(
            "
            Attempted to assert fuzzy equality.
            The following are not fuzzy equal:
            LEFT VALUE---{:?}
            RIGHT VALUE--{:?}
            ",
            left_val, right_val
          )
        }
      }
    }
  };
}
