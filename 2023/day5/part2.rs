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


fn process_seeds(numbers: Vec<[i64; 2]>, maps: &[Vec<[i64; 3]>; 7]) -> i64 {
    let mut res = i64::MAX;
    let mut results: Vec<[i64;2]> = vec![];

    for j in 0..numbers.len() {
        let mut to_process: Vec<[i64; 2]> = vec![numbers[j]];
        let mut num: Vec<[i64; 2]> = vec![];

        for map in maps {
            to_process.append(&mut num);

            while !to_process.is_empty() {
                dbg!(&to_process);
                for i in 0..to_process.len() {
                    for range in map {
                        if to_process[i][0] >= range[1] && to_process[i][1] <= range[0]+range[2]-1 {
                            num.push([range[0]+to_process[i][0]-range[1], 
                                    range[0]+to_process[i][1]-range[1]]);
                            to_process.remove(i);
                        } else {
                            
                            match interjection(&to_process[i], range) {
                                1 => {
                                    to_process.push([range[1]+range[2], to_process[i][1]]);
                                    to_process[i][1] = range[1]+range[2]-1;
                                },
                                2 => {
                                    to_process.push([range[1], to_process[i][1]]);
                                    to_process[i][1] = range[1]-1;
                                },
                                3 => {
                                    to_process.push([range[1], range[1]+range[2]-1]);
                                    to_process.push([range[1]+range[2], to_process[i][1]]);
                                    to_process[i][1] = range[1]-1;
                                },
                                _ => {
                                    num.push(to_process[i]);
                                    to_process.remove(i);
                                    println!(" case 0");
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
        results.append(&mut num);
    }

    for n in &results {
        if n[0] < res {
            res = n[0];
        }
    }

    res
}


fn interjection(values: &[i64;2], range: &[i64;3]) -> i8 {
    if values[0] > range[1] && values[0] < range[1]+range[2]-1 && values[1] > range[1]+range[2]-1 {
        return 1; //parzialmente maggiore
    } else if values[0] < range[1] && values[1] > range[1] && values[1] < range[1]+range[2]-1 {
        return 2; //parzialmente minore
    } else if values[0] > range[1] && values[1] < range[1]+range[2]-1 {
        return 3; //incluso
    }

    dbg!(values, range[1], range[1]+range[2]-1);
    0
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