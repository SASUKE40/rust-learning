fn main() {
  let number = 13;
  println!("Tell me about {}", number);
  match number {
    2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
    13..=19 => println!("A teen"),
    _ => println!("Ain't special"),
  }
}
