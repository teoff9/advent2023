use std::fs::read_to_string;

fn main() {

    let games: Vec<[i64;2]> = parse("input.txt");
    let answer = games.iter().fold(1, |acc, game|  acc*process_game(game));
    println!("{}", answer);
}

fn process_game(game: &[i64;2]) -> i64 {
    let mut wins = 0;
    for t in 1..=game[0] {
        if (game[0]-t)*t > game[1] {
            wins += 1;
        }
    }
    wins
}

fn parse(file_name: &str) -> Vec<[i64;2]> {
    let mut input: Vec<[i64;2]> = vec![];
    let ns: Vec<i64> = read_to_string(file_name).unwrap().lines().map(|n| n.to_string().trim().parse::<i64>().unwrap()).collect();
    for i in 0..ns.len()/2 {
        input.push([ns[i], ns[i+ns.len()/2]]);
    }
    input
}