extern crate graphics;

mod vec2 {
  use graphics::geometry::Vec2;

  #[test]
  fn adding_two_vectors_immutably () {
    let a = Vec2::new(5.0, 12.0);
    let b = Vec2::new(-10.0, 13.0);

    assert_eq!(Vec2::new(-5.0, 25.0), a + b);
    assert_eq!(Vec2::new(-5.0, 25.0), b + a);
    assert_eq!(Vec2::new(10.0, 24.0), a + a);
  }

  #[test]
  fn adding_two_vectors_mutably () {
    let mut a = Vec2::new(5.0, 12.0);
    let b = Vec2::new(-10.0, 13.0);

    a += b;

    assert_eq!(Vec2::new(-5.0, 25.0), a);
  }

  #[test]
  fn subtracting_two_vectors_immutably () {
    let a = Vec2::new(5.0, 12.0);
    let b = Vec2::new(-10.0, 13.0);

    assert_eq!(Vec2::new(15.0, -1.0), a - b);
    assert_eq!(Vec2::new(-15.0, 1.0), b - a);
  }

  #[test]
  fn subtracting_two_vectors_mutably () {
    let mut a = Vec2::new(5.0, 12.0);
    let b = Vec2::new(-10.0, 13.0);
    a -= b;

    assert_eq!(Vec2::new(15.0, -1.0), a);
  }

  #[test]
  fn dot_product_of_two_vectors () {
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
  fn immutable_scalar_multiplication () {
    assert_eq!(Vec2::new(6.2, 5.4), Vec2::new(3.1, 2.7) * 2.0);
    assert_eq!(Vec2::new(0.0, 0.0), Vec2::new(3.1, 2.7) * 0.0);
    assert_eq!(Vec2::new(3.1, 2.7), Vec2::new(3.1, 2.7) * 1.0);
    assert_eq!(Vec2::new(15.5, -12.5), Vec2::new(-3.1, 2.5) * -5.0);

    assert_eq!(Vec2::new(6.2, 5.4), 2.0 * Vec2::new(3.1, 2.7));
    assert_eq!(Vec2::new(0.0, 0.0), 0.0 * Vec2::new(3.1, 2.7));
    assert_eq!(Vec2::new(3.1, 2.7), 1.0 * Vec2::new(3.1, 2.7));
    assert_eq!(Vec2::new(15.5, -12.5), -5.0 * Vec2::new(-3.1, 2.5));
  }

  #[test]
  fn immutable_scalar_division () {
    assert_eq!(Vec2::new(3.1, 2.7), Vec2::new(6.2, 5.4) / 2.0);
    assert_eq!(Vec2::new(1.0 / 0.0, 1.0 / 0.0), Vec2::new(12345.67899, 999.333) / 0.0);
    assert_eq!(Vec2::new(3.1, 2.7), Vec2::new(3.1, 2.7) / 1.0);
    assert_eq!(Vec2::new(-3.1, 2.5), Vec2::new(15.5, -12.5) / -5.0);
  }

  #[test]
  fn mutable_scalar_multiplication_of_vec2 () {
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
  fn mutable_scalar_division_of_vec2 () {
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
  fn unary_negation_operator () {
    assert_eq!(Vec2::new(-2.0, -3.7), -Vec2::new(2.0, 3.7));
    assert_eq!(Vec2::new(2.0, -3.7), -Vec2::new(-2.0, 3.7));
  }

  #[test]
  fn unit_constructor () {
    let v = Vec2::unit();
    assert_eq!(1.0, v.x());
    assert_eq!(1.0, v.y());
  }

  #[test]
  fn null_constructor () {
    let v = Vec2::null();
    assert_eq!(0.0, v.x());
    assert_eq!(0.0, v.y());
  }

}
