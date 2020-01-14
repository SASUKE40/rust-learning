fn main() {
  let a_binding;
  {
    let x = 2;
    a_binding = x * x;
  }
  println!("a binding: {}", a_binding);
}
