use std::collections::HashMap;

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
    println!("");

    create_string();
    properly_encoded_strings();
    update_string();
    concatenate_string();
    index_strings();
    slice_strings();
    iterate_over_strings();
    println!("");

    create_hashmap();
    hashmap_ownership();
    hashmap_access();
    hashmap_update();
    hashmap_update_based_on_current_value();
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
    let _my_first_string = String::new();

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
    println!(
        "Now our strings are \"{}\" and \"{}\" and \"{}\"!",
        s, s2, s3
    );
}

fn concatenate_string() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note that this line will move s1 into s3, and so s1 can no longer be used

    println!("Our new string is \"{}\"!", s3);

    let tic = String::from("tic");
    let tic_2 = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");

    let tic_tac_toe = tic + "-" + &tac + "-" + &toe;
    let format_string = format!("{}-{}-{}", tic_2, tac, toe);
    println!("Our tic_tac_toe string is \"{}\"!", tic_tac_toe);
    println!("And with the format operation, it's \"{}\"!", format_string);
}

fn index_strings() {
    let s1 = String::from("Hello!");
    println!("An unindexed string: \"{}\"", s1);
    /*
        the following code is not allowed because strings in rust are
        implemented as a wrapper over a Vec<u8>. But if the first character
        takes two bytes, we would only get one byte, and so an invalid character,
        if we took index[0]. So rust prevents us from compiling to avoid that.

        // let h = s1[0];
    */

    /*
        as an example, let's look at the Hindi word "नमस्ते" in the Devanagari script.
        as a Vec<u8>, it is an array of 18 bytes:
        [
            224, 164, 168, 224, 164, 174,
            224, 164, 184, 224, 165, 141,
            224, 164, 164, 224, 165, 135
        ]
        which is 18 bytes. These can be interpreted as Unicode scalar values,
        which would result in ['न', 'म', 'स', '्', 'त', 'े'], 6 characters where the
        4th and 6th character are not even valid on their own. Since the interpretation
        can be ambiguous, Rust avoids the situation and does not allow string indexing.
    */
}

fn slice_strings() {
    let hello = "Здравствуйте";
    let s = &hello[0..4];

    println!(
        "A slice with the first four bytes of \"{}\" yields \"{}\"!",
        hello, s
    );
    // note that &hello[0..1] would cause a Rust panc, since byte index 1 is not a valid char.
}

fn iterate_over_strings() {
    for character in "नमस्ते".chars() {
        println!("A character in \"नमस्ते\" is {}!", character);
        // again, this prints the 6 characters, 2 of which are invalid
    }

    for byte in "नमस्ते".bytes() {
        println!("A byte in \"नमस्ते\" is {}!", byte);
        // this prints the 18 byte array above
    }
}

fn create_hashmap() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("Our scores HashMap is \"{:?}\".", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores_from_vectors: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
    /*
        the zip() method creates a vector of tuples from two vectors,
        and the collect() method gathers data into collection types,
        including HashMap.
    */

    println!(
        "Meanwhile, our scores_from_vectors HashMap is \"{:?}\".",
        scores_from_vectors
    );
}

fn hashmap_ownership() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    println!("Map is :\"{:?}\".", map);

    /*
        the following line is not valid because map.insert takes ownership
        of its arguments, meaning field_name and field_value are no longer valid
    */
    // println!("Map contains key {} and value {}", field_name, field_value);
}

fn hashmap_access() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name); // note that this returns an Option<T>

    match score {
        Some(num) => println!("Team {} has a score of {:?} points!", team_name, num),
        None => println!("Team {} does not seem to have a score!", team_name),
    }

    for (key, value) in &scores {
        // note that this iteration is in an arbitrary order
        println!("Iterating: team {} has a score of {:?} points!", key, value);
    }
}

fn hashmap_update() {
    let mut scores = HashMap::new();

    // overwrite previous value
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    for (key, value) in &scores {
        println!(
            "After the overwrite, team {} has a score of {:?} points!",
            key, value
        );
    }

    // pattern to only insert if a key has no value
    scores.entry(String::from("Yellow")).or_insert(50); // no current value, so 50 is entered
    scores.entry(String::from("Blue")).or_insert(50); // nothing happens

    // or_insert returns a mutable reference for the value for the key, or
    // it'll insert the value given as an argument.

    for (key, value) in &scores {
        println!(
            "In the end, team {} has a score of {:?} points!",
            key, value
        );
    }
}

fn hashmap_update_based_on_current_value() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
        // since or_insert returns a mutable reference that is stored in count,
        // dereferencing and incrementing count increases the value inside the hashmap.
    }

    println!("Our final map is {:?}!", map);
}
