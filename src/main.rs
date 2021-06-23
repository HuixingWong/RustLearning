use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Please input your guess.");


    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // match 类似于when
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please input a number");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => { println!("{} too small, the result is {}", guess, "你猜") }
            Ordering::Greater => { println!("{} too big {} ", guess, "你猜") }
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
