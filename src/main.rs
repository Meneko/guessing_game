/*
Requirements
It is a CLI-based game, so you need to use the command line to interact with the game. The game should work as follows:

When the game starts, it should display a welcome message along with the rules of the game.
The computer should randomly select a number between 1 and 100.
User should select the difficulty level (easy, medium, hard) which will determine the number of chances they get to guess the number.
The user should be able to enter their guess.
If the user’s guess is correct, the game should display a congratulatory message along with the number of attempts it took to guess the number.
If the user’s guess is incorrect, the game should display a message indicating whether the number is greater or less than the user’s guess.
The game should end when the user guesses the correct number or runs out of chances.
Here is a sample output of the game:

Welcome to the Number Guessing Game!
I'm thinking of a number between 1 and 100.
You have 5 chances to guess the correct number.

Please select the difficulty level:
1. Easy (10 chances)
2. Medium (5 chances)
3. Hard (3 chances)

Enter your choice: 2

Great! You have selected the Medium difficulty level.
Let's start the game!

Enter your guess: 50
Incorrect! The number is less than 50.

Enter your guess: 25
Incorrect! The number is greater than 25.

Enter your guess: 35
Incorrect! The number is less than 35.

Enter your guess: 30
Congratulations! You guessed the correct number in 4 attempts.
To make the game more interesting, you can add the following features:

#DONE!Allow the user to play multiple rounds of the game (i.e., keep playing until the user decides to quit). You can do this by asking the user if they want to play again after each round.
#DONE!Add a timer to see how long it takes the user to guess the number.
#TODO!Implement a hint system that provides clues to the user if they are stuck.
#DONE!Keep track of the user’s high score (i.e., the fewest number of attempts it took to guess the number under a specific difficulty level).
*/

use std::{io::{self, Write}, time};
use rand::*;

fn hint(number_to_guess: u32, difficulty: &String) {
    let mut rng = rand::rng();
    let (between_left, between_right) = match difficulty.as_str() {
        "Easy" => { 
            (number_to_guess.saturating_sub(rng.random_range(1..=10)).max(1),
            (number_to_guess + rng.random_range(1..=10)).min(100))
        },
        "Medium" => {
            (number_to_guess.saturating_sub(rng.random_range(1..=25)).max(1),
            (number_to_guess + rng.random_range(1..=25)).min(100))
        },
        "Hard" => {
            (number_to_guess.saturating_sub(rng.random_range(1..=50)).max(1),
            (number_to_guess + rng.random_range(1..=50)).min(100))
        },
        _ => (0, 100),
    };

    println!("The number is between {} and {}", between_left, between_right);
}

fn main() {
    println!("Welcome to the Number Guessing Game!
I'm thinking of a number between 1 and 100.
You have 5 chances to guess the correct number.");

    let mut chances: u32; 
    let mut difficulty: String;
    let mut input = String::new();
    let mut rng = rand::rng();
    let mut highest_score: Option<u32> = None;
    let mut hints_uses: u32;

    'game: loop {
        let number_to_guess = rng.random_range(1..=100);

        println!("Please select the difficulty level:
1. Easy (10 chances)
2. Medium (5 chances)
3. Hard (3 chances)");

        loop {
            input.clear();

            print!("\nEnter your choice: ");
            io::stdout().flush().expect("erro no stdout");
            io::stdin()
            .read_line(&mut input)
            .expect("erro no stdin");

            if input.trim().parse::<u32>().is_err() {
                println!("Please just enter numbers!");
            }
            else {
                break;
            }
        }

        match input.trim() {
            "1" => {
                chances = 10;
                difficulty = "Easy".to_string();
                hints_uses = 3;
            },
            "2" => {
                chances = 5;
                difficulty = "Medium".to_string();
                hints_uses = 2;
            },
            "3" => {
                chances = 3;
                difficulty = "Hard".to_string();
                hints_uses = 1;
            },
            _ => {
                println!("Invalid choice, try again!");
                continue;
            }
        }

        println!("\nGreat! You have selected the {} difficulty level.
Let's start the game!", difficulty);

        let elapsed_time = time::Instant::now();

        for chance in 1..=chances {
            loop {
                input.clear();

                print!("\nEnter your guess or 'H' for a hint({}/3 hints to use): ", hints_uses);
                io::stdout().flush().expect("erro no stdout");
                io::stdin()
                .read_line(&mut input)
                .expect("erro no stdin");

                input = input.trim().to_string();

                if input.parse::<u32>().is_ok() {
                    break;
                }
                else if input.to_ascii_uppercase() == "H" && hints_uses > 0 {
                    hint(number_to_guess, &difficulty);
                    hints_uses -= 1;
                }
                else if hints_uses > 0 {
                    println!("Please just enter numbers or 'H' for a hint!");
                }
                else {
                    println!("Please just enter numbers!");
                }
            }

            let number_to_try = input.parse::<u32>().unwrap();

            if number_to_try == number_to_guess {
                println!("\nCongratulations! You guessed the correct number in {} attempts.", chance);
                if highest_score > Some(chance) || highest_score.is_none() {
                    highest_score = Some(chance);
                }
                break;
            }
            else if number_to_try < number_to_guess {
                println!("Incorrect! The number is greather than {}", number_to_try)
            }
            else if number_to_try > number_to_guess {
                println!("Incorrect! The number is less than {}", number_to_try)
            }

            if chance == chances {
                println!("\nYou lose the number was {}!", number_to_guess);
            }
        }

        let elapsed_seconds = elapsed_time.elapsed().as_secs();

        println!(
            "Highest Score (attempts to win): {}",
            match highest_score {
                Some(score) => score.to_string(),
                None => "None".to_string(),
            }
        );
        print!("Elapsed time: ");
        if elapsed_seconds >= 60 && elapsed_seconds <= 3600 {
                println!("{} minutes and {} seconds.", elapsed_seconds/60, elapsed_seconds%60)
        }
        else {
            println!("{} seconds", elapsed_seconds)
        }

        let mut continue_the_game: String = String::new();

        loop {
            continue_the_game.clear();

            print!("\nYou want to play again? (Y: Yes, N: No): ");
            io::stdout().flush().expect("erro no stdout");
            io::stdin()
            .read_line(&mut continue_the_game)
            .expect("Erro no stdin");

            continue_the_game = continue_the_game.trim().to_ascii_uppercase();

            if continue_the_game == "Y".to_string() {
                println!("");
                continue 'game;
            }
            else if continue_the_game == "N".to_string() {
                println!("Thanks for playing!");
                break 'game;
            }
            else {
                println!("Please just choose between 'Y' or 'N'");
            }
        }

    }
}
