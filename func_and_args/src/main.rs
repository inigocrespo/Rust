
fn print_value(x:i32) {
    println!("{}", x);
}

fn increase(x: &mut i32) {
    *x += 1;
}

fn product(x:i32, y:i32) -> i32 {
    x * y
}

fn functions() {
    print_value(33);

    let mut z = 1;
    increase(&mut z);
    println!("{}", z) ;

    let a = 3;
    let b = 4;
    let p = product(a, b);
    println!("{} * {} = {}", a, b, p) ;

}

fn main() {
    functions();
}
