fn main() {
    println!("Hello, world!");

    another_function();

    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn another_function() {
    println!("Another function.");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1 // most functions return the last expression implicitly.
          // Without semicolons
}
