use std::fmt::Formatter;
use std::fmt::Error;
use std::fmt::Display;
use std::ops::{
  Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg
};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec2(f64, f64);

impl Vec2 {
  pub fn new (x: f64, y: f64) -> Self {
    Vec2(x, y)
  }

  pub fn unit () -> Self {
    Vec2(1.0, 1.0)
  }

  pub fn null () -> Self {
    Vec2(0.0, 0.0)
  }

  pub fn x (&self) -> f64 {
    self.0
  }

  pub fn y (&self) -> f64 {
    self.1
  }
}

impl Add for Vec2 {
  type Output = Vec2;

  fn add (self, rhs: Vec2) -> Self::Output {
    let Vec2(a_x, a_y) = self;
    let Vec2(b_x, b_y) = rhs;

    Vec2(a_x + b_x, a_y + b_y)
  }
}

impl Display for Vec2 {
  fn fmt (&self, fmt: &mut Formatter) -> Result<(), Error> {
    let Vec2(x, y) = self;
    write!(fmt, "<{}, {}>", x, y)
  }
}

impl AddAssign for Vec2 {
  fn add_assign (&mut self, rhs: Self) {
    let Vec2(b_x, b_y) = rhs;
    self.0 += b_x;
    self.1 += b_y;
  }
}

impl Sub for Vec2 {
  type Output = Vec2;

  fn sub (self, rhs: Vec2) -> Self::Output {
    let Vec2(a_x, a_y) = self;
    let Vec2(b_x, b_y) = rhs;

    Vec2(a_x - b_x, a_y - b_y)
  }
}

impl SubAssign for Vec2 {
  fn sub_assign (&mut self, rhs: Vec2) {
    let Vec2(b_x, b_y) = rhs;
    self.0 -= b_x;
    self.1 -= b_y;
  }
}

impl Mul for Vec2 {
  type Output = f64;

  fn mul (self, rhs: Self) -> Self::Output {
    let Vec2(a_x, a_y) = self;
    let Vec2(b_x, b_y) = rhs;

    a_x * b_x + a_y * b_y
  }
}

impl Mul<f64> for Vec2 {
  type Output = Vec2;

  fn mul (self, rhs: f64) -> Self::Output {
    let Vec2(a_x, a_y) = self;

    Vec2(a_x * rhs, a_y * rhs)
  }
}

impl Mul<Vec2> for f64 {
  type Output = Vec2;

  fn mul (self, rhs: Vec2) -> Self::Output {
    let Vec2(x, y) = rhs;
    Vec2(x * self, y * self)
  }
}

impl MulAssign<f64> for Vec2 {
  fn mul_assign(&mut self, rhs: f64) {
    self.0 *= rhs;
    self.1 *= rhs;
  }
}

impl Div<f64> for Vec2 {
  type Output = Vec2;

  fn div (self, rhs: f64) -> Self::Output {
    let Vec2(a_x, a_y) = self;

    Vec2(a_x / rhs, a_y / rhs)
  }
}

impl DivAssign<f64> for Vec2 {
  fn div_assign(&mut self, rhs: f64) {
    self.0 /= rhs;
    self.1 /= rhs;
  }
}

impl Neg for Vec2 {
  type Output = Vec2;

  fn neg (self) -> Self::Output {
    let Vec2(a_x, a_y) = self;

    Vec2(-a_x, -a_y)
  }
}
