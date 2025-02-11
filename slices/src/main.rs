
fn use_slice(slice: &mut [i32]) {
    println!("fist element = {}, len  = {}", slice[0], slice.len());
    slice[0] = 1234
}

fn slices() {
    let mut data = [1, 2, 3, 4, 5];
    use_slice(&mut data[1..4]);

    println!("{:?}", data)
}

fn main() {
    slices();
}
