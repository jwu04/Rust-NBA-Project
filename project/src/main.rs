use std::{time::{Instant}};
use std::collections::HashMap;
use regex::RegexSet;
use crate::parse_data::read_file::Player;
mod parse_data;
mod algo;
#[cfg(test)]
mod test;

fn main() {
    let start = Instant::now();
    let result = parse_data::read_file::read_player_data();
    match result {
        Ok(players) => {
            let vertices = take_input(&players);
            let graph = algo::search::bfs(&players,  vertices.0, vertices.1);
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
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}

fn take_input(players: &HashMap<i32, Player>) -> (i32, i32) {
    let mut line = String::new();
    println!("====== Do you want to input two basketball players? (y/n): ");
    std::io::stdin().read_line(&mut line).unwrap();
    let re = RegexSet::new(&[r"^n*?o*$", r"y+?e*s*$"]).unwrap();
    let result = re.matches(&line.strip_suffix("\r\n").unwrap().to_lowercase());
    if result.matched(0) {
        let rng = algo::search::gen_ids(players);
        return (rng.0, rng.1)
    }
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

fn invalid_input(players: &HashMap<i32, Player>) -> (i32, i32) {
    println!("====== Did not get valid input, generating two random players from 1949-2019...");
    let rng = algo::search::gen_ids(players);
    return (rng.0, rng.1)
}