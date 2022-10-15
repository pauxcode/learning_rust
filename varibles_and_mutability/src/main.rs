fn main() {
    // Variables
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6; // We’re allowed to change the value bound to x from 5 to 6 when mut is used.
    println!("The new value of x is: {}", x);

    // Constants
    // const mut y = 5; First, you aren’t allowed to use mut with constants.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds is: {}", {THREE_HOURS_IN_SECONDS});

    // Shadowing
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");
}
