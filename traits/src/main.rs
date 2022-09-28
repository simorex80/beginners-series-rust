pub struct Person {
    name: String
}

pub struct Cat {
    name: String
}

pub struct Rabbit {
    name: String
}

pub trait Eat {
    fn eat_dinner(&self) {
        println!("I eat from a dish")
    }
}

impl Eat for Person {
    fn eat_dinner(&self) {
        println!("I eat from a plate")
    }
}

impl Eat for Cat {
    fn eat_dinner(&self)  {
        println!("I eat from a cat bowl")
    }
}

impl Eat for Rabbit {}

fn main() {

    let person = Person {
        name: String::from("Nell")
    };

    let cat = Cat {
        name: String::from("Marvin")
    };

    let rabbit = Rabbit {
        name: String::from("Leia")
    };

    do_eat(&person);

    do_eat(&cat);

    do_eat(&rabbit);

}

fn do_eat(animal_that_eat: &dyn Eat) {
    animal_that_eat.eat_dinner();
}