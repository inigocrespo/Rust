trait Animal {
    fn create(name: &'static str) -> Self where Self:Sized;

    fn name(&self) -> &'static str;

    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str,
}

impl Animal for Human {
    fn create(name: &'static str) -> Human {
        Human { name }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says hello", self.name())
    }
}

struct Cat {
    name: &'static str,
}

impl Animal for Cat {
    fn create(name: &'static str) -> Cat {
       Cat { name }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says Meaw", self.name())
    }
}

enum Creature {
    Human(Human),
    Cat(Cat),
}

fn main() {
    let mut creatures = Vec::new();
    //creatures.push(Human{name: "Jone"});
    //creatures.push(Cat{name: "Catwoman"});
    creatures.push(Creature::Human(Human { name: "Jone" }));
    creatures.push(Creature::Cat(Cat { name: "Catwoman" }));

    for c in creatures {
        match c {
            Creature::Human(h) => h.talk(),
            Creature::Cat(c) => c.talk(),
        }
    }

    let mut animals: Vec<Box<dyn Animal>> = Vec::new();
    animals.push(Box::new(Human { name: "Jone" }));
    animals.push(Box::new(Cat { name: "Catwoman" }));

    for a in animals.iter() {
        a.talk();
    }
}
