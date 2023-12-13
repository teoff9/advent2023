use std::{fs::read_to_string, convert::TryInto};

fn main() {

    let mut input: Vec<String> = read_to_string("input.txt").unwrap().lines().map(|l| l.to_string()).collect();
    let nums: Vec<i64> = input[0].trim().split(" ").map(|n| n.parse().unwrap()).collect();
    input.remove(0);

    let maps: [Vec<[i64; 3]>; 7] = input_to_map(input);
    let numbers: Vec<[i64; 2]> = process_nums(nums);
    
    let res = process_seeds(numbers, &maps);
    println!("{}", res);
}


fn process_seeds(numbers: Vec<[i64; 2]>, maps: &[Vec<[i64; 3]>; 7]) -> i64 {
    let mut res = i64::MAX;
    
    for seed in numbers {
        let mut to_process: Vec<[i64;2]> = vec![seed];
        let mut processed: Vec<[i64;2]> = vec![];

        for map in maps {
            to_process.append(&mut processed);
            while !to_process.is_empty() {
                let mut i = 0;
                'num: while i < to_process.len() {
                    let mut unchanged = false;
                    for range in map {
                        let case = interjection(&to_process[i], range);
                        match case {
                            1..=3 => {
                                let mut new_n = split(&to_process[i], range, case);
                                to_process.remove(i);
                                to_process.append(&mut new_n);
                                break 'num;
                            },
                            4 => {
                                processed.push([range[0]+to_process[i][0]-range[1], range[0]+to_process[i][1]-range[1]]);
                                to_process.remove(i);
                                break 'num;
                            },
                            _ => {
                                unchanged = true;
                            }
                        }

                    }
                    if unchanged {
                        processed.push(to_process[i]);
                        to_process.remove(i);
                        i += 1;
                    }
                }
            }
        }

        for n in processed {
            if n[0] < res {
                res = n[0];
            }
        }
    }
    res
}

fn split(num: &[i64;2], range: &[i64;3], case: i8) -> Vec<[i64;2]> {
    let mut new_n: Vec<[i64;2]> = vec![];
    match case {
        1 => {
            new_n.push([num[0], range[1]+range[2]-1]);
            new_n.push([range[1]+range[2], num[1]]);
        },
        2 => {
            new_n.push([num[0], range[1]-1]);
            new_n.push([range[1], num[1]]);
        },
        3 => {
            new_n.push([num[0], range[1]-1]);
            new_n.push([range[1], range[1]+range[2]-1]);
            new_n.push([range[1]+range[2], num[1]]);
        },
        _ => {}
    }
    new_n
}

fn interjection(values: &[i64;2], range: &[i64;3]) -> i8 {
    if values[0] > range[1] && values[0] < range[1]+range[2]-1 && values[1] > range[1]+range[2]-1 {
        return 1; //parzialmente maggiore
    } else if values[0] < range[1] && values[1] > range[1] && values[1] < range[1]+range[2]-1 {
        return 2; //parzialmente minore
    } else if values[0] < range[1] && values[1] > range[1]+range[2]-1 {
        return 3; //incluso 
    } else if values[0] >= range[1] && values[1] <= range[1]+range[2]-1 {
        return 4;
    } else {
        0
    }
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