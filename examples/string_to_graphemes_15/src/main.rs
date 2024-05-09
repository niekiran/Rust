use unicode_segmentation::UnicodeSegmentation;
fn main() {
    let computer_in_hindi = "कंप्यूटर";
    println!("Hindi word: {}", computer_in_hindi);

    
    print!("All characters of the string: ");
    for ch in computer_in_hindi.chars() {
        print!("{}",ch);
        print!("  ");
    }
    
    println!();
    
    println!("Total chars : {}",computer_in_hindi.chars().count());
    
    // Convert string to byte array
    let byte_array = computer_in_hindi.as_bytes();
    println!("Byte array: {:?}", byte_array);

    // Convert string to grapheme cluster iterator and print
    let graphemes = computer_in_hindi.graphemes(true);
    for grapheme in graphemes {
        println!("Grapheme: {}", grapheme);
    }
}