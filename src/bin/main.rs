extern crate graphics;

use graphics::geometry::Vec2;

fn main () {
  let v = Vec2::new(2.3, 4.4999);
  let u = Vec2::new(1.0, 5.0);
  println!("v = {}", v);
  println!("u = {}", u);
  println!("-----\n\n");

  println!("u+v = {}", u + v);
  println!("3u = {}", u * 3.0);
}
