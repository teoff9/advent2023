//07.12.23 by Matteo Fava

use std::fs::read_to_string;

fn main() {

    let mut sum = 0;

    for (i, line) in read_to_string("input.txt").unwrap().lines().enumerate() {
        if process_line(line.to_string()) {
            sum += i+1;
        }
    }

    println!("{sum}");
}

fn process_line(line: String) -> bool {
    let mut n = 0;
    let mut flag = true;

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
                        if n >= 13 {
                            return false;
                        }
                    },
                    'g' => {
                        if n >= 14 {
                            return false;
                        }
                    },
                    'b' => {
                        if n >= 15 {
                            return false;
                        }
                    }
                    _ => continue
                }

                flag = true;
            }

            
        }
    }

    return true;

}