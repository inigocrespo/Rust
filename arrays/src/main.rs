
use std::mem;

fn arrays() {
    let _x = [1, 2, 3, 4 , 5];

    let mut a:[i32;5] = [1, 2, 3, 4 , 5];

    println!("a has {} elements, first is {}", a.len(), a[0]);
    a[0] = 321;
    println!("a has {} elements, first is {}", a.len(), a[0]);

    println!("{:?}", a);

    if a == [321, 2, 3, 4 , 5] {
        println!("match")
    }

    let b = [1u16;10];
    for i in 0..b.len() {
        println!("{}", b[i]);
    }

    println!("b took up {} bytes", mem::size_of_val(&b));

    let mxt:[[f32;3];2] =
        [
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
        ];
    println!("{:?}", mxt);

    for i in 0..mxt.len() {
        for j in 0..mxt[i].len() {
            if i == j {
                println!("mxt[{}][{}] = {}", i, j, mxt[i][j]);
            }
        }
    }
}

fn main() {
    arrays()
}
