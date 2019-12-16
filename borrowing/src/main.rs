
fn main() {

    let print_vector = |x:&Vec<i32>| {
        println!("{}", x[0])
    };

    let v = vec![1,2 ,3];
    print_vector(&v);

    println!("v[0] = {}", v[0]);

    let mut a = 40;

    let b = &mut a;
    *b += 2;

    println!("b = {}", b);
    println!("a = {}", a);

    let mut z = vec![1,2,3];
    for i in &z {
        println!("i = {}", i);
        z.push(5)
    }

}
