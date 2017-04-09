extern crate rand;
extern crate num_cpus;
#[macro_use]
extern crate clap;

use rand::Rng;
use std::thread;

/// Compute if the point (x, y) is inside the radius of the origin.
fn circle_contains(radius: f64, x: f64, y: f64) -> bool {
    radius.powi(2) >= x.powi(2) + y.powi(2)
}

/// Estimate π using a Monte Carlo simulation.
fn estimate_pi(iterations: usize) -> f64 {
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

fn estimate_pi_multithreaded(iterations: usize, num_workers: usize) -> f64 {
    let mut workers = vec![];

    for _ in 0..num_workers {
        workers.push(thread::spawn(move || {
            estimate_pi(iterations)
      }));
    }

    workers.into_iter()
        .map(|worker| worker.join().unwrap())
        .sum::<f64>() / num_workers as f64
}

fn main() {
    let matches = clap::App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(clap::Arg::with_name("iterations")
             .short("n")
             .long("iterations")
             .takes_value(true))
        .get_matches();

    let iterations = value_t!(matches, "iterations", usize).unwrap_or(1000);
    let num_workers = num_cpus::get();

    println!("π ≈ {}", estimate_pi_multithreaded(iterations, num_workers));
}


