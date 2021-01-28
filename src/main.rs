#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];

    println!("The third element of our vector is {}!", third);

    match v.get(2) {
        Some(third) => println!("The third element of our vector is {}!", third),
        None => println!("Our vector has no third element."),
    }

    // let does_not_exist = &v[100]; // this line attempts to access out of bounds, causing a runtime crash
    let _does_not_exist = v.get(100); // whereas this line just returns an Option<T> of None

    let first = &v[0];
    println!("The first element of our vector is {}!", first);
    v.push(6);
    // if we tried to access first after this, we'd have an issue with a mutable reference, and won't compile

    let v2 = vec![100, 32, 57];
    for i in &v2 {
        println!("Vector 2 has element {}!", i);
    }

    let mut v2 = vec![100, 32, 57];
    for i in &mut v2 {
        *i += 50; // note the usage of the dereference operator (*)
        println!("New vector 2 has element {}!", i);
    }

    // we can use enums to put objects of different known types into a vector
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in &row {
        println!("Row vector has element {:?}!", i);
    }
}
