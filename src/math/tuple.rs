use crate::macros::fuzzy_eq::FuzzyEq;
use std::fmt::{Debug, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone)]
pub struct Tuple {
  pub x: f64,
  pub y: f64,
  pub z: f64,
  pub w: f64,
}

//  Instantiations
impl Tuple {
  pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
    Self { x, y, z, w }
  }

  pub fn point(x: f64, y: f64, z: f64) -> Self {
    Self { x, y, z, w: 1.0 }
  }

  pub fn vector(x: f64, y: f64, z: f64) -> Self {
    Self { x, y, z, w: 0.0 }
  }

  pub fn zero() -> Self {
    Self::vector(0.0, 0.0, 0.0)
  }

  pub fn origin() -> Self {
    Self::point(0.0, 0.0, 0.0)
  }
}

//  Verifications
impl Tuple {
  pub fn is_point(&self) -> bool {
    self.w == 1.0
  }

  pub fn is_vector(&self) -> bool {
    self.w == 0.0
  }
}

//  Methods
impl Tuple {
  pub fn magnitude(&self) -> f64 {
    self.magnitude_squared().sqrt()
  }

  pub fn magnitude_squared(&self) -> f64 {
    if !self.is_vector() {
      panic!("Can only compute the magnitude for vectors.");
    }
    self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
  }

  pub fn normalize(self) -> Self {
    if !self.is_vector() {
      panic!("Can only normalize vectors.");
    }
    let magnitude = self.magnitude();
    self / magnitude
  }

  pub fn dot(self, rhs: Self) -> f64 {
    if !self.is_vector() || !rhs.is_vector() {
      panic!("Can only compute the dot products for two vectors.");
    }
    (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
  }

  pub fn cross(self, rhs: Self) -> Self {
    if !self.is_vector() || !rhs.is_vector() {
      panic!("Cross product can only be calculated for two vectors.");
    }
    Tuple::vector(
      (self.y * rhs.z) - (self.z * rhs.y),
      (self.z * rhs.x) - (self.x * rhs.z),
      (self.x * rhs.y) - (self.y * rhs.x),
    )
  }
}

//  Operations
impl Add<Self> for Tuple {
  type Output = Self;

  fn add(self, rhs: Self) -> Self {
    if self.is_point() && rhs.is_point() {
      panic!("Cannot add two points.");
    }
    Tuple {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
      w: self.w + rhs.w,
    }
  }
}

impl AddAssign<Self> for Tuple {
  fn add_assign(&mut self, rhs: Self) {
    if self.is_point() && rhs.is_point() {
      panic!("Cannot add two points.");
    }
    *self = Tuple {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
      w: self.w + rhs.w,
    }
  }
}

impl Sub<Self> for Tuple {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self {
    if self.is_vector() && rhs.is_point() {
      panic!("Cannot subtract a point from a vector.");
    }
    Tuple {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
      w: self.w - rhs.w,
    }
  }
}

impl SubAssign<Self> for Tuple {
  fn sub_assign(&mut self, rhs: Self) {
    if self.is_vector() && rhs.is_point() {
      panic!("Cannot subtract a point from a vector.");
    }
    *self = Tuple {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
      w: self.w - rhs.w,
    }
  }
}

impl Neg for Tuple {
  type Output = Self;

  fn neg(self) -> Self {
    Tuple::new(-self.x, -self.y, -self.z, self.w)
  }
}

impl Mul<f64> for Tuple {
  type Output = Self;

  fn mul(self, scalar: f64) -> Self {
    if !self.is_vector() {
      panic!("Can only use the '*' operator with vectors, not points.");
    }
    Tuple {
      x: self.x * scalar,
      y: self.y * scalar,
      z: self.z * scalar,
      w: self.w * scalar,
    }
  }
}

impl MulAssign<f64> for Tuple {
  fn mul_assign(&mut self, scalar: f64) {
    if !self.is_vector() {
      panic!("Can only use the '*=' operator with vectors, not points.");
    }
    *self = Tuple {
      x: self.x * scalar,
      y: self.y * scalar,
      z: self.z * scalar,
      w: self.w * scalar,
    }
  }
}

impl Div<f64> for Tuple {
  type Output = Self;

