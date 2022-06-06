fn main() {
    // x is immutable, cannot reassign.
    let x = 5;
    let mut y = 10;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    y = 6;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {}", THREE_HOURS_IN_SECONDS);
}
