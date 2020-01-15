fn main() {
  for n in 1..101 {
    if n % 15 == 0 {
      println!("fizzbuzz");
    } else if n % 3 == 0 {
      println!("fizz");
    } else if n % 5 == 0 {
      println!("buzz");
    } else {
      println!("{}", n)
    }
  }
  for n in 1..=100 {
    if n % 15 == 0 {
      println!("fizzbuzz");
    } else if n % 3 == 0 {
      println!("fizz");
    } else if n % 5 == 0 {
      println!("buzz");
    } else {
      println!("{}", n)
    }
  }
  let names = vec!["Bob", "Frank", "Ferris"];
  for name in names.iter() {
    match name {
      &"Ferris" => println!("There is a rustacean among us!"),
      _ => println!("{}", name),
    }
  }
  println!("{:?}", names);
}
