use std::fs;

pub fn main(debug: bool) {
    let file = fs::read_to_string("src/days/day2/data.txt").unwrap();


    file.lines().for_each(|line| {
        let list: Vec<&str> = line.split(": ").collect();

        let game = list.get(0).unwrap();
        let sets_string = list.get(1).unwrap();

        let sets = sets_string.split("; ");

        sets.enumerate().for_each(|(idx, set)| {
            let groups = set.split(", ");

            groups.for_each(|group| {
                println!("{}:{} {:?}", game, idx + 1, group);
            });
        });
    });
}
