use rand::Rng;
use std::io;
use std::cmp::Ordering;

pub(crate) fn guessNumber() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Please input your guess.");


    loop {
        //创建一个变量 带mut的是可变， 不带mut不可变， mutate吧应该是
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // match 类似于when， 变量可以被重新声明，，，，
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