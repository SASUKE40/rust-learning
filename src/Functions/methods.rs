struct Point {
  x: f64,
  y: f64,
}

impl Point {
  fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
  }
  fn new(x: f64, y: f64) -> Point {
    Point { x: x, y: y }
  }
}

struct Rectangle {
  p1: Point,
  p2: Point,
}

impl Rectangle {
  fn area(&self) -> f64 {
    let Point { x: x1, y: y1 } = self.p1;
    let Point { x: x2, y: y2 } = self.p2;
    ((x1 - x2) * (y1 - y2)).abs()
  }
}

fn main() {
  let rectangle = Rectangle {
    p1: Point::origin(),
    p2: Point::new(3.0, 4.0),
  };
  let area = rectangle.area();
  println!("Rectangle area: {}", area);
}
