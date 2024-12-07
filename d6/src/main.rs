use std::time::Instant;

mod d1;
mod d2;
fn main() {
    d1::main();

    let start_time = Instant::now();
    d2::main();
    let duration = start_time.elapsed();
    println!("Execution time: {:?}", duration);
}
