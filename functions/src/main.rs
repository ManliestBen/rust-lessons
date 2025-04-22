mod coding_challenge;

fn main() {
  // do_stuff("eat cheese");
  // do_other_stuff();
  // do_stuff("cook dinner");
  // do_even_more_stuff();
  let calc = double(4);
  let calc_2 = triple(3);
  // dbg!(calc_2);

  // a unit is an empty tuple without values
  let unit_example = ();
  // a unit is returned by default by a function if nothing else is
  // returned either implicitly or explicitly

  {
    // this is a nested scope and can see variables within the main fn scope
    let some_value = {
      calc + calc_2
    };
    // println!("{some_value}");
  }
  coding_challenge::main_test();
}

fn do_stuff(thing_to_do: &str) {
  println!("do_stuff invoked, doing {thing_to_do}");
}

fn do_other_stuff() {
  println!("do_other_stuff invoked");
}

fn do_even_more_stuff() {
  println!("do_even_more_stuff invoked");
}

fn double(number: i32) -> i32 {
  return number + number;
}

fn triple(number: i32) -> i32 {
  // implicitly returns last line
  // but it cannot have a semicolon
  number + number + number
}


