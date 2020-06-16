extern crate colored; // not needed in Rust 2018
use colored::*;

pub fn handle_input(input: &mut String, player_name: &str, last_input: bool) -> String {
    if last_input {
        println!(
            "{}",
            "!!!===============NO CHEATING===============!!!\n"
                .repeat(60)
                .italic()
                .red()
        );
    }

    let mut done = false;
    //copying input value to the same name. To fix mutable reference problem
    let mut input = input;

    println!("------{}------\n", player_name.bold());
    println!("{}", "Your Move:".bold().blink(),);
    //looping until user inputs valid game move
    while !done {
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                //removing '\r\n' from the input
                if let Some('\n') = input.chars().next_back() {
                    input.pop();
                }
                if let Some('\r') = input.chars().next_back() {
                    input.pop();
                }

                //checking for valid game move
                if input.to_lowercase() == "rock"
                    || input.to_lowercase() == "scissors"
                    || input.to_lowercase() == "paper"
                {
                    //breaking the loop
                    done = true;
                    println!("\n");
                } else {
                    println!(
                        "\n{}, your move can be either {}, {} or {}.  Try again:",
                        "INVALID MOVE!".bold().red(),
                        "'rock'".italic().purple(),
                        "'paper'".italic().blue(),
                        "'scissors'".italic().yellow()
                    );

                    //clearing input to start from an empty string

                    input.clear();
                }
            }
            Err(_e) => println!("\nSomething went wrong\n"),
        }
    }
    input.to_lowercase()
}
