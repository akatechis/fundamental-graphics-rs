use std::ops::{
  Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg
};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec2([f64; 2]);

impl Vec2 {
  pub fn new (x: f64, y: f64) -> Self {
    Vec2([x, y])
  }

  pub fn unit () -> Self {
    Vec2([1.0, 1.0])
  }

  pub fn null () -> Self {
    Vec2([0.0, 0.0])
  }

  pub fn x (&self) -> f64 {
    self.0[0]
  }

  pub fn y (&self) -> f64 {
    self.0[1]
  }
}

impl Add for Vec2 {
  type Output = Vec2;

  fn add (self, rhs: Vec2) -> Self::Output {
    let [a_x, a_y] = self.0;
    let [b_x, b_y] = rhs.0;

    Vec2([a_x + b_x, a_y + b_y])
  }
}

impl AddAssign for Vec2 {
  fn add_assign (&mut self, rhs: Self) {
    let [b_x, b_y] = rhs.0;
    self.0[0] += b_x;
    self.0[1] += b_y;
  }
}

impl Sub for Vec2 {
  type Output = Vec2;

  fn sub (self, rhs: Vec2) -> Self::Output {
    let [a_x, a_y] = self.0;
    let [b_x, b_y] = rhs.0;

    Vec2([a_x - b_x, a_y - b_y])
  }
}

impl SubAssign for Vec2 {
  fn sub_assign (&mut self, rhs: Vec2) {
    let [b_x, b_y] = rhs.0;
    self.0[0] -= b_x;
    self.0[1] -= b_y;
  }
}

impl Mul for Vec2 {
  type Output = f64;

  fn mul (self, rhs: Self) -> Self::Output {
    let [a_x, a_y] = self.0;
    let [b_x, b_y] = rhs.0;

    a_x * b_x + a_y * b_y
  }
}

impl Mul<f64> for Vec2 {
  type Output = Vec2;

  fn mul (self, rhs: f64) -> Self::Output {
    let [a_x, a_y] = self.0;

    Vec2([a_x * rhs, a_y * rhs])
  }
}

impl MulAssign<f64> for Vec2 {
  fn mul_assign(&mut self, rhs: f64) {
    self.0[0] *= rhs;
    self.0[1] *= rhs;
  }
}

impl Div<f64> for Vec2 {
  type Output = Vec2;

  fn div (self, rhs: f64) -> Self::Output {
    let [a_x, a_y] = self.0;

    Vec2([a_x / rhs, a_y / rhs])
  }
}

impl DivAssign<f64> for Vec2 {
  fn div_assign(&mut self, rhs: f64) {
    self.0[0] /= rhs;
    self.0[1] /= rhs;
  }
}

impl Neg for Vec2 {
  type Output = Vec2;

