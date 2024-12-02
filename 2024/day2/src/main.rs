//02.12.2024 by Matteo Fava

use std::fs::read_to_string;

fn main() {
    assert!(part_one("data/sample.txt") == 2);
    let ans1 = part_one("data/input.txt");
    println!("Part one: {ans1}");
    assert!(ans1 == 564);

    assert!(part_two("data/sample.txt") == 4);
    let ans2 = part_two("data/input.txt");
    println!("Part two: {ans2}");
    assert!(ans2 == 604);
}

fn load_data(file: &str) -> Vec<Vec<i32>> {
    let data = read_to_string(file).expect("Can't read input");
    data.lines()
        .map(|l| {
            l.split(" ")
                .map(|n| n.parse().expect("Can't get number"))
                .collect()
        })
        .collect()
}

fn part_one(file: &str) -> i32 {
    let mut input = load_data(file);
    let mut acc = 0;

    input.iter_mut().for_each(|v| {
        if is_valid(&v) {
            acc += 1;
        }
    });
    acc
}

fn part_two(file: &str) -> i32 {
    let mut input = load_data(file);
    let mut acc = 0;

    'outer: for v in input.iter_mut() {
        if is_valid(v) {
            acc += 1;
        } else {
            for j in 0..v.len() {
                let mut w = v.clone();
                w.remove(j);
                if is_valid(&w) {
                    acc += 1;
                    continue 'outer;
                }
            }
        }
    }
    acc
}

fn is_valid(v: &[i32]) -> bool {
    let direction = v[0] < v[1];
    v.windows(2)
        .all(|w| direction == (w[0] < w[1]) && (1..=3).contains(&(w[0] - w[1]).abs()))
}
