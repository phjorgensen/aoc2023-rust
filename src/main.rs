use std::fs;

fn main() {
    day1_part2();
}

fn day1_part2() {
    let file = fs::read_to_string("src/data/day_1.txt").unwrap();

    let lines: u32 = file
        .lines()
        .map(|line| {
            println!("{:?}", line);

            let new_line = replace_word_with_digit(line);
            println!("{:?}", new_line);

            let chars: Vec<char> = new_line.chars().filter(|ch| ch.is_digit(10)).collect();
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

fn replace_word_with_digit(line: &str) -> String {
    let mut new_string: String = line.into();

    let items = vec![
        "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7",
        "eight", "8", "nine", "9",
    ];

    let first = find_first(line, items);
    println!("first: {:?}", first);

    let reversed = line.chars().rev().collect::<String>();
    let last = find_first(reversed.as_str(), items);
    println!("last: {:?}", last);

    return new_string;
}

fn find_first(line: &str, items: Vec<&str>) -> &str {
    let mut matches = vec![];

    for item in items.into_iter() {
        match line.find(item) {
            Some(idx) => matches.push((idx, replace_if_string(item))),
            None => continue,
        };
    };

    println!("line: {:?}, matches: {:?}", line, matches);

    matches.sort_by(|a, b| a.0.cmp(&b.0));

    return matches.pop().unwrap().1;
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
