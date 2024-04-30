fn main() {
    println!("Hello, world!");


    let dog = Dog { name: "Rex".to_string() };
    let cat = Cat { name: "Whiskers".to_string() };

    animal_make_sound(&dog);
    animal_make_sound(&cat);
}

fn animal_make_sound(animal: &dyn Animal) {
    animal.make_sound();
}

trait Animal {
    fn make_sound(&self);
}


struct Dog {
    name: String,
}

impl Animal for Dog {

    fn make_sound(&self) {
        println!("{} says woof", self.name);
    }

}

struct Cat {
    name: String,
}

impl Animal for Cat {
    fn make_sound(&self) {
        println!("{} says meow", self.name);
    }
}