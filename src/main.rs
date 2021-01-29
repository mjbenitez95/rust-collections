#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    access_vector();
    mutate_vector();
    iterate_and_mutate_vector();
    introduce_enum_vector();
}

fn access_vector() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];

    println!("The third element of our vector is {}!", third);

    match v.get(2) {
        Some(third) => println!("The third element of our vector is {}!", third),
        None => println!("Our vector has no third element."),
    }
}

fn mutate_vector() {
    let mut v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[100]; // this line attempts to access out of bounds, causing a runtime crash
    let _does_not_exist = v.get(100); // whereas this line just returns an Option<T> of None

    let first = &v[0]; // <- this is an immutable borrow
    println!("The first element of our vector is {}!", first);
    v.push(6); // <- this is a mutable borrow, so we can't have an immutable one after
}

fn iterate_and_mutate_vector() {
    let v2 = vec![100, 32, 57];
    for i in &v2 {
        println!("Vector 2 has element {}!", i);
    }

    let mut v2 = vec![100, 32, 57];
    for i in &mut v2 {
        *i += 50; // note the usage of the dereference operator (*)
        println!("New vector 2 has element {}!", i);
    }
}

fn introduce_enum_vector() {
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
