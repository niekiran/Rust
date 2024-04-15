
/* Activate this code block and run 'cargo clippy' and 
observe the errors/suggestions
    
fn main() {
    let pi = 3.14;
    let area = pi * 4.0 * 4.0;
    println!("{}", area);
}

*/

/* This code block generates warnings related to unused variabels 
   and unused functions and when you run the command 'cargo fix --allow-dirty' 
   some warnings are auto fixed 

fn calc_circle_area(radius: f32) -> f32 {
    3.14 * 4.0 * 4.0
}

fn main() {
    let pi = 3.14;
    let area = pi * 4.0 * 4.0;
    println!("{}", calc_circle_area(10.0));
}
 */

 /* 
    Running 'cargo build' does not encounter any issues with this code. 
    but when you run 'cargo clippy', it suggests using iterators, 
    which is the idiomatic way to handle iteration in Rust.
 */
 fn main() {
    let array = [1, 2, 3];
    for i in 0..=2 {
        println!("{}", array[i]);
    }
 }