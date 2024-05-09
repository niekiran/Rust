/* String indexing*/

fn main() {
    let s1 = String::from("hello+∞+😊+ರ");
    
    let mut ch = s1.chars().next();
    if let Some(c) = ch {
        println!("{}", c)
    }

    /* Indexing using 'nth' method of Iterator */
    ch = s1.chars().nth(6);
    if let Some(c) = ch {
        println!("{}", c);
    }

      /* Since 100th index is not available , nth(100) returns 'None' */
    ch = s1.chars().nth(100);
    if let Some(c) = ch {
        println!("{}", c);
    } else {
        println!("No character found");
    }

}