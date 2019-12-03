
const MEANING_OF_LIFE:u8 = 42;

static mut Z:i32 = 777;

fn main() {
    println!("{}", MEANING_OF_LIFE);
    unsafe {
        println!("{}", Z);
    }
}
