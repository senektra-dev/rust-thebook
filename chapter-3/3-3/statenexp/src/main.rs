fn main() {
    // Statement:
    let x = 6;

    // A statement
    let y = {
        // This code block ends with an expression, hence the no semi colon
        let z = 7;
        z + x
    };

    println!("The value of y is: {}", y);
}
