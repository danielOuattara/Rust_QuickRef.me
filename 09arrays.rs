fn main() {
    // Array
    let array: [i64; 6] = [92, 97, 98, 99, 98, 94];
    println!("array = {:?}", array); // line display
    println!("array = {:#?}", array); // vertical display

    // multi-dimensional Array
    let array_2d: [[i64; 6]; 2] = [[1, 2, 3, 4, 5, 6], [6, 5, 4, 3, 2, 1]];

    println!("array_2d = {:?}", array_2d); // line display
    println!("array_2d = {:#?}", array_2d); // vertical display

    // mutable array
    // Use the mut keyword to make it mutable.
    let mut array_2: [i32; 3] = [2, 6, 10];

    array_2[1] = 4;
    array_2[2] = 6;

    println!("array = {:?}", array); // vertical display

    // slices
    // Lower range is inclusive and upper range is exclusive

    let mut array: [i64; 4] = [1, 2, 3, 4];
    let mut slices: &[i64] = &array[0..3];

    println!("The elements of the slices are : {slices:?}");

    let some_vector = vec![1, 2, 3, 4, 5];
    println!("some_vector = {:?}", some_vector); // vertical display

    //tuples
    let tuple = (1, 'A', "Cool", 78, true);
    println!(" tuple = {:?}", tuple);
}
