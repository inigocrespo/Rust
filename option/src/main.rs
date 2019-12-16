fn main() {
    let x = 3.0;
    let y = 0.1;

    let result =
        if y != 0.0 { Some(x/y) } else { None };

    match result {
        Some(z) => println!("{}/{} = {}", x, y, z),
        None => println!("cannot divide by zero"),
    }

    if let Some(z) = result  {
        println!("{}/{} = {}", x, y, z)
    }
}
