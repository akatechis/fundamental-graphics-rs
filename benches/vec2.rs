#![feature(test)]
extern crate graphics;
extern crate test;

use test::Bencher;

use graphics::geometry::Vec2;

#[bench]
fn benchmark_immutable_vector_addition (b: &mut Bencher) {
  b.iter(move || {
    // allocate 1000 Vec2's
    let vecs = (0..1000).map(|n| {
      let f = n as f64;
      Vec2::new(f * 2.0, f * f)
    });

    let mut sum = Vec2::null();
    for v in vecs {
      sum = sum + v;
    }
  });
}
