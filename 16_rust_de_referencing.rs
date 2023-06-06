fn main() {
    let mut borrow = 10;
    let deref = &mut borrow;
    println!("{}", *deref);
}
