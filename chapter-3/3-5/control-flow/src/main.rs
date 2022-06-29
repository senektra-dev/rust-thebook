fn main() {
    let number = 3;

    // Blocks of code associated with if expressions are sometimes called arms.
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // If you don't want to use an else expression, you can use a single-line if expression.
    println!("{}", if number > 4 { "yes" } else { "no" });

    // Rust has a special syntax for if expressions that allows you to use a block 
    // as the last expression. This is called a `let` expression.
    let number = if number % 4 == 0 { 4 } else { 5 };
    println!("{}", number);
}
