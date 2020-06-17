fn main() {
    println!("Hello, world!");

    another_function(4, 7);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value if y is: {}", y);

    let z = five();
    let w = plus_one(z);

    println!("Result from function five: {}", z);
    println!("Result from plus_one function: {}", w);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x+1
}
