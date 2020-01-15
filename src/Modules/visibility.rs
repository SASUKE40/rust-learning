mod my_mod {
  fn private_function() {
    println!("call `my_mod::private_function()`");
  }
  pub fn function() {
    println!("call `my_mod::function()`");
  }
  pub fn indirect_access() {
    print!("called `my_mod::indirect_access()`, that\n> ");
    private_function();
  }
}

fn main() {
  my_mod::function();
  my_mod::indirect_access();
}
