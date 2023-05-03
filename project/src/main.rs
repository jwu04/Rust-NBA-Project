use std::{error::Error};
use csv::{self};
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::{Instant};
use std::collections::VecDeque;
#[allow(dead_code)]
#[derive(Clone, Debug)]
struct Player {
    id: i32,
    name: String,
    team: Option<Vec<Team>>
}

#[derive(Clone, Debug, PartialEq)]
struct Team {
    team_id: i32,
    teammate_id: Option<HashSet<i32>>,
    year: String
}

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let mut players= HashMap::<i32,  Player>::new();
    let read_players = csv::Reader::from_path("../data/players.csv");
    for result in read_players?.records() {
        let record = result?;
        let person = Player{id: record[0].parse::<i32>().unwrap(), name: record[1].to_string(), team: Some(Vec::new())};
        players.insert(record[0].parse::<i32>().unwrap(), person);
    }

    let game_data = csv::Reader::from_path("../data/game_player_data.csv");
    let mut team = Vec::<i32>::new();
    let mut temp_tid = -1;
    for result in game_data?.records() {

        let record = result?;

        if record[2].parse::<i32>().unwrap() != temp_tid {
            for id in 0..(team.len()) {
                let mut team_clone = team.clone();
                let current_id = team_clone.swap_remove(id);
                let team_set = HashSet::from_iter(team_clone.iter().cloned());
                let team = Team{team_id: temp_tid, teammate_id: Some(team_set.clone()), year: record[27].to_string()};
                if let Some(player_teammates) = players.get_mut(&current_id) {
                    if let Some(teams) = &mut player_teammates.team {
                        if teams.len() == 0 {
                            teams.push(team);
                        } 
                        else {
                            let mut to_push = true;
                            for season in &mut *teams {
                                if season.team_id == temp_tid && season.year == record[27] {
                                    if let Some(existing_team) = &mut season.teammate_id {
                                        for diff in existing_team.clone().difference(&team_set) {
                                            existing_team.insert(*diff);
                                        }
                                        to_push = false;
                                    }
                                }
                            }
                            if to_push == true {
                                teams.push(team);
                            }
                            
                        }
                    }
                }
            }
            temp_tid = record[2].parse::<i32>().unwrap();
            team.clear();
            
        }
        team.push(record[4].parse::<i32>().unwrap());
    }

    let start_v = 530;
    let end_v = 4193;
    let mut distance: Vec<Option<u32>> = vec![None;4820];
    let mut component: Vec<Option<String>> = vec![None;4820];
    if let Some(player) = players.get(&start_v) {
        let name = &player.name;
        component[start_v as usize] = Some(name.to_string());
    }
    distance[start_v as usize] = Some(0);
    let mut queue: VecDeque<i32> = VecDeque::new();
    queue.push_back(start_v);
    while let Some(v) = queue.pop_front() {
        let mut teammates = Vec::<i32>::new();
        if let Some(player) = players.get(&v) {
            if let Some(team) = &player.team {
                for season in team {
                    if let Some(existing_team) = &season.teammate_id {
                        for id in existing_team {
                            if !teammates.contains(id) {
                                teammates.push(*id);
                            }
                        }
                    }
                }

            }
        }
        for edge in &teammates {
            if let None = distance[*edge as usize] {
                distance[*edge as usize] = Some(distance[v as usize].unwrap() + 1);
                if let Some(connect_player) = players.get(&edge) {
                    if let Some(component_name) = &component[v as usize] {
                        let mut vertex_name = component_name.to_string();
                        let edge_name= &connect_player.name;
                        vertex_name.push_str(&" --> ");
                        vertex_name.push_str(&edge_name);
                        component[*edge as usize] = Some(vertex_name.clone());
                    }
                }
                queue.push_back(*edge); 
            }
        }
    }
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
    print!("   {}:{:?}:{:?}",start_v,distance[end_v as usize],component[end_v as usize]);
    Ok(())
}