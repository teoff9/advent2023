//06.12.23 by Matteo Fava

use std::fs::read_to_string;

fn main() {
    let mut sum = 0;
    let mut words: Vec<String> = vec![];

    for l in read_to_string("words_to_n.txt").unwrap().lines() {
        words.push(l.to_string());
    } 

    for line in read_to_string("input.txt").unwrap().lines() {
        
        let mut line = line.to_string();

        for (i, number) in words.iter().enumerate() {
            let len = number.to_string().len();
            line = line.replace(number, &format!("{}{}{}", &number.to_string()[0..=0], i+1, &number.to_string()[len-1..len]));
        }

        sum += process_line(line);
    }

    println!("{}", sum);
}



fn process_line(line: String) -> i32 {
    let mut first : i32 = 0;
    let mut last : i32 = 0;
    let mut is_first: bool = true;

    println!("{}", line);
    
    for c in line.chars() {

        if c.is_numeric() && is_first {
            first = c.to_digit(10).unwrap() as i32;
            last = first;
            is_first = false;
        } else if c.is_numeric() {
            last = c.to_digit(10).unwrap() as i32;
        }
    }


    println!("{first}, {last}");
    first * 10 + last
}