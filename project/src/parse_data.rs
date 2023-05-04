pub mod read_file {
    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::{error::Error};
    use csv::{self};
    
    #[derive(Clone, Debug)]
    pub struct Player {
        pub id: i32,
        pub name: String,
        pub team: Option<Vec<Team>>
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct Team {
        pub team_id: i32,
        pub teammate_id: Option<HashSet<i32>>,
        pub year: String
    }

    pub fn read_player_data() -> Result<HashMap<i32, Player>, Box<dyn Error>> {
        let player_data = csv::Reader::from_path("../data/players.csv");
        let mut players= HashMap::<i32,  Player>::new();
        for result in player_data?.records() {
            let record = result?;
            let id = record[0].parse::<i32>().unwrap();
            let name = record[1].to_string();
            let person = Player{id: id, name: name, team: Some(Vec::new())};
            players.insert(id, person);
        }
        return Ok(players)
    }

    pub fn read_game_data(mut players: HashMap<i32, Player>) -> Result<HashMap<i32, Player>, Box<dyn Error>> {
        let game_data = csv::Reader::from_path("../data/game_player_data.csv");
        let mut team = Vec::<i32>::new();
        let mut temp_tid = -1;
        for result in game_data?.records() {
            let record = result?;
            let curr_id = record[2].parse::<i32>().unwrap();
            let player_id = record[4].parse::<i32>().unwrap();
            if curr_id != temp_tid {
                for id in 0..(team.len()) {
                    let mut team_clone = team.clone();
                    let current_id = team_clone.swap_remove(id);
                    let team_set = HashSet::from_iter(team_clone.iter().cloned());
                    let year = &record[27];
                    let team = Team{team_id: temp_tid, teammate_id: Some(team_set.clone()), year: year.to_string()};
                    let all_teams = players.get_mut(&current_id).unwrap().team.as_mut().unwrap();
                    if all_teams.is_empty() {
                        all_teams.push(team);
                    } 
                    else {
                        let mut to_push = true;
                        for season in &mut *all_teams {
                            if season.team_id == temp_tid && season.year == year.to_string() {
                                let existing_team = &mut season.teammate_id.as_mut().unwrap();
                                for diff in existing_team.clone().difference(&team_set) {
                                    existing_team.insert(*diff);
                                }
                                to_push = false;
                                break;
                            }
                        }
                        if to_push {
                            all_teams.push(team);
                        }
                    }
                }
                temp_tid = curr_id;
                team.clear();
            }
            team.push(player_id);
        }
        Ok(players)
    }

    pub fn read_team_data() -> Result<HashMap<i32, String>, Box<dyn Error>> {
        let team_data = csv::Reader::from_path("../data/teams.csv");
        let mut teams= HashMap::<i32, String>::new();
        for result in team_data?.records() {
            let record = result?;
            let id = record[0].parse::<i32>().unwrap();
            let name = record[1].to_string();
            teams.insert(id, name);
        }
        return Ok(teams)
    }
}