//if let .. else if let
fn main() {
    let point = (4, 6);
    
    if let (0, 0) = point {
        println!("Point is at the origin");
    } else if let (_, y @ 1..=4) = point {
        println!("{} is within the range 1..4", y);
    } else {
        println!("something else");
    }
}