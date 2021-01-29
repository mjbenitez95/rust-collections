#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    access_vector();
    println!("");
    mutate_vector();
    println!("");
    iterate_and_mutate_vector();
    println!("");
    introduce_enum_vector();
    println!("");

    create_string();
    println!("");
    properly_encoded_strings();
    println!("");
    update_string();
    println!("");
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

fn create_string() {
    let mut my_first_string = String::new();

    let data = "variable_to_string";
    let variable_to_string = data.to_string();

    let literal_to_string = "literal_to_string".to_string();
    let string_from_literal = String::from("string_from_literal");

    println!(
        "We have {} and {} and {}!",
        variable_to_string, literal_to_string, string_from_literal
    );
}

fn properly_encoded_strings() {
    // since strings are UTF-8 encoded, we can include any properly encoded data in them
    let strings: Vec<String> = vec![
        String::from("السلام عليكم"),
        String::from("Dobrý den"),
        String::from("Hello"),
        String::from("שָׁלוֹם"),
        String::from("नमस्ते"),
        String::from("こんにちは"),
        String::from("안녕하세요"),
        String::from("你好"),
        String::from("Olá"),
        String::from("Здравствуйте"),
        String::from("Hola"),
    ];

    for some_string in strings {
        println!("Our strings vector has the string \"{}\"!", some_string);
    }
}

fn update_string() {
    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s2 = String::from("foo");
    let s2_suffix = "bar";
    s2.push_str(s2_suffix);

    let mut s3 = String::from("lo");
    s3.push('l');
    println!("Now our strings are {} and {} and {}!", s, s2, s3);
}
