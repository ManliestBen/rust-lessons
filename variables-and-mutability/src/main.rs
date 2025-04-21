mod coding_challenge;

fn main() {
  let num_tortillas = 5;
  let num_beef_slices = 10;
  let my_name ="Ben";

  // this is an example of shadowing
  let num_tortillas = 10;
  println!("I have {} tortillas", num_tortillas);
  // shadowing is different from mutability
  // shadowing allows us to reuse the same name for a new variable with a different value
  // mutability allows us to change the value of a variable

  let mut counter = 0; // mut is used to make the variable mutable
  counter = counter + 1;
  println!("Counter: {}", counter);
  
  let unused_variable = 10;  // unused variable will throw a warning
  let _unused_variable = 10;  // this unused variable will not throw a warning

  #[allow(unused_variables)]  // compiler directive to suppress the warning
  let another_unused_variable = 10;
  
  // example of interpolation
  println!("Hello, {}", my_name);
  
  // another example of interpolation
  println!("Hello, {0}", my_name);
  
  // another example of interpolation
  println!("Hello, {my_name}");

  // example of concatenation
  println!("I have {} tortillas and {} beef slices", num_tortillas, num_beef_slices);
  
  // another example of concatenation
  println!("I have {0} tortillas and {1} beef slices", num_tortillas, num_beef_slices);
  
  // another example of concatenation
  println!("I have {num_tortillas} tortillas and {num_beef_slices} beef slices");

  // example of function call
  print_name(my_name);

  coding_challenge::main_test();
}

fn print_name(name: &str) {
  println!("Hello, {}", name);
}