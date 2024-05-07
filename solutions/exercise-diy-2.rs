// Exercise: Format floating point values in scientific notation

// Expected Output: Scientific notation (2 decimal places): 1.23e4

// Hints: Combine the concepts of precision (.2) and scientific notation (e) in your format string

fn main() {
    let float_value = 12345.6789;

    /* 
     * Print the floating-point value in scientific notation 
     * with 2 decimal places
     */
    println!("{:.2e}", float_value);
}
