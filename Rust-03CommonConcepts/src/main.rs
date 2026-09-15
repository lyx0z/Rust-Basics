fn main() {
  let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    // shadowing
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    //data types
    println!("The value of x is: {x}");
    let _z = 2.0; // f64
    let _y: f32 = 3.0; // f32
}
