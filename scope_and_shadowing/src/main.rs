fn scope_and_shadowing() {
    let a = 123;

    {
        let b = 456;
        println!("b = {}", b);

        let a = 777;
        println!("a inside = {}", a);

    }

    println!("a = {}", a);
}

fn main() {
    scope_and_shadowing()
}
