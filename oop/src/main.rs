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

fn main() {

    let person = Person::new(String::from("Mr"), String::from("Nell"), 39);

    person.say_your_name();

    let employee = Employee::new(person, 1700);

    println!("Employee {:?}", employee);

}
