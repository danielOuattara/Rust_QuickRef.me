fn calc_rect(x: u32, y: u32) -> u32 {
    x * y
}

fn main() {
    let x: u32 = 10;
    let y: u32 = 20;
    println!("calc_rect {x} and {y} returns: {:?}", calc_rect(x, y))
}
