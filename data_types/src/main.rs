fn main() {
    // integer types
    let guess: u8 = "42".parse().expect("Not a number!");
    println!("{guess}");

    let decimal: u32 = 32_321;
    println!("{decimal}");


    // floating types
    let x = 2.0; // 64;
    let y: f32 = 3.0; // 32


    // numeric operations
    // addition
    let sum = 5 + 10; // i32

    // subtraction
    let difference = 95.5 - 4.3; // f64

    // multiplication
    let product = 4 * 30; // i32

    // division
    let quotient = 56.7 / 32.2; // f64
    let floored = 2 / 3; // Results in 0 i32

    // remainder
    let remainder = 43 % 5; // i32


    // boolean types
    let t = true; // bool

    let f: bool = false;


    // character type
    let c = 'z'; // char
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻'; // char
    

    // tuple type
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // The variable tup binds to the entire tuple, 
    // because a tuple is considered a single compound element.
    // To get the individual values out of a tuple, we can use pattern 
    // matching to destructure a tuple value, like this:
    let (x, y, z) = tup;
    // We can also access a tuple element directly by using a period (.)
    // followed by the index of the value we want to access. For example:
    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;


    // array type
    // Another way to have a collection of multiple values is with an array. 
    // Unlike a tuple, every element of an array must have the same type.
    let a = [1, 2, 3, 4, 5];
    // You write an array’s type using square brackets with the type of each 
    // element, a semicolon, and then the number of elements in the array, 
    // like so:
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    // You can also initialize an array to contain the same value for each 
    // element by specifying the initial value, followed by a semicolon, 
    // and then the length of the array in square brackets, as shown here:
    let c = [3; 5]; // c = [3, 3, 3, 3, 3]
    // An array is a single chunk of memory of a known, 
    // fixed size that can be allocated on the stack. 
    // You can access elements of an array using indexing, like this:
    let first = a[0]; // 1
    let second = a[1]; // 2
}
