mod days;

use std::env::args;

fn main() {
    let day = get_argument_with_value("--day", "Missing day number");
    let part = get_argument_with_value("--part", "Missing day part number");
    let debug = get_bool_argument("--debug");

    match (day.as_ref(), part.as_ref()) {
        ("1", "1") => days::day1::part1::main(debug),
        ("1", "2") => days::day1::part2::main(debug),
        _ => panic!("Day or part not found!"),
    };
}

fn get_argument_with_value(argument_name: &str, expect_string: &str) -> String {
    return args()
        .position(|arg| arg.eq(argument_name))
        .and_then(|idx| args().nth(idx + 1))
        .expect(expect_string);
}

fn get_bool_argument(argument_name: &str) -> bool {
    return args()
        .find(|arg| arg.eq(argument_name))
        .and_then(|_| Some(true))
        .unwrap_or(false);
}
