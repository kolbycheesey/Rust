fn main() {
    let x = 5;

    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scop is: {x}")
    }

    println!("The value of x outside the inner scope is: {x}")
}