struct Creature {
    name: String
}

impl Creature {
    fn new(name: &str) -> Creature {
        println!("{} enters the game ", name);
        Creature{name: name.into()}
    }
}

impl Drop for Creature {
    fn drop(&mut self) {
        println!("{} is dead", self.name);
    }
}

fn drops() {
    let goblin = Creature::new("Jeff");
    println!("game proceeds");
    drop(goblin);
}

fn main() {
    drops();
}
