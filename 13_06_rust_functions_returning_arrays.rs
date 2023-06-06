fn multiply(mut arr: [i32; 5]) -> [i32; 5] {
    arr[2] = 90;
    for i in 0..5 {
        arr[i] = arr[i] * arr[2];
    }

    return arr;
}

fn main() {
    let arr: [i32; 5] = [2, 4, 6, 8, 10];
    multiply(arr);
    println!("The array is : {:?}", multiply(arr));
}
