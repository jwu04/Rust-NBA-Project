use std::{time::{Instant}};
use std::collections::HashMap;
use regex::RegexSet;
use crate::parse_data::read_file::Player;
mod parse_data;
mod algo;
#[cfg(test)]
mod test;

// Welcome to this project!
// This part runs our breadth-first search on the dataset from
// https://www.kaggle.com/datasets/harisbeslic/nba-player-data-by-game-from-1949-to-2019?select=all_game_scores.csv
// The algorithm will find the first path that it sees that connects two players (if it can) and display it to the use
// The only parameters for you to change are when you input two players manually into the program
fn main() {
    // Timer for the program
    let start = Instant::now();
    // Parsing data
    let result = parse_data::read_file::read_player_data();
    match result {
        // If data can be parsed, try to run BFS on it
        Ok(players) => {
            let vertices = take_input(&players);
            let graph = algo::search::bfs(&players, vertices.0, vertices.1);
            match graph {
                Ok(bfs_graph) => {
                    println!("{}", bfs_graph);
                    let duration = start.elapsed();
                    println!("----------------------------------\n====== Search completed in: {:?}", duration);   
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        }
        // Display error otherwise
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}

// Allows user input
// Any bad input leads to the program automatically selecting two NBA players at random and trying to connect them
fn take_input(players: &HashMap<i32, Player>) -> (i32, i32) {
    let mut line = String::new();
    println!("====== Do you want to input two basketball players? (y/n): ");
    std::io::stdin().read_line(&mut line).unwrap();
    // Using regex to determine if you inputted a 'yes' or 'no'
    let re = RegexSet::new(&[r"^n*?o*$", r"y+?e*s*$"]).unwrap();
    let mut lowercase = line.to_lowercase();
    let remove_nl = remove_newline(&mut lowercase);
    let result = re.matches(&remove_nl);
    // If you said no, randomly selected two NBA players and try to connect them
    if result.matched(0) {
        let rng = algo::search::gen_ids(players);
        return (rng.0, rng.1)
    }
    // Otherwise, you input two NBA players you want to connect and verifies that these players are real
    else if result.matched(1) {
        let mut player_a = String::new();
        println!("====== Input the first basketball player (from 1949-2019):");
        std::io::stdin().read_line(&mut player_a).unwrap();
        let p1_id = algo::search::find_id(&players, player_a.strip_suffix("\r\n").unwrap().to_string());
        if p1_id != -1 {
            let mut player_b = String::new();
            println!("====== Input the second basketball player (from 1949-2019):");
            std::io::stdin().read_line(&mut player_b).unwrap();
            let p2_id = algo::search::find_id(&players, player_b.strip_suffix("\r\n").unwrap().to_string());
            if p2_id != -1 {
                return (p1_id, p2_id)
            }
            else {
                return invalid_input(players)
            }
        }
        else {
            return invalid_input(players)
        }
    }
    else {
        return invalid_input(players)
    }
}

// Helper method to display that you inputted an invalid answer and auto-selects two random players to connect
fn invalid_input(players: &HashMap<i32, Player>) -> (i32, i32) {
    println!("====== Did not get valid input, generating two random players from 1949-2019...");
    let rng = algo::search::gen_ids(players);
    return (rng.0, rng.1)
}

// Helper method to remove trailing newlines (for cross OS support)
fn remove_newline(line: &mut String) -> &str {
    if line.ends_with('\n') {
        line.pop();
        if line.ends_with('\r') {
            line.pop();
        }
    }
    return line
}