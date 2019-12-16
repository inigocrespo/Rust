struct Person {
    name: String
}

struct Company<'z> {
    name: String,
    ceo: &'z Person
}

impl Person {
    fn get_ref_name(&self) -> &String {
        &self.name
    }
}

fn main() {
    //let boss = Person{ name: String::from("Elon Musk")};
    //let tesla = Company{ name: String::from("Tesla"), ceo: &boss};

    let mut z: &String;
    {
        let p = Person{ name: String::from("John")};
        z = p.get_ref_name()
    }
}
