mod handle_input;

//TODO: COLORIZE THE OUTPUT

fn main() {
    println!("\n========================================\nRock!!!\nPaper!!\nScissors!!!\n");

    // Player1
    let mut player1 = String::new();
    handle_input::handle_input(&mut player1, "player1", false);

    // Player2
    let mut player2 = String::new();
    handle_input::handle_input(&mut player2, "player2", true);

    println!("================== END ======================")
}
