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

    let employee = Employee::new(person, 1700);

    println!("Employee {:?}", employee);

}
