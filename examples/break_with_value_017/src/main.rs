/*
 Example of 'break with value'.
 When i equals 3,  'break' statement is executed with the value i * 2.
 This exits the loop and returns the value 6 as the result of the loop.
 */
fn main() {
    let mut i = 0;

    let result = loop {
        if i == 3 {
            break i * 2;
        }
        i += 1;
    };

    println!("result = {}", result);
}

