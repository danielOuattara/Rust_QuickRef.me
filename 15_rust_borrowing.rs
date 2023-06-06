fn main() {
    let foo = 4;
    let borrowed_foo = &foo;
    println!("{borrowed_foo}");

    let mut bar = 3;
    let mutable_borrowed_bar = &mut bar;
    println!("{mutable_borrowed_bar}");

    // Here borrowed value borrows the value from value one using & operator.
}
