
struct Point {
    x:f64,
    y:f64,
}

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn len(&self) -> f64 {
        let dx = self.start.x - self.end.x;
        let dy = self.start.x - self.end.y;
        (dx * dx + dy * dy).sqrt()
    }
}

fn main() {
    let p1 = Point {x: 3.0, y: 4.0};
    let p2 = Point {x: 4.0, y: 10.0};
    let  line = Line {start:p1, end:p2};

    println!("length = {}", line.len());
}