  fn div(self, scalar: f64) -> Self {
    if !self.is_vector() {
      panic!("Can only use the '/' operator with vectors, not points.");
    }
    Tuple {
      x: self.x / scalar,
      y: self.y / scalar,
      z: self.z / scalar,
      w: self.w / scalar,
    }
  }
}

impl DivAssign<f64> for Tuple {
  fn div_assign(&mut self, scalar: f64) {
    if !self.is_vector() {
      panic!("Can only use the '/=' operator with vectors, not points.");
    }
    *self = Tuple {
      x: self.x / scalar,
      y: self.y / scalar,
      z: self.z / scalar,
      w: self.w / scalar,
    }
  }
}

//  Fuzzy Equality
impl FuzzyEq<Tuple> for Tuple {
  fn fuzzy_eq(&self, rhs: &Self) -> bool {
    self.x.fuzzy_eq(&rhs.x)
      && self.y.fuzzy_eq(&rhs.y)
      && self.z.fuzzy_eq(&rhs.z)
      && self.w.fuzzy_eq(&rhs.w)
  }
}

//  Debug
impl Debug for Tuple {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(
      f,
      "x: {:indent$.4}, \ty: {:indent$.4}, \tz: {:indent$.4}",
      self.x,
      self.y,
      self.z,
      indent = 12
    )
  }
}

#[cfg(test)]
mod tuple_tests {
  use super::*;

  mod instantiation_tests {
    use super::*;

    #[test]
    fn point_fn_creates_a_point() {
      let point = Tuple::point(4.0, -4.0, 3.0);

      assert!(point.is_point());
    }

    #[test]
    fn vector_fn_creates_a_vector() {
      let vector = Tuple::vector(4.0, -4.0, 3.0);

      assert!(vector.is_vector());
    }
  }

  mod verification_tests {
    use super::*;

    #[test]
    fn a_tuple_with_a_w_of_one_is_a_point() {
      let point = Tuple::new(4.3, -4.2, 3.1, 1.0);

      assert!(point.is_point());
    }

    #[test]
    fn a_tuple_with_a_w_of_zero_is_a_vector() {
      let vector = Tuple::new(4.3, -4.2, 3.1, 0.0);

      assert!(vector.is_vector());
    }
  }

  mod operation_tests {
    use super::*;
    use crate::assert_feq;

    #[test]
    fn adding_two_tuples() {
      let a1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
      let a2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);

      let actual = a1 + a2;
      let expected = Tuple::new(1.0, 1.0, 6.0, 1.0);

      assert_feq!(actual, expected);
    }

    #[test]
    fn add_assigning_two_tuples() {
      let mut a1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
      let a2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);

      a1 += a2;
      let expected = Tuple::new(1.0, 1.0, 6.0, 1.0);