  fn neg (self) -> Self::Output {
    let [a_x, a_y] = self.0;

    Vec2([-a_x, -a_y])
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_adding_two_vectors_immutably () {
    let a = Vec2::new(5.0, 12.0);
    let b = Vec2::new(-10.0, 13.0);

    assert_eq!(Vec2::new(-5.0, 25.0), a + b);
    assert_eq!(Vec2::new(-5.0, 25.0), b + a);
    assert_eq!(Vec2::new(10.0, 24.0), a + a);
  }

  #[test]
  fn test_adding_two_vectors_mutably () {
    let mut a = Vec2::new(5.0, 12.0);
    let b = Vec2::new(-10.0, 13.0);

    a += b;

    assert_eq!(Vec2::new(-5.0, 25.0), a);
  }

  #[test]
  fn test_subtracting_two_vectors_immutably () {
    let a = Vec2::new(5.0, 12.0);
    let b = Vec2::new(-10.0, 13.0);

    assert_eq!(Vec2::new(15.0, -1.0), a - b);
    assert_eq!(Vec2::new(-15.0, 1.0), b - a);
  }

  #[test]
  fn test_subtracting_two_vectors_mutably () {
    let mut a = Vec2::new(5.0, 12.0);
    let b = Vec2::new(-10.0, 13.0);
    a -= b;

    assert_eq!(Vec2::new(15.0, -1.0), a);
  }

  #[test]
  fn test_dot_product_of_two_vec2 () {
    let a1 = Vec2::new(3.0, 5.0);
    let a2 = Vec2::new(2.0, 10.0);
    assert_eq!(56.0, a1 * a2);

    let b1 = Vec2::new(0.0, 0.0);
    let b2 = Vec2::new(1.0, 1.0);
    assert_eq!(0.0, b1 * b2);

    let c1 = Vec2::new(1.0, 1.0);
    let c2 = Vec2::new(10.0, 3.7);
    assert_eq!(13.7, c1 * c2);
  }

  #[test]
  fn test_scalar_multiplication_of_vec2 () {
    assert_eq!(Vec2::new(6.2, 5.4), Vec2::new(3.1, 2.7) * 2.0);
    assert_eq!(Vec2::new(0.0, 0.0), Vec2::new(3.1, 2.7) * 0.0);
    assert_eq!(Vec2::new(3.1, 2.7), Vec2::new(3.1, 2.7) * 1.0);
    assert_eq!(Vec2::new(15.5, -12.5), Vec2::new(-3.1, 2.5) * -5.0);
  }

  #[test]
  fn test_scalar_division_of_vec2 () {
    assert_eq!(Vec2::new(3.1, 2.7), Vec2::new(6.2, 5.4) / 2.0);
    assert_eq!(Vec2::new(1.0 / 0.0, 1.0 / 0.0), Vec2::new(12345.67899, 999.333) / 0.0);
    assert_eq!(Vec2::new(3.1, 2.7), Vec2::new(3.1, 2.7) / 1.0);
    assert_eq!(Vec2::new(-3.1, 2.5), Vec2::new(15.5, -12.5) / -5.0);
  }

  #[test]
  fn test_scalar_multiplication_assignment_of_vec2 () {
    let mut v1 = Vec2::new(3.1, 2.7);
    v1 *= 2.0;
    assert_eq!(Vec2::new(6.2, 5.4), v1);

    let mut v2 = Vec2::new(3.1, 2.7);
    v2 *= 0.0;
    assert_eq!(Vec2::new(0.0, 0.0), v2);

    let mut v3 = Vec2::new(3.1, 2.7);
    v3 *= 1.0;
    assert_eq!(Vec2::new(3.1, 2.7), v3);

    let mut v4 = Vec2::new(-3.1, 2.5);
    v4 *= -5.0;
    assert_eq!(Vec2::new(15.5, -12.5), v4);
  }

  #[test]
  fn test_scalar_division_assignment_of_vec2 () {
    let mut v1 = Vec2::new(6.2, 5.4);
    v1 /= 2.0;
    assert_eq!( Vec2::new(3.1, 2.7), v1);

    let mut v2 = Vec2::new(3.1, 2.7);
    v2 /= 0.0;
    assert_eq!( Vec2::new(1.0 / 0.0, 1.0 / 0.0), v2);

    let mut v3 = Vec2::new(3.1, 2.7);
    v3 /= 1.0;
    assert_eq!( Vec2::new(3.1, 2.7), v3);

    let mut v4 = Vec2::new(15.5, -12.5);
    v4 /= -5.0;
    assert_eq!( Vec2::new(-3.1, 2.5), v4);
  }

  #[test]
  fn test_vec2_unary_negation_operator () {
    assert_eq!(Vec2::new(-2.0, -3.7), -Vec2::new(2.0, 3.7));
    assert_eq!(Vec2::new(2.0, -3.7), -Vec2::new(-2.0, 3.7));
  }

  #[test]
  fn test_unit_vec2_constructor () {
    let v = Vec2::unit();
    assert_eq!(1.0, v.x());
    assert_eq!(1.0, v.y());
  }

  #[test]
  fn test_null_vec2_constructor () {
    let v = Vec2::null();
    assert_eq!(0.0, v.x());
    assert_eq!(0.0, v.y());
  }
}
