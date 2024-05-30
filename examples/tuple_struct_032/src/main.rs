/*
   Note:
   If two tuple structs have the same sequence of field types,
   they are considered as different types, even if
   the field types have the same name and value
   struct Size(i32, i32, i32);
   struct Point(i32, i32, i32);
*/
//elements of the tuple struct do not have names
struct Size(i32, i32, i32);
struct Point(i32, i32, i32);

// function gets the struct as mutable reference 
fn refactor_point(point: &mut Point) {
    point.0 += 5;
    point.1 += 10;
}

//function gets the struct as value (struct moved here)
fn print_point(point: Point) {
    println!("x: {}, y: {}, z: {}", point.0, point.1, point.2);
}

fn main() {
    let size = Size(0, 0, 0);
    let mut origin = Point(0, 0, 0);

    refactor_point(&mut origin);
    print_point(origin);

    // print_point(size); //Error
}
