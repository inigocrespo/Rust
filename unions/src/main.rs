union IntOrFloat {
    i: i32,
    f: f32,
}

fn process_value(iof:IntOrFloat) {
    unsafe {
        match iof {
            IntOrFloat{ i:  42} => {
                println!("meaning of life value")
            }
            IntOrFloat{f} =>  {
                println!("value is equal = {}", f)
            }
        }
    }

}

fn main() {
    let mut iof = IntOrFloat{i:123};
    iof.i = 321;

    let value = unsafe {iof.i};
    println!("iof.i = {}", value);

    process_value(IntOrFloat{i: 5});
}
