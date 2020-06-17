use crate::store_output;

pub fn handle_input(input: &mut String, player_name: &str, last_input: bool) -> String {
    if last_input {
        store_output::no_cheating();
    }

    //copying input value to the same name. To fix mutable reference problem
    let mut input = input;

    store_output::player_turn(player_name);

    let mut done = false;
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
                    //Invalid move details
                    store_output::invalid_move();

                    //clearing input to start from an empty string
                    input.clear();
                }
            }
            Err(_e) => println!("\nSomething went wrong\n"),
        }
    }
    input.to_lowercase()
}
