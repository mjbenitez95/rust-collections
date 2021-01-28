fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];

    println!("The third element of our vector is {}!", third);

    match v.get(2) {
        Some(third) => println!("The third element of our vector is {}!", third),
        None => println!("Our vector has no third element."),
    }

    // let does_not_exist = &v[100]; // this line attempts to access out of bounds, causing a runtime crash
    let does_not_exist = v.get(100); // whereas this line just returns an Option<T> of None
}
