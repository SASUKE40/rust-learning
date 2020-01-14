use std::convert::From;

#[derive(Debug)]
struct Number {
  value: i32,
}

impl From<i32> for Number {
  fn from(value: i32) -> Self {
    Number { value }
  }
}

fn main() {
  let my_str = "hello";
  let my_string = String::from(my_str);
  println!("my_string is {}", my_string);
  let num = Number::from(32);
  println!("num is {:?}", num);

  let int = 5;
  let num2: Number = int.into();
  println!("num2 is {:?}", num2);
}
