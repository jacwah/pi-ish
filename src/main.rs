extern crate rand;

use rand::Rng;

fn circle_contains(radius: i32, x: i32, y: i32) -> bool {
   radius.pow(2) >= x.pow(2) + y.pow(2)
}
    

fn main() {
    let radius = 1_000;
    let iterations = 1_000_000;

    let mut num_in_circle = 0;

    for _ in 0..iterations {
        if circle_contains(radius, 
                          rand::thread_rng().gen_range(-radius, radius),
                          rand::thread_rng().gen_range(-radius, radius))
        {
            num_in_circle += 1;
        }
    }

    let estimation = 4.0 * (num_in_circle as f64) / (iterations as f64);

    println!("π ≈ {}", estimation);
}
