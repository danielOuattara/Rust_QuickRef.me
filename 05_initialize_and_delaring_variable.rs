fn main() {
    // Initializing and declaring a variable
    let some_variable = "This_is_a_variable";
    println!("{some_variable}");

    // Making a variable mutable
    let mut mutable_variable = "Mutable";
    println!("{mutable_variable}");

    mutable_variable = "new mutation";
    println!("{mutable_variable}");

    // Assigning multiple variables
    let (name, age) = ("ElementalX", 20);
    println!("name is {name}");
    println!("Age is {age}");

    // (Global) constant
    const SCREAMING_SNAKE_CASE: i64 = 9;
    println!("{SCREAMING_SNAKE_CASE}");
}
