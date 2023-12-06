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

    let first = find_first(line);
    println!("{:?}", first);

    // Can't reverse, because one will become eno.
    let reversed = line.chars().rev().collect::<String>();
    let last = find_first(reversed.as_str());
    println!("{:?}", last);

    return new_string;
}

fn find_first(line: &str) -> &str {
    // Still does not find the first match in the word, just the first match in the items array.
    let items = vec![
        "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7",
        "eight", "8", "nine", "9",
    ];

    let mut found = false;
    let mut found_item: &str = "";

    items.into_iter().for_each(|item| {
        if found {
            return;
        }

        let found_it = line.find(item).unwrap_or(1000);

        if found_it != 1000 {
            found = true;
            found_item = item;
        }
    });

    return found_item;
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
