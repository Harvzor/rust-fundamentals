struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let p = Point {
        x: 3.0,
        y: 4.0,
    };

    println!("point p is at ({}, {})", p.x, p.y);
}
