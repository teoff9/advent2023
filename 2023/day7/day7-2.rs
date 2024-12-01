use std::{fs::read_to_string, collections::HashSet, convert::TryInto};

#[derive(Debug)]
struct Hand {
    raw: String,
    cards: Vec<i8>,
    power: i8,
    bid: i32
}

impl Hand {
    fn new(line: String) -> Self {
        let mut line:Vec<String> = line.split(" ").map(|n| n.to_string()).collect();
        let bid = line[1].trim().parse::<i32>().unwrap();
        line.remove(1);
        let cards: Vec<i8> = line[0].chars().map(|c| {
            if c.is_numeric() {
                c.to_string().parse::<i8>().unwrap()
            } else {
                convert_symbol(c)
            }
        }).collect();
        let power = find_power(&cards);

        Self {
            raw: line.pop().unwrap(),
            cards,
            power,
            bid
        }
    }
}

fn main() {
    let mut hands: Vec<Hand> = read_to_string("sample.txt").unwrap().lines().map(|l| Hand::new(l.to_string())).collect();
    sort_hands(&mut hands);
    let mut answ = 0;
    for i in 0..hands.len() {
        answ += (i+1) as i32 * hands[i].bid;
    }
    
    println!("{}", answ);
}

fn sort_hands(hands: &mut Vec<Hand>) {
    for i in 0..hands.len() {
        let mut i_min = i;
        let mut min = hands[i].power;
        for k in i+1..hands.len() {
            if hands[k].power < min {
                i_min = k;
                min = hands[k].power;
            } else if hands[k].power == min {
                for (z, cm) in hands[i_min].cards.clone().into_iter().enumerate() {
                    if hands[k].cards[z] < cm {
                        i_min = k;
                        min = hands[k].power;
                        break;
                    } else if hands[k].cards[z] > cm {
                        break;
                    }
                }
            } 
        }
        hands.swap(i, i_min);
    }
}

fn convert_symbol(sy: char) -> i8 {
    match sy {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
         _ => panic!("Not a card")
    }
}

fn find_power(cards: &Vec<i8>) -> i8 {
    let mut j_times = 0;
    for i in 0..cards.len() {
        if cards[i] == 11 {
            j_times += 1;
        }
    }
    
    if j_times == 0 {
        let cs = cards.into_iter().collect::<HashSet<_>>();
    match cs.len().try_into().unwrap() {
        1 => 7,
        2 => {
            let mut i: i8 = 0;
            for card in cards {
                for unique in &cs {
                    if *unique == card {
                        i += 1;
                    }
                    break;
                }
            }
            match i {
                2..=3 => 5,
                _ => 6
            }
        },
        3 => {
            let mut max_i: i8 = 0;
            for unique in &cs {
                let mut i = 0;
                for card in cards {
                    if *unique == card {
                        i += 1;
                    }
                    if i>max_i { max_i = i}
                }
            }
            match max_i {
                3 => 4,
                _ => 3
            } 
        },
        4 => 2,
        5 => 1,
        _ => panic!("something isn't right")
    }
    }
}