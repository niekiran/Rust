struct Point {
    x: f32,
    y: f32,
}

/*
    The main difference between methods and
    associated functions is the presence of the 'self' keyword in methods.
*/
impl Point {
    // Method that takes ownership of self
    fn into_tuple(self) -> (f32, f32) {
        (self.x, self.y)
    }

    // Method that borrows self immutably
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    // Method that borrows self mutably
    fn translate(&mut self, dx: f32, dy: f32) {
        self.x += dx;
        self.y += dy;
    }

    // there is not 'self' keyword used here in the function arguments
    // This is associated function of the Struct Point
    fn from_tuple(coords: &(f32, f32)) -> Point {
        Point {
            x: coords.0,
            y: coords.1,
        }
    }
}

fn main() {
    //to call a method first yo u need an instance(p) of the struct Point
    let mut p = Point { x: 3.0, y: 4.0 }; 

    //distance_from_origin() method borrows 'p' immutably
    let dist = p.distance_from_origin();
    println!("Distance from origin: {}", dist);

    //translate() method borrows 'p' mutably
    p.translate(1.0, 1.0);
    println!("Translated point: ({}, {})", p.x, p.y);

    //into_tuple() methods owns the instance 'p'
    let tuple = p.into_tuple();
    println!("Point as tuple: {:?}", tuple);

    //Variable 'p' is not valid here

    let tuple = (10_f32, 20_f32);
    //Associated funtion is called on the struct itself, not on the instance of the struct.
    let q = Point::from_tuple(&tuple);
    println!("Point from tuple: ({}, {})", q.x, q.y);

}
