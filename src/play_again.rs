use crate::store_output;

pub fn play_again() -> bool {
    store_output::play_again();
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            //removing '\r\n' from the input
            if let Some('\n') = input.chars().next_back() {
                input.pop();
            }
            if let Some('\r') = input.chars().next_back() {
                input.pop();
            }

            input = input.to_lowercase();
            //adding additional check
            if input == "y" || input == "yes" {
                return true;
            } else {
                return false;
            }
        }
        Err(_e) => return false,
    }
}
