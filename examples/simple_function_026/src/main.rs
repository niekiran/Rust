/* 
    Note:  It is mandatory to specify the types for all function parameters.
    Rust does not perform type inference for function parameter types 
*/

//Function which returns f64 value
fn convert_kg_to_grams(in_kg: f64) -> f64 {
    in_kg * 1000_f64
}

//Function which returns String there by transfers the ownership of the 'String' value to caller
fn concatenate_strings(first: &str, second: &str) -> String {
    first.to_string() + second
}

fn main() {
    //the types must match exactly when you call a function. 
    // 'as' used to convert 4 which is i32 to f64 value or just write 4_f64
    println!("{}", convert_kg_to_grams(4 as f64));
    println!("{}", concatenate_strings("Good", " Morning"))
}
