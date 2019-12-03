fn main() {

    let a = 2;
    let a_cube = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cube);

    let b = 2.5;
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{}^pi is {}", b, b_to_pi);

    let c = 1 | 2;
    println!("c = {}", c);

    let two_to_ten = 1 << 10;
    println!("two_to_ten= {}", two_to_ten);

    let pi_less_4 = std::f64::consts::PI < 4.0;
    println!("pi_less_4 = {}", pi_less_4);

    let x = 5;
    let x_is_5 = x == 5;
    println!("x is 5 = {}", x_is_5)



}
