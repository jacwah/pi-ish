extern crate rand;

use rand::Rng;

/// Compute if the point (x, y) is inside the radius of the origin.
fn circle_contains(radius: i32, x: i32, y: i32) -> bool {
   radius.pow(2) >= x.pow(2) + y.pow(2)
}

/// Estimate π using a Monte Carlo simulation.
fn estimate_pi(radius: i32, iterations: i32) -> f64 {
    let mut num_in_circle = 0;
    let sample = || rand::thread_rng().gen_range(-radius, radius);

    for _ in 0..iterations {
        if circle_contains(radius, sample(), sample()) {
            num_in_circle += 1;
        }
    }

    4.0 * (num_in_circle as f64) / (iterations as f64)
}

fn main() {
    let radius = 1_000;
    let iterations = 1_000_000;

    println!("π ≈ {}", estimate_pi(radius, iterations));
}


