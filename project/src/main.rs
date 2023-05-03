use std::time::{Instant};
mod parse_data;
mod algo;

fn main() {
    let start = Instant::now();
    let mut players = parse_data::read_file::read_player_data();
    if let Result::Ok(data) = players {
        players = parse_data::read_file::read_game_data(data);
    }
    if let Result::Ok(data) = players {
        let graph = algo::search::bfs(data, 1234, 733);
        println!("{:?}", graph);
    }
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}