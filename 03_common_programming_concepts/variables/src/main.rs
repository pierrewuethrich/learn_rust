fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let x = x + 6; //Shadowing
    println!("The value of x is: {}", x);
}
