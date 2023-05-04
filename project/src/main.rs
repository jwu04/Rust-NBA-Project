use std::{time::{Instant}};
use rand::prelude::*;
use std::collections::HashMap;
use regex::Regex;
use crate::algo::search::get_size;
use crate::parse_data::read_file::Player;
mod parse_data;
mod algo;

fn main() {
    let start = Instant::now();
    let mut players = parse_data::read_file::read_player_data();
    let input = take_input(&players.as_ref().unwrap());
    if let Result::Ok(data) = players {
        players = parse_data::read_file::read_game_data(data.clone());
    }
    if let Result::Ok(data) = players {
        let graph = algo::search::bfs(data, input.0, input.1);
        println!("{}", graph);
    }
    let duration = start.elapsed();
    println!("\nSearch completed in: {:?}", duration);
}

fn take_input(players: &HashMap<i32, Player>) -> (i32, i32) {
    let mut line = String::new();
    println!("====== Do you want to input two basketball players? (y/n): ");
    std::io::stdin().read_line(&mut line).unwrap();
    let no_re = Regex::new(r"^n+?o*$").unwrap();
    let yes_re = Regex::new(r"^y+?e*s*$").unwrap();
    let no_result = no_re.is_match(line.to_lowercase().strip_suffix("\r\n").unwrap());
    let yes_result = yes_re.is_match(line.to_lowercase().strip_suffix("\r\n").unwrap());
    if no_result {
        let size = get_size(&players);
        let rng_start = thread_rng().gen_range(1..=size) as i32;
        let rng_end = thread_rng().gen_range(1..=size) as i32;
        return (rng_start, rng_end)
    }
    else if yes_result {
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
    println!("====== Did not get valid input, generating two random players in history... ======");
    let size = get_size(&players);
    let rng_start = thread_rng().gen_range(1..=size) as i32;
    let rng_end = thread_rng().gen_range(1..=size) as i32;
    return (rng_start, rng_end)
}