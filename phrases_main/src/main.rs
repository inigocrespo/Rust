//extern crate phrases;

use phrases::phrases::english;
use phrases::phrases::spanish;

fn main() {
    println!("{}", english::say_hello());
    println!("{}", spanish::say_hello())
}
