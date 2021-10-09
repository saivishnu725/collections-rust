fn main() {
    // First declare the vector and then add values whenever we want
    let mut v: Vec<i32> = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    v.push(40);
    v.push(50);
    v.pop();
    println!("v[1] = {}", v[1]);
    println!("Length of vector v = {}", v.len());

    // Initialise the values to the vector while declaring, it can be push() and pop() anytime
    let v2 = vec![10, 20, 30];
    println!("v2[1] = {}", v2[1]);

    //Vectors have a problem!!
    // They can go out of index bound while runtime.
    // So to avoid it we can use get() which returns a Option enum
    // Which can be used with match command to avoid crashing.

    //Within index bound
    match v.get(1) {
        Some(val) => println!("Value is {}", val),
        None => println!("Index out of bound"),
    }
    // Outside index bound
    match v.get(7) {
        Some(val) => println!("Value is {}", val),
        None => println!("Index out of bound"),
    }

    // Iterating through vector using for
    // Immutable
    for i in &v {
        println!("{}", i);
    }
    println!("");
    // Mutable
    for i in &mut v {
        *i += 10;
        println!("{}", i);
    }

    // Vectors can hold values of a single data type.
    // So in order to use multiple types, create a enum and use it as the vector type
    enum ValueLol {
        Int(i32),
        Float(f32),
        Text(String),
    }
    let v3 = vec![
        ValueLol::Int(32),
        ValueLol::Float(3.2),
        ValueLol::Text(String::from("Sai Vishnu")),
    ];
    match &v3[1] {
        ValueLol::Int(i) => println!("Integer = {}", i),
        ValueLol::Float(f) => println!("Float = {}", f),
        ValueLol::Text(s) => println!("String = {}", s),
    }
}
