use std::fs;

pub fn main(debug: bool) {
    let file = fs::read_to_string("src/days/day1/data.txt").unwrap();

    let lines: u32 = file
        .lines()
        .map(|line| {
            let chars: Vec<char> = line.chars().filter(|ch| ch.is_digit(10)).collect();
            let first_digit = chars.first().unwrap();
            let last_digit = chars.last().unwrap();
            let string_number = format!("{}{}", first_digit, last_digit);

            if debug {
                println!("{:?}", line);
                println!("{:?}", chars);
                println!("first: {:?}, last: {:?}", first_digit, last_digit);
                println!("{:?}", string_number);
                println!("------");
            }

            return string_number.parse::<u32>().unwrap();
        })
        .sum();

    println!("Result: {:?}", lines);
}
