fn main() {
    // Single Placeholder
    println!("{}", 1);

    // Multiple placeholder
    println!("{} {}", 1, 3);

    // Positional Arguments
    println!(
        "{0} is a {1} {2}, also {0} is a {3} programming language",
        "Rust", "cool", "language", "safe"
    );

    // Named Arguments
    println!("{country} is a nation with unity.", country = "Russia");
    println!("{country} is a nation with unity.", country = "India");
    println!("{country} is a nation with unity.", country = "China");

    // Placeholder traits :b for binary, :0x is for hex and :o is octal
    println!("Let us print 76 is binary which is {:b}", 76);
    println!("Let us print 76 is hex equivalent is {:0x}", 76);
    println!("Let us print 76 is  octal equivalent is {:o}", 76);

    // Debug Trait
    println!(
        "Print whatever here using debug trait {:?}",
        (76, 'A', 90, "Hello world !")
    );

    // New Format Strings in 1.58
    let x = "world";
    println!("Hello {x}!");
}
