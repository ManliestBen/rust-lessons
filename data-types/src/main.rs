mod coding_challenge;
#[allow(unused_variables)]

fn main() {
  // types of variables
  // scalar types - single values
    // 4 types (integers, floating-points, Booleans, characters)
  
  // integers
    // signed (positive or negative)
    let signed_int: i32 = -23;
    // number after i is bits (32 bits)
    // bit is smallest unit of computer memory (0 or 1)
    // 8 bits is a byte
    // 32 bits = 4 bytes
    // unsigned (zero or positive - can store a larger value in positive direction)
    let unsigned_int: u32 = 4389;

    // f32 has 6-9 digits of precision
    // f64 has 15-17 digits of precision
    let float_num: f32 = 3.23980;

    // another way to define a type
    let some_value = -3i8;

    // underscores can be used as separators (like commas)
    let big_num: i32 = 32_234_234;

    // usize and isize are aliases for existing types
      // varies based on what computer is running the program
    let some_num: usize = 435; // will be u32 on 32bit architecture, u64 on 64-bit architecture

  // string literal (written directly into source code)
  let name = "Ben";

  // special characters
  
  // println!("Make a line break\nlike this");
  // println!("Make a tab\tlike this");
  // println!("Use double quotes \"like this\"");
  // println!("Use backslashes \\ like this");
  // raw strings can be used instead for the above example

  // println!(r"Use backslashes \ like this");

  // how to use methods built into values
  // println!("{}", some_value.abs());

  let dirty_string = "    some text        ";
  // println!("{}", dirty_string.trim());

  // println!("{}", some_value.pow(3));

  // floating point number (15-17 digits of precision)
  let pi: f64 = 3.14159;

  // must explicitly declare type for these methods to work!
  // println!("{}", pi.floor());
  // println!("{}", pi.ceil());
  // println!("{}", pi.round());
  
  // format specifier
  // println!("The current value of pi is {pi:.2}");
  // or 
  // println!("The current value of pi is {:.2}", pi);
  
  // casting to a type
  let miles_away = 50;
  let miles_away_i8: i8 = miles_away as i8;
  let miles_away_u8: u8 = miles_away as u8;
  
  let pi_f32 = pi as f32;
  let pi_without_decimals = pi as i32;

  // math operations
  let addition = 5 + 4;
  let subtraction = 5 - 4;
  let multiplication = 5 * 4;
  let floor_division = 5 / 4; 
  let decimal_division = 5f64 / 4f64;
  let other_decimal_division = 5.0 / 4.0;

  let remainder = 7 % 2;
  // println!("{remainder}")

  // augmented assignment operator
  let mut year = 2025;
  year += 1;
  println!("{year}");
  // ++ and -- not included in Rust


  // Booleans
  let is_yummy = true;

  // println!("{is_yummy}");
  
  let age: i32 = 24;
  let can_buy_booze = age >= 21;
  
  // println!("{can_buy_booze}");
  // flip a value
  // println!("{}", !can_buy_booze)
  // must explicitly declare type for these methods to work
  // println!("{} {}", age.is_positive(), age.is_negative());

  // equality and inequality
  // println!("{} {}", 3 == 4, 3 != 4);
  // println!("{}", "Taco" == "taco");

  let ben_age = 42;
  let ben_name = "ben";
  let can_ben_buy_booze = ben_age >= 21 && (ben_name == "Ben" || ben_name == "ben");
  // println!("{}", can_ben_buy_booze);

  // character type
  let first_initial = 'B';
  let rocket_emoji = '🚀';

  // println!("{} {}", rocket_emoji.is_alphabetic(), rocket_emoji.is_alphanumeric());
  // println!("{} {}", rocket_emoji.is_ascii_hexdigit(), rocket_emoji.is_uppercase());

  // compound type - array
  // array is a fixed-size collection of homogenous data
  // arrays will not grow or shrink during program execution

  // let empty_array = []; <-- This will not work!
  let empty_array: [i32; 0] = [];
  let mut some_numbers = [1, 2, 3];
  let letters = ['A', 'B', 'C'];

  // println!("{}", some_numbers.len());
  // println!("{}", some_numbers[1]);
  some_numbers[1] = 4;
  // println!("{}", some_numbers[1]);

  // traits
  // like a contract that requires that a type support one or more methods
  // they establish consistency between types
  // a type 'implements' a trait
  // similar to interfaces in TS
  // display trait requires that a type can be represented as a user-friendly, readable string
  // arrays don't implement the display trait

  // println!("{}", some_numbers); <-- will not work
  // debug trait allows this
  // println!("{:?}", some_numbers);
  // or
  // println!("{some_numbers:?}");
  // or
  // println!("{some_numbers:#?}");  // called pretty print

  // debug macro can give even more useful information
  // dbg!(some_numbers);

  // tuples are similar to arrays, but don't require homogenous data
  let person = ("Ben", 42, "silly");
  // dot notation used to access index
  let person_name = person.0;
  let person_age = person.1;
  let person_trait = person.2;
  // similar to unpacking!
  let (other_name, other_age, other_trait) = person;
  // dbg!(person);

  // range is a sequence of consecutive values
  let num_range = 1..10;
  let inclusive_num_range = 1..=10;
  // dbg!(num_range);  // <-- not super useful
  // iteration
  for i in num_range {
    // println!("{}", i);
  }

  let letters = 'a'..='z';

  for letter in letters {
  //   println!("{}", letter);
  }

  // generics
  // generic is a type argument
  // generic type will act as a placeholder for a future type
  let month_days: std::ops::RangeInclusive<i32> = 1..=31;
  coding_challenge::main_test();
  
}
