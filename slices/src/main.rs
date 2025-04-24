mod coding_challenge;
fn main() {
  // slice is a reference to a portion/sequence of a collection type
  // string slice is a reference of a sequence of characters from a string
  // array slice is reference of a sequence of elements in an array
  // slice does not take ownership of the collection
  let actor = String::from("Noah Wyle");
  
  // borrows the whole string value
  // let string_reference = &actor;

  let first_name = &actor[..4];
  // let first_name = &actor[0..4]; // <== same as line above
  let last_name = &actor[5..];
  // let last_name = &actor[5..9]; // <== same as line above
  println!("{first_name}");
  println!("{last_name}");
  let full_name = &actor[..]; // <== makes a copy of the full string

  // length of string slice refers to byte length, not characters
  let food = "pizza";
  println!("{}", food.len());
  let piece_of_food = &food[0..3]; // <== works because English letters are one byte
  println!("{}", piece_of_food.len());
  
  let other_food = "🍕";
  println!("{}", other_food.len()); // <== returns 4
  // let piece_of_other_food = &other_food[0..3]; // <== will not work because valid collection is 0-4
  // println!("{}", piece_of_other_food.len());

  do_hero_stuff(&actor);

  let values = [1, 5, 9, 11, 16, 23];

  let slice = &values[..3];
  println!("{slice:?}");

  let regular_reference = &values;
  let slice_of_three = &values[..3];

  print_length(regular_reference);
  print_length(slice_of_three);

  let mut some_array = [15, 65, 45, 78, 89];
  let some_slice = &mut some_array[2..4];
  println!("{:?}", some_slice);
  some_slice[0] = 22;
  println!("{:?}", some_slice);
  println!("{:?}", some_array);

  coding_challenge::main();
}

fn do_hero_stuff(hero_name: &str) {
  println!("{hero_name} saves the day!");
}

fn print_length(reference: &[i32]) {
  println!("{}", reference.len());
}