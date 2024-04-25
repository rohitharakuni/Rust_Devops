fn area_of_rectangle(length: f64, width: f64) -> f64 {
    length * width
}

fn main() {
    let length = 5.0;
    let width = 3.0;
    let area = area_of_rectangle(length, width);
    println!("Area of the rectangle: {}", area);
}
