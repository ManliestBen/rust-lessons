/*
Declare an i32 variable assigned to 1337.
Use the underscore character to add a visual
separator between the numbers.

Cast the i32 to an i16 integer and assign the result
to a separate variable.

Declare a floating-point value of your choosing.
Print out the number with 3 digits of precision.

Declare a 'with_milk' variable set to a Boolean.
Declare a 'with_sugar` variable set to a Boolean.

Declare a 'is_my_type_of_coffee` variable. It should
be set to true if the coffee has both milk and sugar.

Declare an `is_acceptable_coffee` variable. It should
be set to true if the coffee has either milk or
sugar.

Declare an array with four i8 integers of your choosing
Print out the array in its Debug representation.

Declare a tuple consisting of the integer, float,
a Boolean, and the array that you previously declared.
Print out the tuple in its Debug representation.
*/

pub(crate) fn main_test() {
  let num_1 = 1_337;
  let num_2 = num_1 as i16;
  let num_3 = 4.567;
  let with_milk = true;
  let with_sugar = false;
  let is_my_type_of_coffee = with_milk && with_sugar;
  let is_acceptable_coffee = with_milk || with_sugar;
  let num_array: [i8; 4] = [1, 2, 3, 4];
  dbg!(num_array);
  let fancy_tuple = (num_1, num_3, with_milk, num_array);
  dbg!(fancy_tuple);
}