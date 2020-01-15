fn main() {
  let pair = (0, -2);
  println!("Tell me about {:?}", pair);
  match pair {
    (0, y) => println!("First is `0` and `y` is `{:?}`", y),
    _ => println!("It doesn't matter what ther are"),
  }
}
