pub mod search {
    use std::collections::VecDeque;
    use std::collections::HashMap;
    use crate::parse_data;
    use crate::parse_data::read_file::Player;

    pub fn find_id(players: &HashMap<i32, Player>, p1: String) -> i32 {
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

    pub fn bfs(players: HashMap<i32, Player>, v_start: i32, v_end: i32) -> String {
        let sizes = get_max_id(&players);
        let mut distance: Vec<Option<u32>> = vec![None;sizes];
        let mut component: Vec<Option<String>> = vec![None;sizes];
        component[v_start as usize] = Some(players.get(&v_start).unwrap().get_id().to_string());
        distance[v_start as usize] = Some(0);
        let mut queue: VecDeque<i32> = VecDeque::new();
        queue.push_back(v_start);
        while let Some(v) = queue.pop_front() {
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
            for edge in &teammates {
                if let None = distance[*edge as usize] {
                    distance[*edge as usize] = Some(distance[v as usize].unwrap() + 1);
                    let connect_player = players.get(&edge).unwrap();
                    if let Some(component_id) = &component[v as usize] {
                        let mut vertex_id = component_id.to_string();
                        let edge_id= connect_player.get_id();
                        vertex_id.push_str(&format!(",{}", edge_id));
                        component[*edge as usize] = Some(vertex_id);
                    }
                    queue.push_back(*edge); 
                }
            }
        }
        return bfs_graph(component, players, v_start, v_end);
    }

    pub fn get_max_id(players: &HashMap<i32, Player>) -> usize {
        return players.len()+1
    }

    fn bfs_graph(c: Vec<Option<String>>, players: HashMap<i32, Player>, v_start: i32, v_end: i32) -> String {
        let start_name = players.get(&v_start).unwrap().get_name();
        let end_name = players.get(&v_end).unwrap().get_name();
        let edges = &c[v_end as usize];
        match edges {
            Some(edge_string) => {
                let parsed = edge_string.split(',').collect::<Vec<&str>>(); 
                let edges_len = parsed.len();
                let mut graph = String::new();
                if edges_len > 1 {
                    for index in 0..edges_len-1 as usize {
                        let player_one = players.get(&parsed[index].parse::<i32>().unwrap()).unwrap();
                        let player_two = players.get(&(parsed[index+1].parse::<i32>().unwrap())).unwrap();
                        let cxn = same_team_season(&players, player_one.get_id(), player_two.get_id());
                        graph.push_str(&format!("\n====== {} -- [{}] -- {}", player_one.get_name(), cxn, player_two.get_name()));
                        if index == edges_len-2 {
                            graph.push_str(&format!("\n----------------------------------\n====== Found {} degree(s) of separation between [{}] and [{}]!", edges_len-1, player_one.get_name(), player_two.get_name()));
                        }
                    }
                } 
                else {
                    let player = players.get(&parsed[0].parse::<i32>().unwrap()).unwrap();
                    let cxn = same_team_season(&players, player.get_id(), player.get_id());
                    graph.push_str(&format!("\n====== {0} -- [{1}] -- {2}\n====== Found {3} degree(s) of separation between [{0}] and [{2}]!", player.name, cxn, player.name, edges_len-1))
                }
                return format!("----------------------------------\n====== NBA 6 Degrees of Freedom Between: \n====== [{}] and [{}]\n----------------------------------{}",start_name, end_name, graph);
            }
            None => {
                let error_text = format!("====== Due to insufficient data, it could not establish a connection between {} and {}.\n====== Try again!", start_name, end_name);
                return format!("----------------------------------\n====== NBA 6 Degrees of Freedom Between: \n====== [{}] and [{}]\n----------------------------------\n{}", start_name, end_name, error_text);
            }
        }
    }

    fn same_team_season(players: &HashMap<i32, Player>, p1_id: i32, p2_id: i32) -> String {
        let p1_seasons = players.get(&p1_id).unwrap().get_team();
        let teams = parse_data::read_file::read_team_data().unwrap();
        if p1_id == p2_id {
            return format!("{}, {} Season", teams.get(&p1_seasons[0].get_team_id()).unwrap(), p1_seasons[0].get_year());
        }
        let mut team_year = "".to_string();
        for season in p1_seasons {
            if season.has_teammate_id(p2_id) {
                team_year.push_str(&format!("{}, {} Season", teams.get(&season.get_team_id()).unwrap(), season.get_year()));
                break;
            }
        }
        return team_year
    }

}