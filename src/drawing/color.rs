use crate::macros::fuzzy_eq::FuzzyEq;

use std::fmt::{Debug, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone)]
pub struct Color {
  pub red: f64,
  pub green: f64,
  pub blue: f64,
}

//  Instantiations
impl Color {
  pub fn new(red: f64, green: f64, blue: f64) -> Self {
    Self { red, green, blue }
  }

  pub fn black() -> Self {
    Self {
      red: 0.0,
      green: 0.0,
      blue: 0.0,
    }
  }

  pub fn white() -> Self {
    Self {
      red: 1.0,
      green: 1.0,
      blue: 1.0,
    }
  }

  pub fn red() -> Self {
    Self {
      red: 1.0,
      green: 0.0,
      blue: 0.0,
    }
  }

  pub fn green() -> Self {
    Self {
      red: 0.0,
      green: 1.0,
      blue: 0.0,
    }
  }

  pub fn blue() -> Self {
    Self {
      red: 0.0,
      green: 0.0,
      blue: 1.0,
    }
  }
}

impl Add<Self> for Color {
  type Output = Self;

  fn add(self, rhs: Self) -> Self {
    Self {
      red: self.red + rhs.red,
      green: self.green + rhs.green,
      blue: self.blue + rhs.blue,
    }
  }
}

impl AddAssign<Self> for Color {
  fn add_assign(&mut self, rhs: Self) {
    *self = Self {
      red: self.red + rhs.red,
      green: self.green + rhs.green,
      blue: self.blue + rhs.blue,
    }
  }
}

impl Neg for Color {
  type Output = Self;

  fn neg(self) -> Self {
    Color::new(-self.red, -self.green, -self.blue)
  }
}

impl Sub<Self> for Color {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self {
    Color {
      red: self.red - rhs.red,
      green: self.green - rhs.green,
      blue: self.blue - rhs.blue,
    }
  }
}

impl SubAssign<Self> for Color {
  fn sub_assign(&mut self, rhs: Self) {
    *self = Color {
      red: self.red - rhs.red,
      green: self.green - rhs.green,
      blue: self.blue - rhs.blue,
    }
  }
}

impl Mul<f64> for Color {
  type Output = Self;

  fn mul(self, scalar: f64) -> Self {
    Color {
      red: self.red * scalar,
      green: self.green * scalar,
      blue: self.blue * scalar,
    }
  }
}

impl MulAssign<f64> for Color {
  fn mul_assign(&mut self, scalar: f64) {
    *self = Color {
      red: self.red * scalar,
      green: self.green * scalar,
      blue: self.blue * scalar,
    }
  }
}

impl Div<f64> for Color {
  type Output = Self;

  fn div(self, scalar: f64) -> Self {
    Color {
      red: self.red / scalar,
      green: self.green / scalar,
      blue: self.blue / scalar,
    }
  }
}

impl DivAssign<f64> for Color {
  fn div_assign(&mut self, scalar: f64) {
    *self = Color {
      red: self.red / scalar,
      green: self.green / scalar,
      blue: self.blue / scalar,
    }
  }
}

impl Mul<Color> for Color {
  type Output = Self;

  fn mul(self, rhs: Color) -> Self {
    Color {
      red: self.red * rhs.red,
      green: self.green * rhs.green,
      blue: self.blue * rhs.blue,
    }
  }
}

impl MulAssign<Color> for Color {
  fn mul_assign(&mut self, rhs: Color) {
    *self = Color {
      red: self.red * rhs.red,
      green: self.green * rhs.green,
      blue: self.blue * rhs.blue,
    }
  }
}

impl FuzzyEq<Color> for Color {
  fn fuzzy_eq(&self, rhs: &Self) -> bool {
    self.red.fuzzy_eq(&rhs.red) && self.green.fuzzy_eq(&rhs.green) && self.blue.fuzzy_eq(&rhs.blue)
  }
}

impl Debug for Color {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(
      f,
      "red: {:indent$.4}, \tgreen: {:indent$.4}, \tblue: {:indent$.4}",
      self.red,
      self.green,
      self.blue,
      indent = 12
    )
  }
}

#[cfg(test)]
mod color_tests {
  use super::*;

  mod instantiation_tests {
    use super::*;
    use crate::assert_feq;

    #[test]
    fn colors_are_red_green_blue_tuples() {
      let c = Color::new(-0.5, 0.4, 1.7);

      assert_feq!(c.red, -0.5);
      assert_feq!(c.green, 0.4);
      assert_feq!(c.blue, 1.7);
    }
  }

  mod operation_tests {
    use super::*;
    use crate::assert_feq;

    #[test]
    fn adding_colors() {
      let c1 = Color::new(0.9, 0.6, 0.75);
      let c2 = Color::new(0.7, 0.1, 0.25);

      let actual = c1 + c2;
      let expected = Color::new(1.6, 0.7, 1.0);

      assert_feq!(actual, expected);
    }

    #[test]
    fn subtractinging_colors() {
      let c1 = Color::new(0.9, 0.6, 0.75);
      let c2 = Color::new(0.7, 0.1, 0.25);

      let actual = c1 - c2;
      let expected = Color::new(0.2, 0.5, 0.5);

      assert_feq!(actual, expected);
    }

    #[test]
    fn multiplying_a_color_by_a_scalar() {
      let c = Color::new(0.2, 0.3, 0.4);

      let actual = c * 2.0;
      let expected = Color::new(0.4, 0.6, 0.8);

      assert_feq!(actual, expected);
    }

    #[test]
    fn multiplying_colors() {
      let c1 = Color::new(1.0, 0.2, 0.4);
      let c2 = Color::new(0.9, 1.0, 0.1);

      let actual = c1 * c2;
      let expected = Color::new(0.9, 0.2, 0.04);

      assert_feq!(actual, expected);
    }
  }
}
