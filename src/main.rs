use std::fs;

fn main() {
    day1_part2();
}

fn day1_part2() {
    let file = fs::read_to_string("src/data/day_1.txt").unwrap();

    let lines: u32 = file
        .lines()
        .map(|line| {
            let digits = find_digits(line);
            let first_digit = digits.first().unwrap().1;
            let last_digit = digits.last().unwrap().1;
            let final_number = join_digits(first_digit, last_digit);

            println!("-----");
            println!("line: {:?}", line,);
            println!("matches: {:?}", digits);
            println!("first: {:?}, last: {:?}", first_digit, last_digit);
            println!("final number: {:?}", final_number);
            println!("-----");

            let string_number = vec![first_digit, last_digit].join("");
            println!("{:?}", string_number);

            return string_number.parse::<u32>().unwrap();
        })
        .sum();

    println!("{:?}", lines);
}

fn find_digits(line: &str) -> Vec<(usize, &str)> {
    let items = vec![
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];

    let mut matches = vec![];

    for item in items.into_iter() {
        let res: Vec<_> = line.match_indices(item).collect();

        for occurence in &res {
            matches.push((occurence.0, replace_if_string(occurence.1)));
        }
    }

    matches.sort_by(|a, b| a.0.cmp(&b.0));

    return matches;
}

fn replace_if_string(found: &str) -> &str {
    let map = vec![
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    for conversion in map {
        if conversion.0 == found {
            return conversion.1;
        }
    }

    return found;
}

fn join_digits(first_digit: &str, last_digit: &str) -> u32 {
    let string_number = vec![first_digit, last_digit].join("");
    return string_number.parse::<u32>().unwrap();
}

fn day1_part1() {
    let file = fs::read_to_string("src/data/aoc1.txt").unwrap();

    let lines: u32 = file
        .lines()
        .map(|line| {
            println!("{:?}", line);

            let chars: Vec<char> = line.chars().filter(|ch| ch.is_digit(10)).collect();
            println!("{:?}", chars);

            let first_char = chars.first().unwrap().to_string();
            let last_char = chars.last().unwrap().to_string();
            println!("first: {:?}, last: {:?}", first_char, last_char);

            let string_number = vec![first_char, last_char].join("");
            println!("{:?}", string_number);

            return string_number.parse::<u32>().unwrap();
        })
        .sum();

    println!("{:?}", lines);
}
