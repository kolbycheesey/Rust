fn main() {
    println!("Hello, world!"); // first print

    let z = another_function(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };
    println!("Hi"); //third print
    println!("The value of y is: {y}, {z}"); //-> 4, 6 //fourth print

    if z < 5 {
        println!("{z} is less than 5");
    } else {
        println!("{z} is either equal to or greater than 5");
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

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

    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;
    // while index < 5 {
    //     println!("the value is: {}", a[index]);
    //     index += 1;
    // }
    // for element in a {
    //     println!("the value is: {element}");
    // }
    // for number in (1..4).rev() {
    //     println!("{number}!");
    // }
    // println!("LIFTOFF!!!");
}

fn another_function(x:i32, c:char) -> i32 {
    println!("Another function x: {x}{c}."); //second print
    6
}
