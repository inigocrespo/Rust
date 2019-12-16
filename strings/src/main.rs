fn strings() {
    let s:&'static str = "hello there!";

    for c in s.chars() {
        println!("{}", c);
    }

    if let Some(first_char) = s.chars().nth(0) {
        println!("fist char = {}", first_char)
    }

    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }

    println!("{}",letters);

    let u:&str = &letters;

    let z = letters + "abc";

    let mut abc = String::from("hellow_world");
    let mut cba = "helloWorld".to_string();
    cba.remove(0);
    cba.push_str("!!!");
    println!("cba = {}", cba);

}

fn main() {
    strings();
}
