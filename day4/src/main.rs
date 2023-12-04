// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53

use std::{str::FromStr, io::{BufReader, BufRead}, fs::File};

struct Cards {
    cards: Vec<Card>
}

impl Cards {
    fn read_file(path: &str)->Cards {
        
        let cards: Vec<Card> =BufReader::new(File::open(path).unwrap())
            .lines()
            .map(|line| line.unwrap().as_str().into())
            .collect();

        Cards {
            cards
        }
    }

    fn score(&self)->usize {
        self.cards
            .iter()
            .map(|card| card.winning_score())
            .sum()
    }
}

struct Card {
    id: usize,
    numbers: Vec<usize>,
    winning_numbers: Vec<usize>,
}

impl Card {

    fn winning_score(&self)->usize {
        let winning_count = self.numbers
            .iter()
            .filter(|number| self.winning_numbers.contains(number))
            .count();
        if winning_count == 0 {
            return 0;
        }
        2_usize.pow(winning_count as u32 -1)
    }
}

impl From<&str> for Card {

    fn from(line: &str) -> Self {
        let (id_part, numbers) = line.split_once(":").unwrap();
        let id = id_part.split_once(" ").unwrap().1.trim().parse::<usize>().unwrap();
        let (number_part, winning_part) = numbers.split_once("|").unwrap();
        Self {
            id,
            numbers: number_part.split(" ")
                .filter(|i| !i.is_empty())
                .map(|i| 
                    i.trim().parse::<usize>().unwrap()
                )
                .collect(),
            winning_numbers: winning_part.split(" ")
                .filter(|i| !i.is_empty())
                .map(|i| 
                    i.trim().parse::<usize>().unwrap()
                )
                .collect(),
        }
    }
}

fn main() {
    println!("score: {}", Cards::read_file("day4/input.txt").score())

}


#[cfg(test)]
mod test {
    use crate::Card;

    #[test]
    fn test_parse_card() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card: Card = input.into();
        assert_eq!(1,card.id);
        assert_eq!(8,card.winning_score());
    }
}