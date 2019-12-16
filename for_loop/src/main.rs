
fn for_loop() {
    for x in 1..11 {
        println!("x = {}", x)
    }

    for (pos, y) in (30..41).enumerate() {
        println!("pos = {} value = {}", pos, y)
    }
}


fn main() {
    for_loop();
}
