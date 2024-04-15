

fn calc_circle_area(_radius: f32) -> f32 {
    3.14 * 4.0 * 4.0
}

fn main() {
    let pi = std::f32::consts::PI;
    let _area = pi * 4.0 * 4.0;
    println!("{}", calc_circle_area(10.0));
}
