use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number, folk!!");

    // create an immutable variable
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Secret number is {}", secret_number);

    // looping, isn't this crazy
    loop {
        // create a mutable variable
        println!("Please input the number you want to guess:");
        let mut guess = String::new();

        // read user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!!");

        // shadowing without mutating the variable above
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // match function, super cool
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!!"),
            Ordering::Greater => println!("Too big!!"),
            Ordering::Equal => {
                println!("You win folk!");
                break;
            }
        }
    }
}
