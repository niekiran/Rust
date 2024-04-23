/* Use a match statement to check the values of the tuple against several different patterns, 
each of which corresponds to a specific action to take.
If the second item in the tuple is negative, the code prints a message indicating that the second item is negative and takes "action 1".
If both items in the tuple are zero, the code prints a message indicating that both items are zero and takes "action 2".
If the tuple does not match either of these patterns, the code prints a message indicating that "all fine".
*/
fn main() {
    //imagine you get the point information from somewhere else
    let point = (0, 0);

    match point {
        // Pattern 1: Any first element and a negative second element.
        // `_` means we don't care about the value of the first element of the tuple
        (_, y) if y < 0 => {
            println!("Second element is negative:{}", y);
            println!("Take action 1");
        }

        // Pattern 2: Both elements of the tuple are exactly (0, 0)
        (0, 0) => {
            println!("Point is zero");
            println!("Take action 2");
        }

        // Default case: If none of the above patterns match
        _ => println!("All fine"),
    }
}
