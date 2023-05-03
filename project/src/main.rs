use std::{error::Error};
use csv::{self};
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::{Duration, Instant};
use std::collections::VecDeque;
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

        // let record = result?;
        // if record[0].parse::<i32>().unwrap() % 100000 == 0 {
        //     println!("{:?}", record[0].parse::<i32>().unwrap());
        // }

        if record[2].parse::<i32>().unwrap() != temp_tid {
            for id in 0..(team.len()) {
                let mut team_clone = team.clone();
                let current_id = team_clone.swap_remove(id);
                let team_set = HashSet::from_iter(team_clone.iter().cloned());
                let team = Team{team_id: temp_tid, teammate_id: Some(team_set.clone()), year: record[27].to_string()};
                if let Some(player_teammates) = players.get_mut(&current_id) {
                    if let Some(Team) = &mut player_teammates.team {
                        if Team.len() == 0 {
                            Team.push(team);
                        } 
                        else {
                            let mut to_push = true;
                            for season in &mut *Team {
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
                                Team.push(team);
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

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
    // println!("{:?}", players.get(&3380));
    // println!("{:?}", players);
    Ok(())
}