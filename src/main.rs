extern crate rand;
extern crate num_cpus;

use rand::Rng;
use std::thread;

/// Compute if the point (x, y) is inside the radius of the origin.
fn circle_contains(radius: f64, x: f64, y: f64) -> bool {
   radius.powi(2) >= x.powi(2) + y.powi(2)
}

/// Estimate π using a Monte Carlo simulation.
fn estimate_pi(iterations: i32) -> f64 {
    let radius = 1.0;
    let mut rng = rand::thread_rng();
    let mut num_in_circle = 0;

    let mut sample = || rng.gen_range(-radius, radius);

    for _ in 0..iterations {
        if circle_contains(radius, sample(), sample()) {
            num_in_circle += 1;
        }
    }

    4.0 * (num_in_circle as f64) / (iterations as f64)
}

fn main() {
    let iterations = 1_000_000_000;
    let num_workers = num_cpus::get();

    let mut workers = vec![];

    for _ in 0..num_workers {
       workers.push(thread::spawn(move || {
          estimate_pi(iterations)
      }));
    }

    let estimation = workers.into_iter()
        .map(|worker| worker.join().unwrap())
        .sum::<f64>() / num_workers as f64;

    println!("π ≈ {}", estimation);
}


