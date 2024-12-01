use std::fs::read_to_string;

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

fn main() {
    let pps = read_input("sample.txt");
    for i in 1..pps.len() {
        for j in 0..3 {
            pps[i].p[j] -= pps.[0].p[j];
            pps[i].v[j] -= pps.[0].v[j];
        }
    }


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

