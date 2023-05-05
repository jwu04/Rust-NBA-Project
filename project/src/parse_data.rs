pub mod read_file {
    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::{error::Error};
    use csv::{self};
    
    // Creating a player struct with 3 properties
    #[derive(Clone, Debug)]
    pub struct Player {
        pub id: i32,
        pub name: String,
        pub team: Option<Vec<Team>>
    }

    // Implementing methods for Player struct
    impl Player {
        // Method to create Player
        pub fn new(id: i32, name: String, team: Option<Vec<Team>>) -> Player {
            return Player {id: id, name: name, team: team}
        }
        
        // Method to get a player id
        pub fn get_id(&self) -> i32 {
            return self.id
        }

        // Method to get a player name
        pub fn get_name(&self) -> String {
            return (&self.name).to_string()
        }

        // Method to get a player team
        pub fn get_team(&self) -> &Vec<Team> {
            return self.team.as_ref().unwrap()
        }
    }

    // Creating a Team struct with 3 properties
    #[derive(Clone, Debug, PartialEq)]
    pub struct Team {
        pub team_id: i32,
        pub teammate_id: Option<HashSet<i32>>,
        pub year: String
    }

    // Implementing methods for Team struct
    impl Team {
        // Method to create Team
        pub fn new(team_id: i32, teammate_id: Option<HashSet<i32>>, year: &String) -> Team {
            return Team{team_id: team_id, teammate_id: teammate_id, year: year.to_string()}
        }

        // Method to get Team year
        pub fn get_year(&self) -> String {
            return self.year.to_string()
        }

        // Method to get Team ID
        pub fn get_team_id(&self) -> i32 {
            return self.team_id
        }

        // Method to get set of teammates ID
        pub fn get_teammmate_id(&self) -> &HashSet<i32> {
            return self.teammate_id.as_ref().unwrap()
        }

        // Method to check if a year has a specific teammate
        pub fn has_teammate_id(&self, id: i32) -> bool {
            return self.get_teammmate_id().contains(&id)
        }
    }

    // Function read player csv
    pub fn read_player_data() -> Result<HashMap<i32, Player>, Box<dyn Error>> {
        let player_data = csv::Reader::from_path("../data/players.csv");
        let mut players= HashMap::<i32,  Player>::new();
        // Creates a hashmap and assigns k: id, v: Player from csv file
        for result in player_data?.records() {
            let record = result?;
            let id = record[0].parse::<i32>().unwrap();
            let name = record[1].to_string();
            let person = Player::new(id, name, Some(Vec::new()));
            players.insert(id, person);
        }
        // Updates each Player and their teammates atrribute by filling in the year, season, and teammates that they played with
        let game_data = read_game_data(players);
        match game_data {
            Ok(data) => {
                return Ok(data)
            }
            Err(e) => {
                return Err(e)
            }
        }
    }

    // Function that reads team csv
    pub fn read_team_data() -> Result<HashMap<i32, String>, Box<dyn Error>> {
        let team_data = csv::Reader::from_path("../data/teams.csv");
        let mut teams= HashMap::<i32, String>::new();
        // Creates a hashmap with k: id v: team name from csv file
        for result in team_data?.records() {
            let record = result?;
            let id = record[0].parse::<i32>().unwrap();
            let name = record[1].to_string();
            teams.insert(id, name);
        }
        return Ok(teams)
    }

    // Function that reads from the player game data
    fn read_game_data(mut players: HashMap<i32, Player>) -> Result<HashMap<i32, Player>, Box<dyn Error>> {
        let game_data = csv::Reader::from_path("../data/game_player_data.csv");
        let mut team = Vec::<i32>::new();
        let mut temp_tid = -1;
        // Creates a vector that has a team and all of the player in that team 
        for result in game_data?.records() {
            let record = result?;
            let curr_team_id = record[2].parse::<i32>().unwrap();
            let player_id = record[4].parse::<i32>().unwrap();
            // Checks if the team id from CSV is the same as the previously stored team id
            if curr_team_id != temp_tid {
                // If it is not, then loop through the vector and create a HashSet of teammates if the year and team is the same
                // Then push that vector into the Team struct of the player (and only keeping the union of both data structures)
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
}