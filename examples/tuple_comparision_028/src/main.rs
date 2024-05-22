
/*
    tuples can be compared lexicographically if all of their constituent elements 
    implement the 'PartialOrd' and 'PartialEq' traits. 
 */
fn main() {
    let tuple1 = (2, 3, 4);
    let tuple2 = (8, 3, 4);
    let tuple3 = (1, 2, 3);

   
    if tuple1 < tuple2 {
        println!("tuple1 is smaller");
    } else {
        println!("tuple2 is smaller")
    }
    
    if tuple1 == tuple3 {
        println!("Tuples equal")
    } else {
        println!("Tuples not equal")
    }
    
    let tuple4 = (1, "world x");
    let tuple5 = (1, "world z");
    let tuple6 = (1, "world x");

    if tuple4 > tuple5 {
        println!("tuple4 is bigger")
    } else {
        println!("tuple5 is bigger")
    }
    
    if tuple4 == tuple6 {
        println!("Tuples equal")
    } else {
        println!("Tuples not equal")
    }
}