/*
    Example demonstrates different ways of iteration in Rust, 
    by immutable borrow, by mutable borrow, and by value. 
*/
fn main() {
    let mut words = [
        "hello".to_string(),
        "World".to_string(),
        "how".to_string(),
        "are".to_string(),
        "you".to_string(),
    ];

    //iterate by immutable borrrow (read only iteration)
    for s in &words {
        println!("{s}");
    }

    //iterate by mutable borrow (you can modify the elements)
    for s in &mut words {
        if s == "hello" {
            s.push_str(" good morning");
        }
        println!("{s}");
    }

    //iterate by value 
    // here, for loop consumes the array 'words'. i.e array 'words' is moved(It doesn't implement the copy trait)
    for s in words {
        println!("{s}");
    }

    //cannot access moved array
    //println!("{:?}", words); //Error
}
