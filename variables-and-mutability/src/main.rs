fn main() {
  let num_tortillas = 5;
  let num_beef_slices = 10;
  let my_name ="Ben";
  
  let unused_variable = 10;  // unused variable will throw a warning
  let _unused_variable = 10;  // this unused variable will not throw a warning

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

}
