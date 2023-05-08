fn main() {
    /* String literal
    -------------------- */
    let c_sheet = "cheat sheet";
    // share cheat sheet for developers
    println!("Share {c_sheet} for developers");

    let cs: &str = "cheat sheet";
    // share cheat sheet for developers
    println!("Share {cs} for developers");

    /* String Object
    -----------------*/

    // creating empty string object
    let _my_string = String::new;

    let a_string = 45;
    println!("{a_string}");

    //converting to a string object
    let s_string = a_string.to_string();
    println!("{s_string}");

    // creating an initialized string object
    let lang = String::from("Rust");
    println!("First language is {lang}");

    // .capacity()
    let words = String::from("Random String");
    println!("{:?}", words.capacity());

    // .contains()
    let name = String::from("ElementalX");
    println!("{}", name.contains("Element")); // -> true
    println!("{}", !name.contains("Element")); // -> false

    // Pushing a single character
    let mut half_text = String::from("Hal");
    half_text.push('f');
    println!("{}", half_text); // => Half

    // Pushing an entire String
    let mut hi_there = String::from("Hey there...");
    hi_there.push_str(" How are you doing ?");

    println!("{hi_there}")
}
