use colored::*;
pub fn greetings() {
    println!(
        "\n{}\n{}\n{}\n{}\n\n",
        "===================================================="
            .bold()
            .cyan(),
        "Rock!!!".italic().purple(),
        "Paper!!!".italic().blue(),
        "Scissors".italic().yellow()
    );
}

pub fn end_game() {
    println!(
        "{}",
        "\n\n====================== END ============================"
            .bold()
            .cyan()
    )
}

pub fn no_cheating() {
    println!(
        "{}",
        "!!!===============NO CHEATING===============!!!\n"
            .repeat(60)
            .italic()
            .red()
    );
}

pub fn invalid_move() {
    println!(
        "\n{}, your move can be either {}, {} or {}.  Try again:",
        "INVALID MOVE!".bold().red(),
        "'rock'".italic().purple(),
        "'paper'".italic().blue(),
        "'scissors'".italic().yellow()
    );
}

pub fn player_turn(player: &str) {
    println!("------------{}------------\n", player.bold());
    println!("{}", "Your Move:".bold().blink(),);
}

pub fn play_again() {
    println!("\n\n{}", "Do you want to play again? Y/N".bold().purple())
}
