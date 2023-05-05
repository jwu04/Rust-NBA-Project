# Rust | NBA - 6 Degrees of Separation
Collaborators: none  
Jonathan Wu  
Final Project Spring 2023  

## Project Description:  
For this project, I chose a dataset from Kaggle titled “NBA player data by game from 1949 to 2019,” which can be found at the bottom of the README.md. Through this project, I wanted to focus on finding a path that connects a Player A to Player C, and for that reason, I thought of using the idea of Six Degrees of Freedom into play.  The vertices would be the players, and each edge would have to be the fact that Player A and Player B have been teammates on the same team in the same year. The algorithm I used for this project is Breadth-First Search (BFS) for traversing graph data structures because of its advantage to find me the shortest distance between players, which is what I want.  

--- 

## Observations
* The data only contains players from 1949-2019.
* There may not be a connection between two players in the datasets.
  * I would imagine this is because the data is lackluster and doesn’t have all the necessary information for me to properly find the path between two players.
  * Either that, or there actually isn’t any connection between two players.
* `cargo run –release` is 8x faster than `cargo run`.
* `cargo run –release` takes about 1.5 seconds to run while `cargo run` takes 13 seconds.
* Players with long careers tend to have shorter degrees of separation on average than players with shorter careers.  

## Launch Codes
The following instructions assume that you have Rust already installed.  

1. Clone this repository.
```
$ git clone https://github.com/jwu04/Rust-NBA-Project.git
```
2. CD into the Project Directory
```
$ cd Rust-NBA-Project/
```
3. CD into source
```
$ cd src/
```
4. Compile the program
```
$ cargo run --release
```
5. Input text according to prompt!
6. Enjoy!

---

Kaggle Dataset: https://www.kaggle.com/datasets/harisbeslic/nba-player-data-by-game-from-1949-to-2019?select=all_game_scores.csv
