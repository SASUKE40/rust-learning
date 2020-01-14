fn main() {
  let _immutable_bindings = 1;
  let mut mutable_bindings = 1;
  println!("Before mutation: {}", mutable_bindings);
  mutable_bindings += 1;
  println!("After mutation: {}", mutable_bindings);
  _immutable_bindings += 1;
}
