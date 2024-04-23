/*
   Determine if an array contains negative numbers and update the flag
   variable 'invalid_array' accordingly."
*/
fn main() {
    let array1 = [1, -2, 3, 4];
    
    /*
        let invalid_array = match array1 {
            [n,_,_,_] | [_,n,_,_] |
            [_,_,n,_] | [_,_,_,n] if n < 0 => {
                true
            }

            _ => false,
        };
    */

    let invalid_array = matches!(array1, [n,_,_,_] | [_,n,_,_] | [_,_,n,_] | [_,_,_,n] if n < 0);

    if invalid_array {
        println!("Array is invalid");
    } else {
        println!("Array is valid");
    }
}