      assert_feq!(a1, expected);
    }

    #[test]
    #[should_panic]
    fn adding_two_points_should_panic() {
      let p1 = Tuple::point(1.0, 2.0, 3.0);
      let p2 = Tuple::point(2.0, 3.0, 4.0);
      let _actual = p1 + p2;
    }

    #[test]
    fn subracting_two_tuples() {
      let a1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
      let a2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);

      let actual = a1 - a2;
      let expected = Tuple::new(5.0, -5.0, 4.0, 1.0);

      assert_feq!(actual, expected);
    }

    #[test]
    fn sub_assigning_two_tuples() {
      let mut a1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
      let a2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);

      a1 -= a2;
      let expected = Tuple::new(5.0, -5.0, 4.0, 1.0);

      assert_feq!(a1, expected);
    }

    #[test]
    fn subracting_two_points_gives_a_vector() {
      let p1 = Tuple::point(3.0, 2.0, 1.0);
      let p2 = Tuple::point(5.0, 6.0, 7.0);

      let actual = p1 - p2;
      let expected = Tuple::vector(-2.0, -4.0, -6.0);

      assert_feq!(actual, expected);
    }

    #[test]
    fn subtracting_a_vector_from_a_point_gives_a_point() {
      let p = Tuple::point(3.0, 2.0, 1.0);
      let v = Tuple::vector(5.0, 6.0, 7.0);

      let actual = p - v;
      let expected = Tuple::point(-2.0, -4.0, -6.0);

      assert_feq!(actual, expected);
    }

    #[test]
    fn subracting_two_vectors_gives_a_vector() {
      let v1 = Tuple::vector(3.0, 2.0, 1.0);
      let v2 = Tuple::vector(5.0, 6.0, 7.0);

      let actual = v1 - v2;
      let expected = Tuple::vector(-2.0, -4.0, -6.0);

      assert_feq!(actual, expected);
    }

    #[test]
    #[should_panic]
    fn subtracting_a_point_from_a_vector_should_panic() {
      let p = Tuple::point(3.0, 2.0, 1.0);
      let v = Tuple::vector(5.0, 6.0, 7.0);
      let _actual = v - p;
    }

    #[test]
    fn subtracting_a_vector_from_the_zero_vector() {
      let zero = Tuple::zero();
      let v = Tuple::vector(1.0, 2.0, 3.0);

      let actual = zero - v;
      let expected = Tuple::vector(-1.0, -2.0, -3.0);

      assert_feq!(actual, expected);
    }

    #[test]
    fn negating_a_tuple() {
      //  Note: This test differs from the book as I want points to remanin
      //    points and vectors to remain vectors.
      let a = Tuple::point(1.0, -2.0, 3.0);

      let actual = -a;
      let expected = Tuple::point(-1.0, 2.0, -3.0);

      assert_feq!(actual, expected);
    }

    #[test]
    fn multiplying_a_tuple_by_a_scalar() {
      //  Note: This test differs from the book as I only want vectors to be
      //    scalable. Points shuld not scale.
      let a = Tuple::vector(1.0, -2.0, 3.0);

      let actual = a * 2.0;
      let expected = Tuple::vector(2.0, -4.0, 6.0);

      assert_feq!(actual, expected);
    }

    #[test]
    #[should_panic]
    fn multiplying_a_point_by_a_scalar_should_panic() {
      //  Note: This test differs from the book as I only want vectors to be
      //    scalable. Points shuld not scale.
      let a = Tuple::point(1.0, -2.0, 3.0);

      let _actual = a * 2.0;
    }

    #[test]
    fn dividing_a_tuple_by_a_scalar() {
      //  Note: This test differs from the book as I only want vectors to be
      //    scalable. Points shuld not scale.
      let a = Tuple::vector(1.0, -2.0, 3.0);

      let actual = a / 2.0;
      let expected = Tuple::vector(0.5, -1.0, 1.5);

      assert_feq!(actual, expected);
    }

    #[test]
    #[should_panic]
    fn dividing_a_point_by_a_scalar_should_panic() {
      //  Note: This test differs from the book as I only want vectors to be
      //    scalable. Points shuld not scale.
      let a = Tuple::point(1.0, -2.0, 3.0);

      let _actual = a / 2.0;
    }
  }

  mod method_tests {
    use super::*;
    use crate::assert_feq;

    #[test]
    fn computing_the_magnitude_of_a_vector() {
      let v = Tuple::vector(1.0, 2.0, 3.0);

      let actual = v.magnitude();
      let expected = f64::sqrt(14.0);

      assert_feq!(actual, expected);
    }

    #[test]
    #[should_panic]
    fn computing_the_magnitude_of_a_point_should_panic() {
      let p = Tuple::point(1.0, 2.0, 3.0);

      let _actual = p.magnitude();
    }

    #[test]
    fn normalizing_a_vector() {
      let v = Tuple::vector(1.0, 2.0, 3.0);

      let actual = v.normalize();
      let expected = Tuple::vector(0.26726, 0.53452, 0.80178);

      assert_feq!(actual, expected);
    }

    #[test]
    #[should_panic]
    fn normalizing_a_point_should_panic() {
      let p = Tuple::point(1.0, 2.0, 3.0);

      let _actual = p.normalize();
    }

    #[test]
    fn the_magnitude_of_a_normalized_vector_should_be_one() {
      let v = Tuple::vector(1.0, 2.0, 3.0);

      let actual = v.normalize().magnitude();
      let expected = 1.0;

      assert_feq!(actual, expected);
    }

    #[test]
    fn the_dot_product_of_two_vectors() {
      let a = Tuple::vector(1.0, 2.0, 3.0);
      let b = Tuple::vector(2.0, 3.0, 4.0);

      let actual = Tuple::dot(a, b);
      let expected = 20.0;

      assert_feq!(actual, expected);
    }

    #[test]
    #[should_panic]
    fn the_dot_product_including_a_point_should_panic() {
      let a = Tuple::vector(1.0, 2.0, 3.0);
      let b = Tuple::point(2.0, 3.0, 4.0);

      let _actual = Tuple::dot(a, b);
    }

    #[test]
    fn the_cross_product_of_two_vectors() {
      let a = Tuple::vector(1.0, 2.0, 3.0);
      let b = Tuple::vector(2.0, 3.0, 4.0);

      let actual_one = Tuple::cross(a.clone(), b.clone());
      let actual_two = Tuple::cross(b, a);
      let expected_one = Tuple::vector(-1.0, 2.0, -1.0);
      let expected_two = Tuple::vector(1.0, -2.0, 1.0);

      assert_feq!(actual_one, expected_one);
      assert_feq!(actual_two, expected_two);
    }

    #[test]
    #[should_panic]
    fn the_cross_product_including_a_point_should_panic() {
      let a = Tuple::vector(1.0, 2.0, 3.0);
      let b = Tuple::point(2.0, 3.0, 4.0);

      let _actual = Tuple::cross(a, b);
    }
  }
}
