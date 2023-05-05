pub mod search {
    use std::collections::VecDeque;
    use std::collections::HashMap;
    use std::{error::Error};
    use rand::prelude::*;
    use crate::parse_data;
    use crate::parse_data::read_file::Player;

    // Function to find ID of a player given a string
    pub fn find_id(players: &HashMap<i32, Player>, mut p1: String) -> i32 {
        p1 = p1.to_lowercase();
        for p_id in 1..get_max_id(players) as i32 {
            let player = players.get(&p_id).unwrap();
            let p_name = player.get_name().to_lowercase();
            if p_name.eq(&p1) {
                return player.get_id()
            }
        }
        println!("====== Could not find a player with that name (either the player does not exist in the data or it is incorrectly spelled).");
        return -1
    }

    // Function that performs BFS 
    pub fn bfs(players: &HashMap<i32, Player>, mut v_start: i32, mut v_end: i32) -> Result<String, Box<dyn Error>> {
        let sizes = get_max_id(&players);
        let mut distance: Vec<Option<u32>> = vec![None;sizes];
        let mut component: Vec<Option<String>> = vec![None;sizes];
        // First verifies that the given vertices are valid
        // If not, then replace the vertices with ones that are valid (randomly generated)
        if !verify_vertices(players, v_start, v_end) {
            let nums = gen_ids(players);
            v_start = nums.0;
            v_end = nums.1;
        }
        // Set the vectors of component and distance of the starting vector
        component[v_start as usize] = Some(players.get(&v_start).unwrap().get_id().to_string());
        distance[v_start as usize] = Some(0);
        // Initialize a Deque and push the starting vertex
        let mut queue: VecDeque<i32> = VecDeque::new();
        queue.push_back(v_start);
        // Keep looping through the Deque until there are no more players to loop through
        while let Some(v) = queue.pop_front() {
            // This creates a vector of all of a player's teammates
            let mut teammates = Vec::<i32>::new();
            let player_career = players.get(&v).unwrap().get_team();
            for season in player_career {
                let existing_team = season.get_teammmate_id();
                for id in existing_team {
                    if !teammates.contains(id) {
                        teammates.push(*id);
                    }
                }
            }
            // Looping through all the player's teammates
            for edge in &teammates {
                // If there is no distance, add 1 + previous distance
                if let None = distance[*edge as usize] {
                    distance[*edge as usize] = Some(distance[v as usize].unwrap() + 1);
                    //  Creates a string of vertices and edges that connects player A to player B
                    let connect_player = players.get(&edge).unwrap();
                    if let Some(component_id) = &component[v as usize] {
                        let mut vertex_id = component_id.to_string();
                        let edge_id= connect_player.get_id();
                        vertex_id.push_str(&format!(",{}", edge_id));
                        component[*edge as usize] = Some(vertex_id);
                    }
                    // Push teammate ID to the deque
                    queue.push_back(*edge); 
                }
            }
        }
        return bfs_graph(component, &players, v_start, v_end);
    }

    // Function to get the max_id of a player
    // * The max_id is 4821, but the HashMap length is 4820, which is why I add 1
    pub fn get_max_id(players: &HashMap<i32, Player>) -> usize {
        return players.len()+1
    }

    // Function to generate two random vertices within the dataset
    pub fn gen_ids(players: &HashMap<i32, Player>) -> (i32, i32) {
        let size = get_max_id(&players);
        let rng_start = thread_rng().gen_range(1..=size) as i32;
        let rng_end = thread_rng().gen_range(1..=size) as i32;
        return (rng_start, rng_end)
    }

    // Function that creates the 'graph' of our players
    fn bfs_graph(c: Vec<Option<String>>, players: &HashMap<i32, Player>, v_start: i32, v_end: i32) -> Result<String, Box<dyn Error>> {
        let start_name = players.get(&v_start).unwrap().get_name();
        let end_name = players.get(&v_end).unwrap().get_name();
        let edges = &c[v_end as usize];
        // Checking to see if there is a connection
        match edges {
            // If there is, turn the string of IDs into i32s to get the name
            // Then format it into a string and concatenate it with another string
            // * Checks the edge case if the player IDs are the same as well
            Some(edge_string) => {
                let parsed = edge_string.split(',').collect::<Vec<&str>>(); 
                let edges_len = parsed.len();
                let mut graph = String::new();
                if edges_len > 1 {
                    for index in 0..edges_len-1 as usize {
                        let player_one = players.get(&parsed[index].parse::<i32>().unwrap()).unwrap();
                        let player_two = players.get(&(parsed[index+1].parse::<i32>().unwrap())).unwrap();
                        let cxns = same_team_season(&players, player_one.get_id(), player_two.get_id());
                        match cxns {
                            Ok(cxn) => {
                                graph.push_str(&format!("\n====== {} -- [{}] -- {}", player_one.get_name(), cxn, player_two.get_name()));
                                if index == edges_len-2 {
                                    graph.push_str(&format!("\n----------------------------------\n====== Found {} degree(s) of separation between [{}] and [{}]!", edges_len-1, player_one.get_name(), player_two.get_name()));
                                }
                            }
                            Err(e) => {
                                return Err(e)
                            }
                        }

                    }
                } 
                else {
                    let player = players.get(&parsed[0].parse::<i32>().unwrap()).unwrap();
                    let cxns = same_team_season(&players, player.get_id(), player.get_id());
                    match cxns {
                        Ok(cxn) => {
                            graph.push_str(&format!("\n====== {0} -- [{1}] -- {2}\n====== Found {3} degree(s) of separation between [{0}] and [{2}]!", player.name, cxn, player.name, edges_len));
                        }
                        Err(e) => {
                            return Err(e)
                        }
                    }
                }
                return Ok(format!("----------------------------------\n====== NBA 6 Degrees of Freedom Between: \n====== [{}] and [{}]\n----------------------------------{}",start_name, end_name, graph));
            }
            None => {
                let error_text = format!("====== Due to insufficient data, it could not establish a connection between {} and {}.\n====== Try again!", start_name, end_name);
                return Ok(format!("----------------------------------\n====== NBA 6 Degrees of Freedom Between: \n====== [{}] and [{}]\n----------------------------------\n{}", start_name, end_name, error_text));
            }
        }
    }

    // Function that retrieves the first year and team that two teammates played together in
    fn same_team_season(players: &HashMap<i32, Player>, p1_id: i32, p2_id: i32) -> Result<String, Box<dyn Error>> {
        let p1_seasons = players.get(&p1_id).unwrap().get_team();
        let teams_data = parse_data::read_file::read_team_data();
        match teams_data {
            Ok(teams) => {
                if p1_id == p2_id {
                    return Ok(format!("{}, {} Season", teams.get(&p1_seasons[0].get_team_id()).unwrap(), p1_seasons[0].get_year()));
                }
                let mut team_year = "".to_string();
                for season in p1_seasons {
                    if season.has_teammate_id(p2_id) {
                        team_year.push_str(&format!("{}, {} Season", teams.get(&season.get_team_id()).unwrap(), season.get_year()));
                        break;
                    }
                }
                return Ok(team_year)
            }
            Err(e) => {
                return Err(e)
            }
        }

    }

    // Function that verifies if the vertices are within the bounds of the dataset
    fn verify_vertices(players: &HashMap<i32, Player>, v1: i32, v2: i32) -> bool{
        let size: usize = crate::algo::search::get_max_id(&players);
        if (v1 >= 1 && v1 < size as i32) && (v2 >= 1 && v2 < size as i32) {
            return true
        }
        else {
            return false
        }
    }
}