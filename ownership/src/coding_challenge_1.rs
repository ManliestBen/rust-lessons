/*
Declare a `is_concert` variable set to a boolean.
Declare a `is_event` variable assigned to `is_concert`.
Will Rust move ownership? State your answer, then confirm
by trying to printing both variables out.

Declare a `sushi` variable to set to a string literal of "Salmon"
Declare a `dinner` variable assigned to the `sushi` variable.
Will Rust move ownership? State your answer, then confirm
by trying to printing both variables out.

Repeat the previous example but use a heap String instead.
Will Rust move ownership? Explain why the result is different
from the previous operation.

The `clear` method modifies a heap String to have no content.
Declare an `eat_meal` function that accepts a `meal` parameter
of type String. In the body of `eat_meal`, invoke the `clear`
method on the `meal` parameter.

In the `main` function, invoke the `eat_meal` function and pass
in your "Salmon" String. Explain what happens when the eat_meal
function runs. Describe the complete movement of ownership of
the "Salmon" String throughout the program.

Say we want to keep the String around after `eat_meal` is
called. How can we continue to have access to the String in
the `main` function? Print out the (empty) String.
*/

pub(crate) fn main() {
  let is_concert = true;
  // ownership is not moved because booleans implements the Copy trait
  let is_event = is_concert;
  println!("{}", is_concert);
  println!("{}", is_event);

  let sushi = "Salmon";
  // ownership is not moved because strings implement the Copy trait
  let dinner = sushi;
  println!("{}", sushi);
  println!("{}", dinner);
  
  let more_sushi = String::from("Salmon");
  // ownership is moved because ownership is transferred to more_dinner
  let more_dinner = more_sushi;
  // println!("{}", more_sushi);
  println!("{}", more_dinner);

  let meal = String::from("Salmon");
  let meal_after_eating = eat_meal(meal);
  // meal is moved to eat_meal, so it is not available to print
  // println!("{}", meal);
  println!("{}", meal_after_eating);
}

fn eat_meal(mut meal: String) -> String {
  meal.clear();
  println!("{}", meal);
  return meal;
}