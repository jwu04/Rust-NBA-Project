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

The following are sample outputs of the program:
```
====== Do you want to input two basketball players? (y/n): 
n
----------------------------------
====== NBA 6 Degrees of Separation Between:
====== [Jack Burmaster] and [Bobby Lewis]
----------------------------------
====== Jack Burmaster -- [Cleveland Rebels, 1949-50 Season] -- Noble Jorgensen
====== Noble Jorgensen -- [Philadelphia 76ers, 1950-51 Season] -- Dolph Schayes
====== Dolph Schayes -- [Philadelphia 76ers, 1962-63 Season] -- Len Chappell
====== Len Chappell -- [Cleveland Cavaliers, 1970-71 Season] -- Bobby Lewis
----------------------------------
====== Found 4 degree(s) of separation between [Len Chappell] and [Bobby Lewis]!
====== Jack Burmaster has an average degrees of separation of 4.6909356!
----------------------------------
====== Search completed in: 1.5031128s
```
```
====== Do you want to input two basketball players? (y/n):
n
----------------------------------
====== NBA 6 Degrees of Separation Between:
====== [Kevin Hervey] and [Allan Ray]
----------------------------------
====== Due to insufficient data, it could not establish a connection between Kevin Hervey and Allan Ray.
====== Try again!
----------------------------------
====== Search completed in: 1.4800397s
```
```
====== Do you want to input two basketball players? (y/n): 
y
====== Input the first basketball player (from 1949-2019):
lebron james
====== Input the second basketball player (from 1949-2019):
wilt chamberlain
----------------------------------
====== NBA 6 Degrees of Separation Between:
====== [LeBron James] and [Wilt Chamberlain]
----------------------------------
====== LeBron James -- [Cleveland Cavaliers, 2004-05 Season] -- Scott Williams
====== Scott Williams -- [Chicago Bulls, 1990-91 Season] -- Bill Cartwright
====== Bill Cartwright -- [New York Knicks, 1979-80 Season] -- Jim Cleamons
====== Jim Cleamons -- [Los Angeles Lakers, 1971-72 Season] -- Wilt Chamberlain
----------------------------------
====== Found 4 degree(s) of separation between [Jim Cleamons] and [Wilt Chamberlain]!
====== LeBron James has an average degrees of separation of 2.6204107!
----------------------------------
====== Search completed in: 13.5168005s
```

---

Kaggle Dataset: https://www.kaggle.com/datasets/harisbeslic/nba-player-data-by-game-from-1949-to-2019?select=all_game_scores.csv
