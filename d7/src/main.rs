mod d1;
mod d2;
fn main() {
    let start1 = std::time::Instant::now();
    d1::main();
    let time1 = start1.elapsed();
    println!("----");

    let start_d2 = std::time::Instant::now();

    d2::main();
    let time2 = start_d2.elapsed();

    println!("Time taken for d1: {:?}", time1);
    println!("Time taken for d2: {:?}", time2);
}
