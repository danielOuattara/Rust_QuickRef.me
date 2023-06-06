fn print_arrays(mut array: [i32; 5]) {
    println!("initially : {array:?}");
    array[0] = 89;
    array[1] = 90;
    array[2] = 91;
    array[3] = 92;
    array[4] = 93;
    println!("Then : {array:?}");
}

fn main() {
    let array: [i32; 5] = [1, 2, 3, 4, 6];
    print_arrays(array);
    // println!("The elements: {array:?}");
}
