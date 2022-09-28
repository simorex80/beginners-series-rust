use std::{fs::File, io::Error};

fn main() {
    //panic!("Farewell");
    //
    //let v = vec![0,1,2,3];
    //println!("{}", v[6]);

    // let fruits = vec!["banana", "apple", "coconut"];

    // let first = fruits.get(0);
    // println!("{:?}", first);

    // let third = fruits.get(2);
    // println!("{:?}", third);

    // let non_existent = fruits.get(99);
    // println!("{:?}", non_existent);

    // let f = File::open("hello.txt");
    
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Can't open the file {:?}", error),
    // };
    
    //let f = File::open("hello.txt").unwrap();
    
    // let f = File::open("hello.txt").expect("Failed to open hello.txt");

    let file_found = match open("hello.txt".to_string()) {
        Ok(file) => Some(file),
        Err(err) => {
            println!("Error opening file {}", err);
            None
        },
    };
}

fn open(path: String) -> Result<File, Error> {
    let f = File::open(path)?;
    Ok(f)
}