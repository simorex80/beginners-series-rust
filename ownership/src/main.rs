fn main() {
    let mut say = String::from("Ca");

    say.push('t');

    let say2 = say;

    // This line will return an error
    // println!("{}", say); 

    print(say2);

}

fn print(out: String) {
    println!("{}", out);
    // HERE say2 is DESTROYED AND FREEDED
}