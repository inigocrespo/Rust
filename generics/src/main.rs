
struct Point<T> {
    x: T,
    y: T,
}

struct Line<T> {
    start: Point<T>,
    end: Point<T>
}

fn generics() {
    let a = Point {x: 0, y: 0};
    let b = Point {x: 1, y: 3};

    let myline = Line {start: a, end: b};
}

fn main() {
    generics();
}
