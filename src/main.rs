extern crate colored; // not needed in Rust 2018
use colored::*;
mod get_result;
mod handle_input;
//TODO: COLORIZE THE OUTPUT

fn main() {
    println!(
        "\n{}\n{}\n{}\n{}\n\n",
        "===================================================="
            .bold()
            .cyan(),
        "Rock!!!".italic().purple(),
        "Paper!!!".italic().blue(),
        "Scissors".italic().yellow()
    );

    // Player1
    let mut player1 = String::new();
    player1 = handle_input::handle_input(&mut player1, "player1", false);

    // Player2
    let mut player2 = String::new();
    player2 = handle_input::handle_input(&mut player2, "player2", true);

    get_result::get_result(&player1, &player2);

    println!(
        "{}",
        "\n\n====================== END =============================="
            .bold()
            .cyan()
    )
}
