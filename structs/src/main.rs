struct Point {
    x: f64,
    y: f64
}

struct Line {
    _start: Point,
    _end: Point
}

fn structures() {
    let p = Point{x: 0.0, y:0.0};
    println!("point is at {{{}, {}}}", p.x, p.y);

    let e = Point{x: 1.1, y: 1.1};

    let _line = Line{_start: p, _end: e};
}


fn main() {
    structures()
}
