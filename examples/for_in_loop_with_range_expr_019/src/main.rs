/*Example of a for..in loop with a range expression, which is an iterator in Rust */
fn main() {
    // a range is an iterator in rust
    for i in -5..=5 {
        println!("{:2}", i);
    }
}
