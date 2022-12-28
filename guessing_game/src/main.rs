use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn game_loop(random_number: u64) {
    loop {
        println!("Guess the Number: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failes to read line");

        print!("You guessed {}", guess);

        let guess: u64 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You won");
                break;
            }
        };
    }
}

fn main() {
    loop {
        let random_number = rand::thread_rng().gen_range(1..=100);
        println!("{random_number}");
         
        game_loop(random_number);
         
        let mut answer = String::new();
        println!("Do you want to play again[Yes/no]");

        io::stdin().read_line(&mut answer)
            .expect("I quit anyway");

        match answer.as_str() {
            "Yes"|"yes"|"y"|"Y" => {
                continue;
            },
            "No"|"no"|"n"|"N" => {
                break;
            },
            _ => {
                println!("Not specified, quitting programm");
                break;
            }
        }

        // if answer == "Yes" || answer == "Y" || answer == "y"{
        //     continue;
        // } else if  answer == "No" || answer == "N" || answer == "n"{
        //     break;
        // } else {
        //     println!("Yes is default, playing again");
        //     continue;
    }
}
