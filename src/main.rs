struct Point<T, V> {
    x: T,
    y: V,
}

struct Line<T> {
    start: Point<T, T>,
    end: Point<T, T>,
}

fn main() {
    let a: Point<u16, u16> = Point { x: 0, y: 0 };
    let b = Point { x: 1.2, y: 3.4 };

    // Doesn't work since Line start and end must be the same type.
    //let line = Line { start: a, end: b };
}
