use std::fs::read_to_string;

struct Card {
    winning: Vec<i32>,
    numbers: Vec<i32>
}

impl Card {
    fn new(line: String) -> Self {
        let parts: Vec<String> = line.split("|").map(|p| p.trim().to_string()).collect();
        let winning: Vec<i32> = parts[0].trim().replace("  ", " ").split(" ").map(|n| n.parse().unwrap()).collect();
        let numbers: Vec<i32> = parts[1].trim().replace("  ", " ").split(" ").map(|n| n.parse().unwrap()).collect();
        Self { winning, numbers }
    }

    fn process_line (&self) -> i32 {
        let mut wins = 0;

        for w in &self.winning {
            for n in &self.numbers {
                if n == w {
                    wins += 1;
                }
            }
        }
        if wins != 0 {
            2_i32.pow(wins-1)
        } else {
            0
        }
    }
}

fn main() {

    let sum: i32 = read_to_string("input.txt").unwrap().lines().map(|line| Card::new(line.to_string()).process_line()).sum();

    println!("{sum}");
}