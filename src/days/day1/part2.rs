use std::fs;

pub fn main(debug: bool) {
    let file = fs::read_to_string("src/days/day1/data.txt").unwrap();

    let lines: u32 = file
        .lines()
        .map(|line| {
            let digits = find_digits(line);
            let first_digit = digits.first().unwrap().1;
            let last_digit = digits.last().unwrap().1;
            let final_number = join_digits(first_digit, last_digit);

            if debug {
                println!("line: {:?}", line,);
                println!("matches: {:?}", digits);
                println!("first: {:?}, last: {:?}", first_digit, last_digit);
                println!("final number: {:?}", final_number);
                println!("-----");
            }

            return final_number;
        })
        .sum();

    println!("Result: {:?}", lines);
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
