use std::{io::{BufReader, BufRead}, fs::File, collections::HashSet};

struct Schematic {
    lines: Vec<String>
}
#[derive(Hash,PartialEq, Eq, Clone,Debug)]
struct SchematicNumber(String,i32,i32);

impl SchematicNumber {
    fn contains(&self, x: i32,y: i32)->bool {
        if self.2 != y {
            return false
        }
        return x >= self.1 && x< self.1 + self.0.len() as i32
    }
}

impl Schematic {
    fn read(path: &str)->Self {
        let buffer = BufReader::new(File::open(path).unwrap());
        Self {
            lines: buffer.lines().map(|f| f.unwrap()).collect()
        }
    }

    fn find_all_numbers(&self)->Vec<SchematicNumber> {
        self.lines.iter().enumerate()
            .flat_map(|(y,line)| {
                Self::find_numbers_for_line(line)
                    .into_iter()
                    .map(move |(x,number)| {
                        SchematicNumber(number,x as i32,y as i32)
                    })
            })
        .collect()
    }
    fn find_part_number_sum(&self)->usize {
        self.find_part_numbers()
            .iter()
            .map(|SchematicNumber(number,_,_)| number.parse::<usize>().unwrap())
            .into_iter()
            .sum()
    }

    fn find_part_numbers(&self)->Vec<SchematicNumber> {
        self.find_all_numbers()
            .into_iter()
            .filter(|SchematicNumber(number, x, y)| {
                let length = number.len();
                (0_usize..length)
                    .any(|offset| self.touches_symbol(*x+offset as i32, *y))
            })
            .collect()
    }
    fn line_width(&self)->usize {
        self.lines.first().unwrap().len()
    }

    fn find_cogs(&self)->Vec<(i32,i32)> {
        self.lines
            .iter()
            .enumerate()
            .flat_map(|(y,line)| line.chars()
                .enumerate()
                .filter(|(index,ch)|*ch == '*')
                . map(move |(x,_)|(x as i32,y as i32))
            )
            .collect()
    }

    fn find_total_cogs(&self)->usize {
        let list = &self.find_part_numbers();
        self.find_cogs()
            .iter()
            .map(|(x,y)| self.find_cog_value(list, *x, *y))
            .sum()
    }

    fn find_cog_value(&self, list: &Vec<SchematicNumber>, x: i32, y: i32)->usize {
        let numbers = self.find_numbers_around(list, x, y);
        if numbers.len() != 2 {
            return 0;
        }
        numbers.iter()
            .map(|schema_nr| schema_nr.0.parse::<usize>().unwrap())
            .product()

    }

    fn find_number_at(&self, list: &Vec<SchematicNumber>, x: i32, y: i32)->Option<SchematicNumber> {
        if x<0 || x>=self.line_width() as i32 || y < 0 || y>=self.lines.len() as i32 {
            return None;
        }
        list.iter()
            .find(|s| s.contains(x, y))
            .map(|f|f.clone())
    }

    fn find_numbers_around(&self, list: &Vec<SchematicNumber>, x: i32, y: i32)->HashSet<SchematicNumber> {
        let mut result = HashSet::new();
        self.find_number_at(list, x-1, y-1).map(|f| result.insert(f));
        self.find_number_at(list, x, y-1).map(|f| result.insert(f));
        self.find_number_at(list, x+1, y-1).map(|f| result.insert(f));
        self.find_number_at(list, x+1, y).map(|f| result.insert(f));
        self.find_number_at(list, x+1, y+1).map(|f| result.insert(f));
        self.find_number_at(list, x, y+1).map(|f| result.insert(f));
        self.find_number_at(list, x-1, y+1).map(|f| result.insert(f));
        self.find_number_at(list, x-1, y).map(|f| result.insert(f));

        result
    }

    fn touches_symbol(&self,x: i32, y: i32)->bool {
        self.is_symbol(x-1, y-1) 
            || self.is_symbol(x, y-1)
            || self.is_symbol(x+1, y-1)
            || self.is_symbol(x+1, y)
            || self.is_symbol(x+1, y+1)
            || self.is_symbol(x, y+1)
            || self.is_symbol(x-1, y+1)
            || self.is_symbol(x-1, y)
            
    }

    fn is_symbol(&self, x: i32, y: i32)->bool {
        if x<0 || x>=self.line_width() as i32 || y < 0 || y>=self.lines.len() as i32 {
            return false;
        }
        let symbol = self.lines[y as usize].chars().nth(x as usize).unwrap();
        return !symbol.is_digit(10) && !(symbol=='.');
    }


    fn find_numbers_for_line(line: &str)->Vec<(usize,String)> {
        let mut result = vec![];
        let mut current = "".to_owned();
        let mut start_index = 0;
        for (index,c) in line.chars().enumerate() {
            if c.is_digit(10) {
                if current.is_empty() {
                    start_index = index;
                }
                current.push(c);
            } else {
                if !current.is_empty() {
                    result.push((start_index,current.clone()));
                    current.clear();
                }
            }
        }
        if !current.is_empty() {
            result.push((start_index,current));
        }
        result
    }
}
fn main() {
    println!("Hello, world!");
    // part 1
    let s = Schematic::read("day3/input.txt");
    println!("part1: {}",s.find_part_number_sum());
    println!("part2: {}",s.find_total_cogs());

}
#[cfg(test)]
mod tests {
    use crate::Schematic;

    #[test]
    fn test_read() {
        let s = Schematic::read("debug.txt");
        assert_eq!(10,s.lines.len())
    }

    #[test]
    fn test_numbers_for_line() {
        let result = Schematic::find_numbers_for_line("467..114..");
        assert_eq!(vec!((0,"467".to_owned()),(5,"114".to_owned())),result);
    }

    #[test]
    fn test_is_symbol() {
        let s = Schematic::read("debug.txt");
        assert!(s.is_symbol(3, 1));
        assert!(!s.is_symbol(0, 0));
        assert!(!s.is_symbol(0, 4));
    }

    #[test]
    fn test_touches_symbol() {
        let s = Schematic::read("debug.txt");
        assert!(s.touches_symbol(2, 0));
    }

    #[test]
    fn test_part_1() {
        let s = Schematic::read("debug.txt");
        // println!("Numbers: {:?}",s.find_part_number_sum());
        assert_eq!(4361,s.find_part_number_sum());
    }

    #[test]
    fn test_part_2() {
        let s = Schematic::read("debug.txt");
        // println!("Numbers: {:?}",s.find_part_number_sum());
        assert_eq!(467835,s.find_total_cogs());
    }


    
    #[test]
    fn test_find_cogs() {
        let s = Schematic::read("debug.txt");
        assert_eq!(vec![(3, 1), (3, 4), (5, 8)], s.find_cogs());
        println!("Cogs: {:?}",s.find_cogs());
    }

    #[test]
    fn test_find_numbers_around() {
        let s = Schematic::read("debug.txt");
        // assert_eq!(vec![(3, 1), (3, 4), (5, 8)], s.find_cogs());
        // println!("Cogs: {:?}",s.find_cogs());
        let schematics = s.find_numbers_around(&s.find_all_numbers(), 3, 1);
        println!("schematics: {:?}",schematics);
    }

}