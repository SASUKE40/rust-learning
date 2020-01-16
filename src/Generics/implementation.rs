struct GenVal<T> {
  gen_val: T,
}

impl<T> GenVal<T> {
  fn value(&self) -> &T {
    &self.gen_val
  }
}

fn main() {
  let y = GenVal { gen_val: 3i32 };
  println!("{}", y.value());
}
