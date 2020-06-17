extern crate colored; 
use colored::*;

pub fn get_result(player1: &String, player2: &String) {
    if player1 == player2 {
        println!("Its a draw")
    } else if player1 == "rock" && player2 == "scissors" {
        println!("Player1 wins")
    } else if player1 == "scissors" && player2 == "paper" {
        println!("Player1 wins")
    } else if player1 == "paper" && player2 == "rock" {
        println!("Player1 wins")
    } else {
        println!("{}{}", "Player2".bold().green(), " wins".italic().green())
    }
}
