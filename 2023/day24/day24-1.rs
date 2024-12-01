use std::fs::read_to_string;

fn main() {
    let pps = read_input("input.txt");
    let mut c = 0;
    let bmax: f64 = 400000000000000.0;
    let bmin: f64 = 200000000000000.0;
    for i in 0..pps.len(){
        for j in i..pps.len() {
            if i != j {
                if intercept_2d(&pps[i], &pps[j], bmin, bmax) {
                    c += 1;
                }
            }
        }
    }
    println!("{c}");
}

#[derive(Debug)]
struct Particle {
    p: [i64; 3],
    v: [i64; 3]
}

impl Particle {
    fn new(p: [i64;3], v:[i64;3]) -> Self {
        Self {p,v}
    }
}

fn intercept_2d(p1: &Particle, p2: &Particle, bounds_min: f64, bounds_max: f64) -> bool {
    let mut matrix = [[0_f64;3];2];
    for i in 0..2 {
        matrix[i] = [p1.v[i] as f64,-p2.v[i] as f64,(p2.p[i]-p1.p[i]) as f64];
    }
    
    let ts = match solve_matrix(&mut matrix){
        None => return false,
        Some(t) => t
    };

    let mut xy = [0.0f64;2];
    for i in 0..2 {
        xy[i] = (p1.p[i] as f64) + p1.v[i] as f64 *ts[0];
    }
    
    ts[0] > 0.0 && ts[1] > 0.0 && xy[0] > bounds_min && xy[1] > bounds_min && xy[0] < bounds_max && xy[1] < bounds_max 
     
}

fn solve_matrix(matrix: &mut [[f64;3];2]) -> Option<[f64;2]> {
    
    let m = matrix[0][0];
    // rendi la prima riga 1 b/a c/a
    if m != 0.0 {
        for j in 0..3 {
            matrix[0][j] /= m;
        }
    }
    //sottrai alla seconda riga rendendola 0 e-bd/a f-bd/a
    let n = matrix[1][0];
    for k in 0..3{
        matrix[1][k] -= n*matrix[0][k];
    }
    //normalizza la seconda riga a 0 1 ans
    let f = matrix[1][1];
    for h in 1..3 {
        matrix[1][h] /= f;
    }
    //rendi prima riga 1 0 ans
    let n = matrix[0][1];
    for l in 1..3 {
        matrix[0][l] -= n*matrix[1][l];
    }
    Some([matrix[0][2], matrix[1][2]])
}

fn read_input(filename: &str) -> Vec<Particle> {
    let mut particles: Vec<Particle> = vec![];

    // Process each line
    for (line_number, line) in read_to_string(filename).unwrap().lines().enumerate() {
        let mut p: [i64; 3] = [0; 3];
        let mut v: [i64; 3] = [0; 3];

        // Split the line into whitespace-separated tokens
        let nums: Vec<i64> = match line.split_whitespace()
            .map(|n| n.parse::<i64>())
            .collect::<Result<Vec<_>, _>>() {
                Ok(nums) => nums,
                Err(e) => panic!("Failed to parse line {}: {}", line_number + 1, e),
            };

        // Ensure we have exactly 6 numbers per line
        if nums.len() != 6 {
            panic!("Line {} does not contain exactly 6 integers: {:?}", line_number + 1, line);
        }

        // Assign numbers to position and velocity arrays
        for i in 0..3 {
            p[i] = nums[i];
            v[i] = nums[i + 3];
        }

        // Create and add the particle to the list
        particles.push(Particle::new(p, v));
    }

    particles
}