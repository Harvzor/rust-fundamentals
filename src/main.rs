struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    // len is a method on the Line class
    // &self is a reference to this class
    fn len(&self) -> f64 {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;

        (dx*dx+dy*dy).sqrt()
    }
}

fn main() {
    let line = Line {
        start: Point {
            x: 3.0,
            y: 3.0,
        },
        end: Point {
            x: 4.0,
            y: 4.0,
        },
    };
    
    println!("{}", line.len());
}
