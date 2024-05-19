/*'step_by' is a method of the Iterator trait in rust, and it allows  
to iterate over a range with a specified step size. */
fn main() {
    for i in (0..100).step_by(2) {
        println!("{:2}", i);
    }
  }