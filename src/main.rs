mod get_result;
mod handle_input;
mod store_output;

fn main() {
    store_output::greetings();
    // Player1
    let mut player1 = String::new();
    player1 = handle_input::handle_input(&mut player1, "player1", false);

    // Player2
    let mut player2 = String::new();
    player2 = handle_input::handle_input(&mut player2, "player2", true);

    get_result::get_result(&player1, &player2);

    store_output::end_game();
}
