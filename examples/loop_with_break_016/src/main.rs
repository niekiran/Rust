/*When break executes, the loop is immediately terminated, and the program 
continues executing from the statement after the loop.
*/
fn main() {
    let mut i = 0;

    loop {
        if i == 3 {
            break;
        }
        println!("i = {}", i);
        i += 1;
    }

    println!("loop ends");
}
