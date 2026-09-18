fn main() {
  let mut x = 5;
  println!("The value of x is: {}", x);
  x = 6;
  println!("The value of x is: {}", x);
  // shadowing
  let x = x + 1;

  {
    let x = x * 3;
    println!("The value of x in the inner scope is: {x}");
  }
  //data types
  println!("The value of x is: {x}");
  let _z = 2.0; // f64
  let _y: f32 = 3.0; // f32

  // statements and expressions
  another_function(5);
  print_labeled_measurement(5, 'h');

  let y = {
    let x = 3;
    x + 1
  };

  println!("The value of y is: {y}");


  let x = plus_one(5);

  println!("The value of x is: {x}");

  //if statements and loops
  let number = 6;

  if number % 4 == 0 {
    println!("number is divisible by 4");
  } else if number % 3 == 0 {
    println!("number is divisible by 3");
  } else if number % 2 == 0 {
    println!("number is divisible by 2");
  } else {
    println!("number is not divisible by 4, 3, or 2");
  }

  let mut count = 0;
  'counting_up: loop {
    println!("count = {count}");
    let mut remaining = 10;

    loop {
      println!("remaining = {remaining}");
      if remaining == 9 {
        break;
      }
      if count == 2 {
        break 'counting_up;
      }
      remaining -= 1;
    }

    count += 1;
  }
  println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
      println!("{number}!");

      number -= 1;
    }

    println!("LIFTOFF!!!");



    let a = [10, 20, 30, 40, 50];

    for element in a {
      println!("the value is: {element}");
    }

    for number in (1..4).rev() {
      println!("{number}!");
  }
    println!("LIFTOFF!!!");

  }




//functions
fn another_function(x: i32) {
  println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
  x + 0
}
fn print_labeled_measurement(value: i32, unit_label: char) {
  println!("The measurement is: {value}{unit_label}");
}
