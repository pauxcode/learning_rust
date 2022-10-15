use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Secret 8)
    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
    
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u8 = match guess
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Equal => {
                println!("You win!!!");
                break;
            },
            std::cmp::Ordering::Greater => println!("Too big!"),
        }
    }
}
