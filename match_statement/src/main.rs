
fn math_statement() {
    let country_code = 44;
    let country = match country_code {
        44 => "UK",
        1...999 => "Known",
        _ => "Unkown"
    };

    println!("country = {}", country);

}

fn main() {
    math_statement()
}
