//03.12.2024 by Matteo Fava

use std::fs::read_to_string;

use regex::Regex;

fn main() {
    assert!(part_one("data/sample.txt") == 161);
    let ans1 = part_one("data/input.txt");
    println!("Part one: {ans1}");
    assert!(part_one("data/input.txt") == 169021493);

    assert!(part_two("data/sample.txt") == 48);
    let ans2 = part_two("data/input.txt");
    println!("Part two: {ans2}");
}

fn part_one(file: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut acc: i32 = 0;
    read_to_string(file).unwrap().lines().into_iter().for_each(|l| {
        re.captures_iter(l).for_each(|c| {
            acc += c[1].parse::<i32>().unwrap() * c[2].parse::<i32>().unwrap();
        });
    });
    acc
}

fn part_two(file: &str) -> i32 {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)").unwrap();
    let mut do_ = true;
    let mut acc: i32 = 0;
    read_to_string(file).unwrap().lines().into_iter().for_each(|l| {
        re.captures_iter(l).for_each(|c| {
            match &c[0] {
                "do()" => do_ = true,
                "don't()" =>  do_ = false,
                _ =>  if do_ {
                    let n: Vec<i32> = c[0].replace("mul(", "").replace(")", "").trim().split(",").map(|n| n.parse::<i32>().unwrap()).collect();
                    acc += n[0] * n[1];
                },
            }
        });
    });
    acc
}