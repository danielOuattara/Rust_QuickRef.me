fn main() {
    {
        // The scope limited to this braces
        let a_number = 1;
    }
    println!("{a_number}");

    // This will produce error as the scope of the variable a_number ends at the braces
}
