use std::fs;

fn main() {
    day1_part2();
}

fn day1_part2() {
    let file = fs::read_to_string("src/data/aoc1.txt").unwrap();

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

fn replace_word_with_digit(line: &str) -> &str {
    let string_digits = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    return "";
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
