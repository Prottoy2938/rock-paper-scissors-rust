pub fn handle_input(input: &mut String, player_name: &str, last_input: bool) {
    //TODO: minify this code
    if last_input {
        let mut counter = 0;
        while counter < 40 {
            println!("!!!===============NO CHEATING===============!!!");
            counter += 1;
        }
        println!("\n")
    }

    let mut done = false;
    //copying input value to the same name. To fix mutable reference problem
    let mut input = input;

    println!("====={}======\n", player_name);
    println!("Your Move: ");
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
                        "\nINVALID MOVE!, your move can be either 'Rock', 'Paper' or 'Scissors'.  Try again"
                    )
                }
            }
            Err(_e) => println!("\nSomething went wrong\n"),
        }
        //clearing input to start from an empty string
        input.clear();
    }
}
