// Exercise: Print a scientific notation number as a Floating Point Value Expected

// Expected output: 314.8 (Value in decimal format)

fn main() {
    //floating point variable in scientific notation
    let float_value = 3.148e2; //equivalent to 3.14 * 10^2

    // Print the above value in non-scientific format of floating value with 1 decimal place
    println!("{:.1}", float_value);
}
