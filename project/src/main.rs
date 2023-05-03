use std::{error::Error};
use csv::{self};
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::{Instant};
use std::collections::VecDeque;
mod parse_data;

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let mut players = parse_data::read_file::read_player_data();
    if let Result::Ok(data) = players {
        players = parse_data::read_file::read_game_data(data);
    }
    println!("{:?}", players);
    // let start_v = 530;
    // let end_v = 4193;
    // let mut distance: Vec<Option<u32>> = vec![None;4820];
    // let mut component: Vec<Option<String>> = vec![None;4820];
    // if let Some(player) = players.get(&start_v) {
    //     let name = &player.name;
    //     component[start_v as usize] = Some(name.to_string());
    // }
    // distance[start_v as usize] = Some(0);
    // let mut queue: VecDeque<i32> = VecDeque::new();
    // queue.push_back(start_v);
    // while let Some(v) = queue.pop_front() {
    //     let mut teammates = Vec::<i32>::new();
    //     if let Some(player) = players.get(&v) {
    //         if let Some(team) = &player.team {
    //             for season in team {
    //                 if let Some(existing_team) = &season.teammate_id {
    //                     for id in existing_team {
    //                         if !teammates.contains(id) {
    //                             teammates.push(*id);
    //                         }
    //                     }
    //                 }
    //             }

    //         }
    //     }
    //     for edge in &teammates {
    //         if let None = distance[*edge as usize] {
    //             distance[*edge as usize] = Some(distance[v as usize].unwrap() + 1);
    //             if let Some(connect_player) = players.get(&edge) {
    //                 if let Some(component_name) = &component[v as usize] {
    //                     let mut vertex_name = component_name.to_string();
    //                     let edge_name= &connect_player.name;
    //                     vertex_name.push_str(&" --> ");
    //                     vertex_name.push_str(&edge_name);
    //                     component[*edge as usize] = Some(vertex_name.clone());
    //                 }
    //             }
    //             queue.push_back(*edge); 
    //         }
    //     }
    // }
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
    // print!("   {}:{:?}:{:?}",start_v,distance[end_v as usize],component[end_v as usize]);
    Ok(())
}