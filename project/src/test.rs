#[cfg(test)]
mod test_cases {
    use std::vec;
    use std::collections::HashSet;

    // Test to make sure the randomly generated vertices fall within our dataset
    #[test]
    fn test_gen_id() {
        let players = crate::parse_data::read_file::read_player_data();
        let size: usize = crate::algo::search::get_max_id(&players.as_ref().unwrap());
        let nums = crate::algo::search::gen_ids(&players.unwrap());
        if (nums.0 >= 1 && nums.0 < size as i32) && (nums.1 >= 1 && nums.1 < size as i32) {
            assert_eq!(true, true)
        }
        else {
            assert_eq!(false, true)
        }
    }

    // Tests BFS in a way that makes sure that you can allow for any id for the start and end vertex
    // The idea is that if you can run it from one static id to the entire dataset, you can imagine
    // that BFS will run on any point in the data
    // ? Q: What if the graph the program shows me isn't right?
    // ! A: Well, the program either shows you a connection or not. I have manually checked each case
    // ! by cross referencing on the internet to make sure that two teammates actually have played with each otehr.
    // ! That includes running the program on the same vertex for start and end, invalid vertices, and any degrees of freedom
    // ! between two NBA players
    #[test]
    fn test_bfs() {
        let players = crate::parse_data::read_file::read_player_data();
        let size: usize = crate::algo::search::get_max_id(&players.as_ref().unwrap());
        for i in 1..size {
            for j in 1..2 {
                let result = crate::algo::search::bfs(players.as_ref().unwrap(), j as i32, i as i32);
                match result {
                    Ok(_r) => assert_eq!(true, true),
                    Err(_r) => assert_eq!(false, true),
                }
            }
        }
        
    }
    
    // Test to make sure the number returns is the max_id that is found from the dataset
    #[test]
    fn test_get_max_id() {
        let players = crate::parse_data::read_file::read_player_data().unwrap();
        let num = crate::algo::search::get_max_id(&players);
        assert_eq!(num as i32, 4821)
    }

    // Test to make sure that the ID found from my function is correct
    #[test]
    fn test_find_id() {
        let players = crate::parse_data::read_file::read_player_data().unwrap();
        let test = vec!["lebron james", "dosNot Exist", "-1", "JERRY WEST"];
        let test_answers = vec![2065, -1, -1, 4522];
        for name in 0..test.len() {
            let answer = crate::algo::search::find_id(&players, test.get(name).unwrap().to_string());
            assert_eq!(answer, test_answers[name]);
        }
    }
    
    // Test to make sure my Player struct can be instantiated and its implemented functions also work.
    #[test]
    fn test_player_attributes() {
        let dude = crate::parse_data::read_file::Player::new(0, "Real Dude".to_string(), Some(vec![crate::parse_data::read_file::Team::new(0, None, &format!("2014-5"))]));
        assert_eq!(dude.id, dude.get_id());
        assert_eq!(dude.name, dude.get_name());
        assert_eq!(dude.get_team(), &dude.clone().team.unwrap());
    }

    // Test to make sure my Team struct can be instantiated and its implemented functions also work.
    #[test]
    fn test_team_attributes() {
        let team = crate::parse_data::read_file::Team::new(0, Some(HashSet::new()), &format!("2014-5"));
        assert_eq!(team.team_id, team.get_team_id());
        assert_eq!(team.get_teammmate_id(), &team.clone().teammate_id.unwrap());
        assert_eq!(team.year, team.get_year()); 
        assert_eq!(false, team.has_teammate_id(0)); 
    }
}