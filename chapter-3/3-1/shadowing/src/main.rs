fn main() {
    let x = 5;

    // x is shadowed in this scope.
    let x = x + 1;

    {
        // Previous x is now shadowed by this new x in the this scope.
        let x = x * 2;
        println!("The value of x in the inner scope is : {}", x);
    }

    println!("The value of x is: {}", x);

    // Shadowing allows us to change types.
    let spaces = "    ";
    let spaces = spaces.len();

    println!("Value of spaces is: {}", spaces);
    
    // We however cannot do the same with mutability. Setting a variable to 
    // mutable means we can only mutate it's value, not type.
    // let mut spaces = "    ";
    // spaces = spaces.len(); Not valid.
}
