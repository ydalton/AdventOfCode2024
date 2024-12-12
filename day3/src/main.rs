const INPUT: &str = include_str!("./test2.txt");

use regex::Regex;

fn get_good_ranges() {
    let do_dont_regex = Regex::new(r"do\(\)|don't\(\)").unwrap();

    for c in do_dont_regex.captures_iter(INPUT) {
        let _match = c.get(0).unwrap();
        let out = vec![(0, 0)];
        match _match.as_str() {
            "do()" => println!("do: {}", _match.start()),
            "don't()" => println!("don't: {}", _match.start()),
            &_ => todo!(),
        }
    }
}

fn do_aoc(day_two: bool) -> i32 {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    get_good_ranges();

    let sum = regex
        .captures_iter(INPUT)
        .map(|c| {
            let is_disabled = false;
            let word_match = c.get(1).unwrap();
            let start = word_match.start();
            //println!("match: {} {}", start, end);
            let (_, [x_str, y_str]) = c.extract();
            let x: i32 = x_str.parse().unwrap();
            let y: i32 = y_str.parse().unwrap();

            if is_disabled {
                0
            } else {
                x * y
            }
        })
        .sum::<i32>();

    sum
}

fn main() {
    println!("{:?}", INPUT.split('\n').collect::<Vec<&str>>());
    let _ = do_aoc(false);
}
