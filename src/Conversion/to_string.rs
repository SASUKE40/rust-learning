use std::fmt;

struct Circle {
  radius: i32,
}

impl fmt::Display for Circle {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Circle of radius {}", self.radius)
  }
}

fn main() {
  let circle = Circle { radius: 10 };
  println!("{}", circle.to_string());
}
