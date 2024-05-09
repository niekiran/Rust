/* Displaying byte representation of a unicode String */
fn main() {
    let message = String::from("hello+∞+😊+ರ");
    
    /* Accessing String as bytes immutably */
    let byte_slice: &[u8] = message.as_bytes();
   
    for byte in byte_slice {
        print!("{:#X}\t", byte);
    }

    /* String to Vector of bytes conversion */
    let byte_array  = message.into_bytes();
    println!("\r{:?}", byte_array);
     //println!("{}", message); //Error
    
  }