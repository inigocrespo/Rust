use std::rc::Rc;

struct Person {
    name: Rc<String>
}


impl Person {
    fn new(name: Rc<String>) -> Person {
        Person{
            name: name
        }
    }

    fn greet(&self) {
        println!("Hi my name is {}", self.name);
    }
}

fn rc_demo() {
    let name = Rc::new("Jhon".to_string());
    println!("Name has {} strong pointers", Rc::strong_count(&name));
    {
        let person = Person::new(name.clone());
        println!("Name has {} strong pointers", Rc::strong_count(&name));
        person.greet();

    }

    println!("Name has {} strong pointers", Rc::strong_count(&name));

    println!("Name is {}", name);
}

fn main() {
    rc_demo();

}
