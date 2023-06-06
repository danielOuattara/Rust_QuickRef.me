fn power_of_three(by_ref: &mut i32) {
    // de-referencing is important
    *by_ref = *by_ref * *by_ref * *by_ref;
    println!("{by_ref}");
}

fn main() {
    let mut by_ref = 3;
    power_of_three(&mut by_ref);
    println!("{by_ref}");
}
