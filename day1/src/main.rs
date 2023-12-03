use std::{fs::File, io::{BufRead, BufReader}};

const DIGIT_LITERALS: [(&'static str,&'static str); 9] = [("one","1"),
("two","2"),
("three","3"),
("four","4"),
("five","5"),
("six","6"),
("seven","7"),
("eight","8"),
("nine","9"),
];

fn main() {
    
    println!("Result: {}",process_file("day1/input.txt"));
}

fn process_file(path: &str)->i32 {
    BufReader::new(File::open(path).unwrap()).lines()
        .map(|line| line.unwrap())
        .map(|line| process_line(&line))
        // .map(|line| line.parse::<i32>().unwrap())
        .sum::<i32>()
}


fn find_first(input: &str)->char {
    let mut processed = 0;
    while processed < input.len() {
        let (_, to_do) = input.split_at(processed);
        if to_do.is_empty() {
            continue;
        }
        let first_char = to_do.chars().nth(0).unwrap();
        if first_char.is_digit(10) {
            return first_char;
        }
        for (text,digit) in DIGIT_LITERALS {
            // let (_,after) = input.split_at(index);
            if to_do.starts_with(text) {
                return digit.chars().nth(0).unwrap();
            }
        }
        processed+=1;
    }

    panic!("No digit found");
}


fn find_last(input: &str)->char {
    let mut processed = input.len();
    while processed > 0 {
        let (to_do, _) = input.split_at(processed);
        let last_char = to_do.chars().last().unwrap();
        if last_char.is_digit(10) {
            return last_char;
        }
        for (text,digit) in DIGIT_LITERALS {
            // let (_,after) = input.split_at(index);
            if to_do.ends_with(text) {
                return digit.chars().nth(0).unwrap();
            }
        }        

        processed-=1;

    }

    panic!("No digit found");
}

fn process_line(input: &str)->i32 {
    let mut result = "".to_owned();
    result.push(find_first(input));
    result.push(find_last(input));
    return result.parse().unwrap();

}


#[cfg(test)]
mod test {
    use crate::{process_line, process_file, find_last, find_first};


    #[test]
    fn test_find_last() {
        assert_eq!('8',find_last("pqr3stu8vwx"));
        assert_eq!('3',find_first("pqr3stu8vwx"));
    }

    #[test]
    fn test_first_last() {
        assert_eq!('3',find_last("eightwothree"));
        assert_eq!('8',find_first("eightwothree"));
    }


    #[test]
    fn test_simple() {
        assert_eq!(12,process_line("1abc2"));
        assert_eq!(38,process_line( "pqr3stu8vwx"));
    }




    #[test]
    fn test_edge_case() {

    }


    #[test]
    fn test_all() {
        assert_eq!(281,process_file("debug.txt"))
    }
}