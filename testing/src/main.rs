fn main() {
    let x = 5;

    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x outside the inner scope is: {x}");
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // must have the u32
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is {guess}");

    // let quotient = 56.7 / 32.2;
    // let truncated = -5 / 3; // Results in -1
    // let trunq = -5.0 / 3.0;     5.0 / 3 -> throws error
    // println!{"The value {quotient}, {truncated}, {trunq}"}

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    // let x: (i32, f64, u8) = (500, 6.4, 1);
    // let five_hundred = x.0;
    // let six_point_four = x.1;
    // let one = x.2;

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    // let mut index = String::new();

    // std::io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read line");

    // let index: usize = index
    //     .trim()
    //     .parse()
    //     .expect("Index entered was not a number");

    // let element = a[index];

    // println!("The value of the element at index {index} is: {element}");
    
}
