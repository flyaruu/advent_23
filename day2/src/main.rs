use std::{io::{BufReader, BufRead}, fs::File};

#[derive(Debug,PartialEq, Eq)]
struct KubeSet {
    red: usize,
    green: usize,
    blue: usize,
}

// type Play = Bag;

struct Game{id: usize, plays: Vec<KubeSet>}

impl Game {
    fn is_possible_for_bag(&self, bag: &KubeSet)->bool {
        self.plays.iter().all(|play| play.is_subset_of(&bag))
    }

    fn min_bag_required(&self)->KubeSet {
        let mut required = KubeSet::default();
        for play in self.plays.iter() {
            if play.red > required.red {
                required.red = play.red;
            }
            if play.green > required.green {
                required.green = play.green;
            }
            if play.blue > required.blue {
                required.blue = play.blue;
            }
        }
        required
    }

    fn power(&self)->usize {
        self.min_bag_required().power()
    }
}

impl KubeSet {
    fn is_subset_of(&self, other: &KubeSet)->bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }

    fn power(&self)->usize {
        self.red * self.green * self.blue
    }
}
impl Default for KubeSet {
    fn default() -> Self {
        Self { red: 0, green: 0, blue: 0 }
    }
}
// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green

impl From<&str> for Game {
    fn from(line: &str) -> Self {
        let (game_id,line) = line.split_once(":").unwrap();
        let id: usize = game_id[5..].parse().unwrap();
        Game {id, plays: line.split("; ").into_iter().map(|l| KubeSet::from(l)).collect()}
    }
}

// 3 blue, 4 red
impl From<&str> for KubeSet {
    fn from(line: &str) -> Self {
        let mut play = KubeSet::default();
        for color in line.trim().split(",") {
            let color = color.trim();
            println!("Color: {}",color);
            if color.ends_with(" red") {
                play.red = color[0..color.len()-4].parse().unwrap();
            }
            if color.ends_with("green") {
                play.green = color[0..color.len()-6].parse().unwrap();
            }
            if color.ends_with("blue") {
                play.blue = color[0..color.len()-5].parse().unwrap();
            }
        }
        play
    }
}

fn read_games(path: &str)->Vec<Game> {
    BufReader::new(File::open(path).unwrap()).lines()
        .map(|line| Game::from(line.unwrap().as_str()))
        .collect()
    // reader
}

// 12 red cubes, 13 green cubes, and 14 blue cubes
fn main() {
    let bag = KubeSet { red: 12, green: 13, blue: 14 };
    let result: usize = read_games("day2/input.txt")
        .iter()
        .filter(|game| game.is_possible_for_bag(&bag))
        .map(|game| game.id)
        .sum();
    println!("Result: {}",result);

    let result: usize = read_games("day2/input.txt")
        .iter()
        .map(|game| game.power())
        .sum();
    println!("Result 2 : {}",result);

}

#[cfg(test)]
mod test {
    use crate::{KubeSet, Game, read_games};

    #[test]
    fn test_read_play() {
        let play = KubeSet::from("3 blue, 4 red");
        assert_eq!(3,play.blue);
        assert_eq!(4,play.red);
        assert_eq!(0,play.green);
    }

    #[test]
    fn test_read_game() {
        let game = Game::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");

        assert_eq!(1,game.id);
        assert_eq!(3,game.plays.len());
    }

    #[test]
    fn test_bag_possible() {
        let game = Game::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        let bag = KubeSet { red: 6, green:6, blue: 6 };
        assert!(game.is_possible_for_bag(&bag));
        let bag = KubeSet { red: 6, green:6, blue: 5 };
        assert!(!game.is_possible_for_bag(&bag));
    }

    #[test]
    fn test_read_games() {
        let games = read_games("debug.txt");
        assert_eq!(5,games.len())
    }

    #[test]
    fn test_bag_required() {
        let game = Game::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        let bag = KubeSet { red: 4, green:2, blue: 6 };
        let required = game.min_bag_required();
        assert_eq!(required,bag);
        // let game_power = required.power();
        assert_eq!(48,game.power());
    }

    #[test]
    fn test_read_games_2() {
        let games = read_games("debug.txt");
        let power: usize = games.iter().map(|game| game.power()).sum();
        assert_eq!(2286,power);
    }


}
