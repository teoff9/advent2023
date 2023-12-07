use std::fs::read_to_string;

fn main() {

    let mut sum = 0;

    for line in read_to_string("input.txt").unwrap().lines() {
        sum += process_line(line.to_string());
    }

    println!("{sum}");
}

fn process_line(line: String) -> i32 {
    let mut n = 0;
    let mut flag = true;
    let mut maxs = [0; 3]; //[r,g,b]
    let mut acc = 1;

    for set in line.split(';') {
        for c in set.chars() {
            if c.is_numeric() && flag {
                n = c.to_digit(10).unwrap() as i32;
                flag = false;
            } else if c.is_numeric() {
                n = 10*n + c.to_digit(10).unwrap() as i32;
            } else {
                match c {
                    'r' => {
                        if n > maxs[0] {
                            maxs[0] = n;
                        }
                    },
                    'g' => {
                        if n > maxs[1] {
                            maxs[1] = n;
                        }
                    },
                    'b' => {
                        if n > maxs[2] {
                            maxs[2] = n;
                        }
                    }
                    _ => continue
                }
                flag = true;
            }

            
        }
    }

    for a in maxs {
        acc *= a;
    }

    acc
}