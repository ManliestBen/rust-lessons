mod coding_challenge_1;
// ownership is a compiler feature - set of rules on how Rust manages memory
  // checks to see that program is free of memory errors
  // compiler doesn't compile a program if ownership rule is violated
  // every value in a program has an owner - variables and parameters can be owners
  // tuples and arrays own their own values

// stack and heap are two different parts of computer's memory
  // each writes/reads data in different ways
  // stack is faster, but it requires static data
  // heap is slower, but can handle dynamic data
  // stack is LIFO
  // stack data must be fixed, consistent size known at compile time

// heap is large area of storage space
  // data with size not known at compile time
  // program called 'memory allocator' finds an empty spot in the heap for data
  // memory allocator returns a reference, which is an address to the value
  // that reference can be stored in a variable, which would be a fixed size, storable on the heap

fn main() {

  let age = 42; // age is owner of value 42
  // owner deallocates memory when it goes out of scope

  // copy trait mandates that a type can be copied
  let time = 2025; // time is responsible for cleaning up its value of 2025
  let year = time; // year is responsible for cleaning up its value of 2025
  
  // println!("{year}");

  let food = "cheese";
  let other_food = food;
  // Strings are stored on the heap
  let snack = String::new();
  let veggie = String::from("broccoli");

  let mut name: String = String::from("Ben");
  println!("{name}");
  name.push_str("jamin");
  println!("{name}");

  // move is the transfer of ownership from one owner to another
  let person: String = String::from("Ben");
  let second_person: String = String::from("David");
  // here, ownership moves from person to genius
  let genius = person;

  // println!("{person}");

  // drop deallocates memory on the heap
  // drop is called behind the scenes when a variable goes out of scope
  drop(genius);
  // let smarty_pants = genius; // won't work because it was dropped

  // if you want to duplicate heap data (and not move ownership), use clone
  let smarty_pants = second_person.clone();

  // borrowing is to use something without taking ownership
  // & is the borrow operator
  let stack_value = 2;
  let integer_reference = &stack_value;
  
  let heap_value = String::from("Ben");
  let heap_reference = &heap_value;

  // * is the dereference operator
  // it is used to access the data stored at a reference
  println!("{}", *heap_reference);

  let burger = String::from("Burger");
  add_fries(burger);

  let cake = bake_cake();
  println!("I now have a {} cake", cake);

  coding_challenge_1::main();

  let drink = String::from("Snapple");
  let beverage = drink;
  let delight = &beverage;
  println!("{beverage}");
}

fn add_fries(mut meal: String) {
  meal.push_str(" and fries");
}

fn bake_cake() -> String {
  let cake = String::from("velvet");
  return cake;
}