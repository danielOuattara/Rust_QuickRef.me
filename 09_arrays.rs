fn main() {
    // Array

    let array: [i64; 6] = [92, 97, 98, 99, 98, 94];
    println!("array = {:?}", array); // line display
    println!("array = {:#?}", array); // vertical display

    for item in array {
        println!("{:?}", item);
    }

    // multi-dimensional Array

    let array_2d: [[i64; 6]; 2] = [[1, 2, 3, 4, 5, 6], [6, 5, 4, 3, 2, 1]];

    println!("array_2d = {:?}", array_2d); // line display
    println!("array_2d = {:#?}", array_2d); // vertical display

    let array_3d: [[i64; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    println!("array_3d = {:?}", array_3d); // line display

    // mutable array: Use the mut keyword to make it mutable.

    let mut array_2: [i32; 3] = [2, 6, 10];

    array_2[1] = 4;
    array_2[2] = 6;

    println!("array = {:?}", array);

    // slices: Lower range is inclusive and upper range is exclusive

    let my_array: [i64; 4] = [1, 2, 3, 4];
    println!("array = {:?}", my_array);

    let copy_slice: &[i64] = &my_array[0..3];
    println!("array = {:?}", copy_slice);

    println!("The elements of the slices are : {copy_slice:?}");

    // vectors

    let some_vector = vec![1, 2, 3, 4, 5];
    println!("some_vector = {:?}", some_vector);

    let copied_vector = &some_vector[0..3];
    println!("copied_vector : {copied_vector:?}");

    // tuples

    let tuple = (1, 'A', "Cool", 78, true);
    println!(" tuple = {:?}", tuple);
}
