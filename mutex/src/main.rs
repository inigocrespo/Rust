use std::sync::{Arc, Mutex};
use std::thread;

struct Person {
    name: Arc<String>,
    state: Arc<Mutex<String>>
}


impl Person {
    fn new(name: Arc<String>, state: Arc<Mutex<String>>) -> Person {
        Person{
            name: name,
            state: state
        }
    }

    fn greet(&self) {

        let mut state =  self.state.lock().unwrap();
        state.clear();
        state.push_str( "excited");

        println!("Hi my name is {}", self.name);
    }
}


fn mutex_demo() {
    let name = Arc::new("Jhon".to_string());
    let state = Arc::new(Mutex::new("Bored".to_string()));
    let person = Person::new(name.clone(), state.clone());

    let t = thread::spawn(move|| {
        person.greet();
    });

    {
        let st =  state.lock().unwrap();
        println!("Name = {} State {}", name, st);
    }

    t.join().unwrap()
}

fn main() {
    mutex_demo();
}
