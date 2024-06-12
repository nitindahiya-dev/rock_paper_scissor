use rand::prelude::*;
use std::io;

fn main() {
    let guess_list = ["rock", "paper", "scissors"];

    loop {
        let random_choice: &str = guess_list[thread_rng().gen_range(0..guess_list.len())];

        println!("Please choose Rock, Paper or Scissors:");
        let mut user_input = String::new().trim().to_lowercase();
        io::stdin()
            .read_line(&mut user_input)
            .expect("failed to read user input");
        if !guess_list.contains(&user_input.as_str()) {
            println!("Invalid choice. Please try again.");
            continue;
        }

        match (user_input.as_str(), random_choice) {
            ("rock", "scissors") | ("paper", "rock") | ("scissors", "paper") => {
                println!("Congratulations! You win!");
            }
            ("rock", "paper") | ("paper", "scissors") | ("scissors", "rock") => {
                println!("You lose!");
            }
            _ => {
                println!("It's a draw!");
            }
        }

        println!("Do you want to play again? (yes/no)");

        let mut play_again = String::new().trim().to_lowercase();
        io::stdin()
            .read_line(&mut play_again)
            .expect("failed to read user input");
        if play_again != "yes" {
            break;
        }
    }
}
