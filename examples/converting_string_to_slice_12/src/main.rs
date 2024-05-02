

fn print_string(arg: &str) {
    println!("{}", arg);
}

fn main() {

    /*Converting a String into slice type */
    let mut s: String = String::from("Hello");
    let slice = &s; //type of 'slice' is &String 
    print_string(slice); //Rust automatically converts &String to &str
    let slice2 = s.as_str(); //type of 'slice2' is &str
    let slice3 = s.as_mut_str(); //type of 'slice3' is &mut str
    // Modify the content of the original string through slice3
    slice3.make_ascii_uppercase();
    print_string(&s); // Prints the orginal String 's' modified to "HELLO"
    
    /*Converting string slice into String*/
    let s = "Good Morning"; //type of 's' is '&str'
    let string1 = s.to_string(); //type of string1 is 'String'
    let string2 = String::from(s); //type of string2 is 'String'

}