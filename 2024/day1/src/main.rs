//01.12.24 by Matteo Fava

use std::fs::read_to_string;

fn main() {
    let ans1 = part_one("data/input.txt");
    println!("Part one: {ans1}");

    let ans2 = part_two("data/input.txt");
    println!("Part two: {ans2}");
}

fn load_data(file: &str) -> [Vec<i32>; 2] {
    let mut input: [Vec<i32>; 2] = [vec![], vec![]];
    read_to_string(file)
        .expect("Can't read input")
        .lines()
        .for_each(|l| {
            l.split("   ")
                .enumerate()
                .for_each(|(i, n)| input[i].push(n.parse().expect("Can't get number")))
        });
    input
}

fn part_one(file: &str) -> i32 {
    let mut input = load_data(file);
    let mut acc: i32 = 0;
    for i in 0..2 {
        input[i].sort();
    }
    input[0].iter().enumerate().for_each(|(i,n)| acc += (n-input[1][i]).abs());
    acc
}

fn part_two(file: &str) -> i32 {
    let input = load_data(file);
    let mut acc: i32 = 0;

    input[0].iter().for_each(|n| {
        let occ = input[1].iter().filter(|&x| x == n).count();
        acc += occ as i32 * n;
    });
    
    acc
}