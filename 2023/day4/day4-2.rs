use std::fs::read_to_string;

struct Card {
    times: i32,
    winning: Vec<i32>,
    numbers: Vec<i32>
}

impl Card {
    fn new(line: String) -> Self {
        let parts: Vec<String> = line.split("|").map(|p| p.trim().to_string()).collect();
        let winning: Vec<i32> = parts[0].trim().replace("  ", " ").split(" ").map(|n| n.parse().unwrap()).collect();
        let numbers: Vec<i32> = parts[1].trim().replace("  ", " ").split(" ").map(|n| n.parse().unwrap()).collect();
        Self { times: 1,winning, numbers }
    }

    fn process_one (&self) -> i32 {
        let mut wins = 0;
        for w in &self.winning {
            for n in &self.numbers {
                if n == w {
                    wins += 1;
                }
            }
        }
        wins
    }
}


fn main() {

    let sum: i32 = process_cards(read_to_string("input.txt").unwrap().lines().map(|line| Card::new(line.to_string())).collect());

    println!("{sum}");
}

fn process_cards(mut cards: Vec<Card>) -> i32 {
        
        for i in 0..cards.len()  {
            let n = cards[i].process_one();
            for j in i+1..i+1+n as usize {
                cards[j].times += cards[i].times;
            }

        } 

        let mut s = 0;
        cards.iter().for_each(|c| s += c.times);
        s
}