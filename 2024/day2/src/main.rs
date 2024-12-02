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
        if v[0] > v[v.len() - 1] {
            v.reverse();
        }

        if v.windows(2)
            .all(|w| condition(0, 1, w))
        {
            acc += 1;
        }
    });
    acc
}

fn part_two(file: &str) -> i32 {
    let mut input = load_data(file);
    let mut acc = 0;
    let mut t = 0;
    let mut i: usize = 0;
    let mut j: usize = 1;

    input.iter_mut().for_each(|v|{
        t = 0;
        i = 0;
        j = 1;
        if v[0] > v[v.len() - 1] {
            v.reverse();
        }
        dbg!(&v);
        while j < v.len() {
            if !condition(i, j, &v) {
                t += 1;
                if j+1 > v.len() {
                    break;
                } else {
                    j += 1;
                }
                if !condition(i, j, &v) {
                    t += 1;
                    break;
                } else {
                    i = j;
                    j += 1;
                }
            } else {
                i += 1;
                j += 1;
            }
        }
        dbg!(t);
        if t <= 1 {
            acc += 1;
        }

    });
    
    dbg!(acc);
    acc
}

fn condition(i: usize, j: usize, w: &[i32]) -> bool {
    w[i] < w[j] && (1..=3).contains(&(w[i] - w[j]).abs())
}
