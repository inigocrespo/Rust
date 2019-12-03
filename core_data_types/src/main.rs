use std::mem;

fn main() {
    let a:u8 = 1;
    println!("{}", a);

    let mut b:i8 = 12;
    b = 123;
    println!("{}", b);

    let mut c = 1234;
    println!("c = {} size = {}", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {} size = {}", c, mem::size_of_val(&c));

    let z:isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up to {} bytes, {}-bit os", z, size_of_z, size_of_z * 8 );

    let d = 'x';
    println!("d = {} size = {}", c, mem::size_of_val(&d));

    let e = 2.5;
    println!("e = {} size = {}", e, mem::size_of_val(&e));

    let g = false;
    println!("g = {} size = {}", g, mem::size_of_val(&g));


}
