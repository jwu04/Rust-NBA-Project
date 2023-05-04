pub mod search {
    use std::collections::VecDeque;
    use std::collections::HashMap;
    use crate::parse_data;
    use crate::parse_data::read_file::Player;

    fn same_team_season(players: &HashMap<i32, Player>, p1_id: i32, p2_id: i32) -> String {
        let p1_seasons = players.get(&p1_id).unwrap().team.as_ref().unwrap();
        let teams = parse_data::read_file::read_team_data().unwrap();
        if p1_id == p2_id {
            return format!("{}, {} Season", teams.get(&p1_seasons[0].team_id).unwrap(), p1_seasons[0].year);
        }
        let mut team_year = "".to_string();
        for season in p1_seasons {
            if season.teammate_id.as_ref().unwrap().contains(&p2_id) {
                team_year.push_str(&format!("{}, {} Season", teams.get(&season.team_id).unwrap(), season.year));
                break;
            }
        }
        return team_year
    }

    pub fn find_id(players: &HashMap<i32, Player>, p1: String) -> i32 {
        for p in 1..get_size(players) {
            let player = players.get(&(p as i32)).unwrap();
            let p_name = &player.name;
            if p_name.to_lowercase().eq(&p1) {
                return player.id
            }
        }
        println!("====== Could not find a player with that name (either the player does not exist in the data or it is incorrectly spelled).");
        return -1
    }

    fn bfs_graph(c: Vec<Option<String>>, players: HashMap<i32, Player>, v_start: i32, v_end: i32) -> String {
        let start_name = players.get(&v_start).unwrap().name.to_string();
        let end_name = players.get(&v_end).unwrap().name.to_string();
        let edges = c[v_end as usize].as_ref();
        match edges {
            Some(edge_string) => {
                let parsed = edge_string.split(',').collect::<Vec<&str>>(); 
                let mut graph = "".to_string();
                if parsed.len() > 1 {
                    for index in 0..parsed.len()-1 as usize {
                        let player_one = players.get(&parsed[index].parse::<i32>().unwrap()).unwrap();
                        let player_two = players.get(&(parsed[index+1].parse::<i32>().unwrap())).unwrap();
                        let cxn = same_team_season(&players, player_one.id, player_two.id);
                        graph.push_str(&format!("\n====== {} -- [{}] -- {}", player_one.name, cxn, player_two.name));
                        if index == parsed.len()-2 {
                            graph.push_str(&format!("\n----------------------------------\n====== Found {} degree(s) of separation between [{}] and [{}]!", parsed.len()-1, players.get(&parsed[0].parse::<i32>().unwrap()).unwrap().name, player_two.name));
                        }
                    }
                } 
                else {
                    let player_one = players.get(&parsed[0].parse::<i32>().unwrap()).unwrap();
                    let player_two = players.get(&(parsed[0].parse::<i32>().unwrap())).unwrap();
                    let cxn = same_team_season(&players, player_one.id, player_two.id);
                    graph.push_str(&format!("\n====== {0} -- [{1}] -- {2}\n====== Found {3} degree(s) of separation between [{0}] and [{2}]!", player_one.name, cxn, player_two.name, parsed.len()-1))
                }
                return format!("----------------------------------\n====== NBA 6 Degrees of Freedom Between: \n====== [{}] and [{}]\n----------------------------------{}",start_name, end_name, graph);
            }
            None => {
                let error_text = format!("====== Due to insufficient data, it could not establish a connection between {} and {}.\n====== Try again!", start_name, end_name);
                return format!("----------------------------------\n====== NBA 6 Degrees of Freedom Between: \n====== [{}] and [{}]\n----------------------------------\n{}", start_name, end_name, error_text);
            }
        }
        // return format!("Starts with: {:?}", c[v_end as usize]);
    }

    pub fn bfs(players: HashMap<i32, Player>, v_start: i32, v_end: i32) -> String {
        let sizes = get_size(&players);
        let mut distance: Vec<Option<u32>> = vec![None;sizes];
        let mut component: Vec<Option<String>> = vec![None;sizes];
        component[v_start as usize] = Some(players.get(&v_start).unwrap().id.to_string());
        distance[v_start as usize] = Some(0);
        let mut queue: VecDeque<i32> = VecDeque::new();
        queue.push_back(v_start);
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
                            let mut vertex_id = component_name.to_string();
                            let edge_id= &connect_player.id.to_string();
                            vertex_id.push_str(&format!(",{}", edge_id));
                            component[*edge as usize] = Some(vertex_id.clone());
                        }
                    }
                    queue.push_back(*edge); 
                }
            }
        }
        return bfs_graph(component, players, v_start, v_end);
    }

    pub fn get_size(players: &HashMap<i32, Player>) -> usize {
        return players.len()+1
    }
}