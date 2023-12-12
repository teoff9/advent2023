use std::{fs::read_to_string, convert::TryInto};

fn main() {

    let mut input: Vec<String> = read_to_string("sample.txt").unwrap().lines().map(|l| l.to_string()).collect();
    let nums: Vec<i64> = input[0].trim().split(" ").map(|n| n.parse().unwrap()).collect();
    input.remove(0);
    let maps: [Vec<[i64; 3]>; 7] = input_to_map(input);
    let numbers: Vec<[i64; 2]> = process_nums(nums);
    let low_location = process_seeds(numbers, &maps);

    println!("{}", low_location);
}


fn process_seeds(mut numbers: Vec<[i64; 2]>, maps: &[Vec<[i64; 3]>; 7]) -> i64 {
    let mut res = i64::MAX;
    let mut results: Vec<[i64;2]> = vec![];

    for _i in 0..numbers.len() {
        let mut num: Vec<[i64;2]> = vec![numbers.pop().unwrap()];
        dbg!(&num);
        for map in maps {
                for i in 0..num.len() {
                    for range in map {

                    if num[i][0] >= range[1] && num[i][1] <= range[0]+range[2]-1 {
                        num[i][0] = range[0]+num[i][0]-range[1];
                        num[i][1] = range[0]+num[i][1]-range[1];
                        
                    } else if num[i][0] >= range[1] && num[i][0] <= range[0]+range[2]-1 && num[i][1] > range[0]+range[2] {
                        num.push([range[1] + range[2], num[i][1]]);
                        num[i][0] = range[0]+num[i][0]-range[1];
                        num[i][1] = range[0]+range[2]-1;
    
                    } else if  num[i][0] <= range[1] && num[i][1] >= range[1] && num[i][1] <= range[0]+range[2]-1 {
                        num.push([num[i][0],range[1] -1]);
                        num[i][0] = range[0] + range[2] - 1;
                        num[i][1] = range[0]+num[i][1]-range[1];

                    } else if num[i][0] < range[1] && num[i][1] > range[1]+range[2]-1 { 
                        num.push([num[i][0], range[1]-1]);
                        num.push([range[1]+range[2], num[i][1]]);
                        num[i][0] = range[0];
                        num[i][1] = range[0]+range[2]-1;

                    }
                }
                dbg!(&num);
                println!();
            }
        }
        //append processed range to results
        for _i in 0..num.len() {
            results.push(num.pop().unwrap());
        }
    }

    for n in &results {
        if n[0] < res {
            res = n[0];
        }
    }
    res
}

fn process_nums(nums: Vec<i64>) -> Vec<[i64; 2]> {
    let mut a: i64 = 0;
    let mut numbers: Vec<[i64; 2]> = vec![];
    for i in 0..nums.len() {
        if (i+1) % 2 == 1 {
            a = nums[i];
        } else {
            numbers.push([a, a+nums[i]-1]);
            }
        }
    numbers
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