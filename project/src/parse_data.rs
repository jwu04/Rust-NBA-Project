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

    impl Player {
        fn new(id: i32, name: String, team: Option<Vec<Team>>) -> Player {
            return Player {id: id, name: name, team: team}
        }
    
        pub fn get_id(&self) -> i32 {
            return self.id
        }

        pub fn get_name(&self) -> String {
            return (&self.name).to_string()
        }

        pub fn get_team(&self) -> &Vec<Team> {
            return self.team.as_ref().unwrap()
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct Team {
        pub team_id: i32,
        pub teammate_id: Option<HashSet<i32>>,
        pub year: String
    }

    impl Team {
        fn new(team_id: i32, teammate_id: Option<HashSet<i32>>, year: &String) -> Team {
            return Team{team_id: team_id, teammate_id: teammate_id, year: year.to_string()}
        }

        pub fn get_year(&self) -> String {
            return self.year.to_string()
        }

        pub fn get_team_id(&self) -> i32 {
            return self.team_id
        }

        pub fn get_teammmate_id(&self) -> &HashSet<i32> {
            return self.teammate_id.as_ref().unwrap()
        }

        pub fn has_teammate_id(&self, id: i32) -> bool {
            return self.get_teammmate_id().contains(&id)
        }
    }

    pub fn read_player_data() -> Result<HashMap<i32, Player>, Box<dyn Error>> {
        let player_data = csv::Reader::from_path("../data/players.csv");
        let mut players= HashMap::<i32,  Player>::new();
        for result in player_data?.records() {
            let record = result?;
            let id = record[0].parse::<i32>().unwrap();
            let name = record[1].to_string();
            let person = Player::new(id, name, Some(Vec::new()));
            players.insert(id, person);
        }
        players = read_game_data(players).unwrap();
        return Ok(players)
    }

    fn read_game_data(mut players: HashMap<i32, Player>) -> Result<HashMap<i32, Player>, Box<dyn Error>> {
        let game_data = csv::Reader::from_path("../data/game_player_data.csv");
        let mut team = Vec::<i32>::new();
        let mut temp_tid = -1;
        for result in game_data?.records() {
            let record = result?;
            let curr_team_id = record[2].parse::<i32>().unwrap();
            let player_id = record[4].parse::<i32>().unwrap();
            if curr_team_id != temp_tid {
                for id in 0..(team.len()) {
                    let mut team_clone = team.clone();
                    let current_id = team_clone.swap_remove(id);
                    let team_set = HashSet::from_iter(team_clone.iter().cloned());
                    let year = record[27].to_string();
                    let team = Team::new(temp_tid, Some(team_set.clone()), &year);
                    let all_teams = players.get_mut(&current_id).unwrap().team.as_mut().unwrap();
                    if all_teams.is_empty() {
                        all_teams.push(team);
                    } 
                    else {
                        let mut to_push = true;
                        for season in &mut *all_teams {
                            if season.get_team_id()== temp_tid && season.get_year() == year {
                                let existing_team = season.teammate_id.as_mut().unwrap();
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
                temp_tid = curr_team_id;
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