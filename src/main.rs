mod bin; 

use std::time::Instant;

use bin::solve_all;

fn main() {
    let timer = Instant::now();
    solve_all();
    println!("Total Time Elapsed: {:?}", timer.elapsed());
}
