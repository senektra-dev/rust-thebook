fn main() {
    // Integers
    let x: u32 = 145;
    let y = 58u8;
    let z = b'A';

    println!("Respectively, x, y, z: {}, {}, {}", x, y, z);

    // Floats
    let x = 2.0;
    let y: f32 = 3.0;

    println!("x, y: {}, {}", x, y);

    // Math ops
    let q1 = 2 / 3; // 0
    let q2 = 2.0 / 3.0; // 0.66
    let q3 = 5 / 2; // 2

    println!("q1, q2, q3: {}, {}, {}", q1, q2, q3);

    // Booleans
    let t = true;
    let f: bool = false;

    println!("t, f: {}, {}", t, f);

    // Chars
    let c = 'c';
    let z = 'Z';
    // Chars are Unicode scalars
    let heart_eyed_cat = '😻';

    println!("c, z, heart_eyed_cat: {}, {}, {}", c, z, heart_eyed_cat);

    // Tuples
    let tup1: (u32, f64, char) = (1024, 0.3434234, '🦉');
    let tup2 = ('a', 100, 0.5);

    // Use a pattern to destructure tup2
    let (x, y, z) = tup1;
    println!("The value of x, y, z: {}, {}, {}", x, y, z);

    // You can access values with the . operator and appropriate index
    println!("The value of tup2: ({}, {}, {})", tup2.0, tup2.1, tup2.2);

    // The unit value '()' has the type called 'unit type'. Its value is 'unit value'

    // Arrays must have the same type for every element
    let arr = [5; 5]; // Short hand for [5, 5, 5, 5, 5]
    // Arrays are good for when you want your data allocated on the stack rather
    // than on the heap or for when you need to keep track of a fixed number of
    // elements. Otherwise use vectors.
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    
    println!("The first value of array, 'arr': {}", arr[0]);
    println!("The last value of array, 'months': {}", months[months.len() - 1]);
}
