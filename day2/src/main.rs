const INPUT: &str = include_str!("input.txt");
const DAY2: bool = false;

fn compute_difference(vec: &Vec<i32>) -> Vec<(usize, i32)> {
    let mut difference: Vec<(usize, i32)> = vec![];

    for i in 1..vec.len() {
        difference.push((i, vec[i] - vec[i - 1]));
    }

    difference
}

fn find_outlier_index(vec: &Vec<i32>) -> Option<usize> {
    let difference = compute_difference(vec).into_iter()
                                            .collect::<Vec<(usize, i32)>>();

    let (_, first_index) = difference[0];

    let sign = first_index.signum();

    for (i, num) in difference.iter() {
        if num.signum() != sign {
            return Some(*i);
        }
        if num.abs() > 3 || num.abs() < 1 {
            return Some(*i);
        }
    }

    None
}

fn is_safe(vec: &Vec<i32>) -> bool {
    if let Some(idx) = find_outlier_index(&vec) {
        if DAY2 {
            /* brute force */
            for i in 0..vec.len() {
                let mut copy = vec.clone();

                copy.remove(i);

                if find_outlier_index(&copy).is_none() {
                    return true;
                }
            }
            return false;
        } else {
            return false;
        }
    }

    true
}

fn main() {
    let arr = INPUT.trim()
                   .split('\n')
                   .map(|s| s.split(' ').map(|n| n.parse::<i32>().unwrap()).collect())
                   .collect::<Vec<Vec<i32>>>();

    /*
    for report in arr.iter() {
        println!("Safe: {} => {:?}", is_safe(&report), report);
    }
    */

    let safe_count = arr.iter()
                        .map(|a| is_safe(a))
                        .filter(|b| *b)
                        .count();

    println!("{}", safe_count);
}
