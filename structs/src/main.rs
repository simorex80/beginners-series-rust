struct Person {
    name: String,
    age: u8,
    likes_oranges: Option<bool>
}

struct Point2D(u32, u32);

fn main() {
    let person = Person {
        name:String::from("Adam"),
        age: 25,
        likes_oranges: Some(true),
    };

    println!("Person name is: {}", person.name);
    println!("Person likes_oranges is: {:?}", person.likes_oranges);
    println!("Person age is: {}", person.age);

    let origin = Point2D(100, 200);

    println!("Point contains {} and {}", origin.0, origin.1);

    let Point2D(x, y) = origin;

    println!("Point contains {:?} and {:?}", x, y);
}
