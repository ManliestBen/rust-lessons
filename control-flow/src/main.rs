mod coding_challenge;

fn main() {

  // must use values of either true/false with 'if' statements
  // no concept of 'truthiness/falsiness' like some other languages
  // no ternary operator in rust

  let age = 42;

  if age >= 21 {
    println!("Let's get a beer.")
  } else {
    println!("Let's get a soda.")
  }

  even_or_odd(54);

  // match is like switch in other languages
  let eval = true;

  match eval {
    // pattern or arm
    true => {println!("matched true");}
    false => {println!("matched false");}
  }

  let drink = match age {
    21..=i32::MAX => "Beer",
    18..=20 => "Soda",
    _ => "Water"
  };
  // println!("You can have {drink}.");

  let mut seconds = 10;
  // loop {
  //   if seconds > 0 {
      // println!("{seconds} seconds left!");
  //     seconds -= 1;
  //   } else {
      // println!("Blastoff!");
  //     break;
  //   }
  // }

  while seconds > 0 {
    println!("{seconds} seconds left");
    seconds -= 1;
  }

  coding_challenge::main_test();
}


fn even_or_odd(num: i32) {
  let result = if num % 2 == 0 {
    "even"
  } else {
    "odd"
  };
  println!("The number is {result}")
}