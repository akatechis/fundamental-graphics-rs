#![feature(test)]
extern crate graphics;
extern crate test;

use test::Bencher;

use graphics::geometry::Vec2;

#[bench]
fn benchmark_immutable_vector_addition (b: &mut Bencher) {
  b.iter(move || {
    let vecs = (0..1000.0).map(|n| Vec2::new(n * 2.0, n * n));
    let mut sum = Vec2::null();
    for v in vecs {
      sum = sum + v;
    }
  });
}
