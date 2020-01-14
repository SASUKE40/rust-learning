// struct Point {
//   x: f32,
//   y: f32,
// }

// struct Rectangle {
//   top_left: Point,
//   bottom_right: Point,
// }

#[derive(Debug)]
struct Person<'a> {
  name: &'a str,
  age: u8,
}

fn main() {
  let name = "Edward";
  let age = 27;
  let edward = Person { name, age };
  println!("{:?}", edward);
}
