use std::{time::{Instant}, mem::take};
use rand::prelude::*;
use std::collections::HashMap;
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
    // if let Result::Ok(data) = players {

    //     let graph = algo::search::bfs(data, rng_start as i32, rng_end as i32);
    //     println!("{}", graph);
    // }
    let duration = start.elapsed();
    println!("\nSearch completed in: {:?}", duration);
}

fn take_input(players: &HashMap<i32, Player>) -> (i32, i32) {
    let mut line = String::new();
    println!("Do you want to input two basketball players? (y/n):");
    std::io::stdin().read_line(&mut line).unwrap();
    
    if line.eq("n") {
        println!("asdf{}", line);
        let size = get_size(&players);
        let rng_start = thread_rng().gen_range(1..=size) as i32;
        let rng_end = thread_rng().gen_range(1..=size) as i32;
        return (rng_start, rng_end)
    }
    // else {
    //     let mut player_a = String::new();
    //     println!("Input a basketball player (from 1949-2019):");
    //     std::io::stdin().read_line(&mut line).unwrap();
    //     println!("{}", player_a);
    //     let mut player_b = String::new();
    //     return (515, 723)
    // }
    return (515, 723)
}