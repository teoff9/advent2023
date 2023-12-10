use std::{fs::read_to_string, convert::TryInto};

fn main() {

    let mut input: Vec<String> = read_to_string("input.txt").unwrap().lines().map(|l| l.to_string()).collect();
    let numbers: Vec<i64> = input[0].trim().split(" ").map(|n| n.parse().unwrap()).collect();
    input.remove(0);
    let map: [Vec<[i64; 3]>; 7] = input_to_map(input);

    let result: i64 = numbers.iter().map(|n| process_seed(*n, &map)).min().unwrap();

    println!("{result}");
}

fn input_to_map(input: Vec<String>) -> [Vec<[i64; 3]>; 7] {
    let mut map = [vec![], vec![], vec![], vec![], vec![], vec![], vec![]];
    let mut i: usize = 0;
    for line in &input {
        if line == "map:" {
            i += 1;
        } else {
            let values: Vec<i64> = line.split(" ").map(|n| n.parse::<i64>().unwrap()).collect();
            map[i].push(values.try_into().unwrap());
        }
    }
    map
}

fn process_seed(mut n: i64, maps: &[Vec<[i64; 3]>; 7]) -> i64 {
    for map in maps {
        for range in map {
            if range[1] <= n && range[1]+range[2]-1 >= n {
                n = range[0] + n - range[1];
                break;   
            }
        }
    }

    n
}