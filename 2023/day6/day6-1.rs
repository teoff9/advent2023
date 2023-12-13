use std::fs::read_to_string;

fn main() {

    let games: Vec<[i32;2]> = parse("input.txt");
    let answer = games.iter().fold(1, |acc, game|  acc*process_game(game));
    println!("{}", answer);
}

fn process_game(game: &[i32;2]) -> i32 {
    let mut wins = 0;
    for t in 1..=game[0] {
        if (game[0]-t)*t > game[1] {
            wins += 1;
        }
    }
    wins
}

fn parse(file_name: &str) -> Vec<[i32;2]> {
    let mut input: Vec<[i32;2]> = vec![];
    let ns: Vec<i32> = read_to_string(file_name).unwrap().split("|").map(|n| n.to_string().trim().parse::<i32>().unwrap()).collect();
    for i in 0..ns.len()/2 {
        input.push([ns[i], ns[i+ns.len()/2]]);
    }
    input
}