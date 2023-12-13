use std::{fs::read_to_string, convert::TryInto};

fn main() {

    let mut input: Vec<String> = read_to_string("sample.txt").unwrap().lines().map(|l| l.to_string()).collect();
    let nums: Vec<i64> = input[0].trim().split(" ").map(|n| n.parse().unwrap()).collect();
    input.remove(0);
    let maps: [Vec<[i64; 3]>; 7] = input_to_map(input);
    let mut result = i64::MAX;
    
    for i in 0..nums.len() {
        if (i+1) % 2 == 1 {
            let n = process_seed(nums[i], &maps);
            if n < result {
                result = n;
            }
        } else {
            for n in nums[i-1]+1..nums[i-1]+nums[i] {
                let n = process_seed(n, &maps);
                if n < result {
                result = n;
                }
            }
        }
    }

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
