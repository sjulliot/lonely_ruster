use lonely_ruster::find_lonely_time;
use lonely_ruster::combinations::Combinations;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;

pub(crate) fn main() {
    // Set the number of threads to 4
    ThreadPoolBuilder::new().num_threads(4).build_global().unwrap();

    let n = 4; // Number of runners
    let max_speed = 10; // Maximum speed

    let combinations = Combinations::new(n, max_speed);

    combinations.par_bridge().for_each(|combo| {
        if let Some(t) = find_lonely_time(&combo) {
            println!("Speeds: {:?}, Smallest lonely time: {}", combo, t);
        } else {
            println!("Speeds: {:?}, No solution found.", combo);
        }
    });
}