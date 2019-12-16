fn if_statement() {
    let temp = 35;
    if temp > 35 {
        println!("Really hot outside")
    } else {
        println!("Really cold")
    }

    let day = if temp > 20 {"sunny"} else {"cloudy"};
    println!("day {}", day);

    println!("day {}", if temp > 20 {"sunny"} else if temp < 10 {"cold"} else {"cloudy"});


    println!("day {}",
             if temp > 20 {
                 if temp > 30 {"very hot"} else {"sunny"}
             } else {
                 if temp < 0 {"very cold"} else {"cold"}
             })

}

fn main() {
    if_statement()
}
