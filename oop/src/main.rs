trait Registry {
    fn get_title(&self) -> String;
    fn get_name(&self) -> String;
    fn get_age(&self) -> i32;
}

#[derive(Debug)]
struct Person {
    title: String,
    name: String,
    age: i32,
}

impl Person {
    fn new(title: String, name: String, age: i32) -> Self {
        Self { title, name, age }
    }
    // Custom method
    fn say_your_name(&self) {
        println!("My name is {}", self.name);
    }
}

impl Registry for Person {

    fn get_title(&self) -> String {
        self.title.clone()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_age(&self) -> i32 {
        self.age
    }

}

#[derive(Debug)]
struct Employee {
    person: Person,
    salary: i32,
}

impl Employee {
    fn new(person: Person, salary: i32) -> Self {
        Self { person, salary }
    }
}

impl Registry for Employee {

    fn get_title(&self) -> String {
        self.person.get_title()
    }

    fn get_name(&self) -> String {
        self.person.get_name()
    }

    fn get_age(&self) -> i32 {
        self.person.get_age()
    }

}

fn main() {

    let person = Person::new(String::from("Mr"), String::from("Nell"), 39);
    println!("Person {:?}", person);
    print_registry(&person);
    person.say_your_name();

    let employee = Employee::new(person, 1700);
    println!("Employee {:?}", employee);
    print_registry(&employee);

}

fn print_registry(registry: &dyn Registry) {

    // composition over inheritance example

    println!("Registry.get_title {}", registry.get_title());
    println!("Registry.get_name {}", registry.get_name());
    println!("Registry.get_age {}", registry.get_age());

}
