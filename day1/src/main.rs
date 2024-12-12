const INPUT: &str = include_str!("input.txt");

const PART2: bool = true;

fn main() {
    let arr = INPUT.split('\n').filter(|s| s.len() != 0).collect::<Vec<&str>>();

    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for line in arr.iter() {
        let numbers = line.split(' ')
                          .filter(|s| s.len() != 0)
                          .map(|n| n.parse::<i32>().unwrap())
                          .collect::<Vec<i32>>();

        left.push(numbers[0]);
        right.push(numbers[1]);
    }

    left.sort();
    right.sort();

    let result: i32;

    if PART2 {
        result = left.iter()
                     .map(|x| {
                         let occurances = right.iter()
                                               .filter(|y| **y == *x)
                                               .count() as i32;
                         x * occurances
                     }).sum::<i32>();
    } else {
        /* basically iterate over both Vecs */
        result = left.iter()
                      .zip(right.iter())
                      .map(|(l, r)| (l - r).abs())
                      .sum::<i32>();
    }

    println!("{}", result);
}
