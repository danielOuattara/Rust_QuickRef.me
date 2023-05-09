fn main() {
    /* Comparison Operators
        ------------------------
    e == f 	e is equal to f
    e != f 	e is NOT equal to f
    e < f 	e is less than f
    e > f 	e is greater f
    e <= f 	e is less than or equal to f
    e >= f 	e is greater or equal to f
    */
    let (e, f) = (1, 100);

    let greater = f > e; // => true
    let less = f < e; // => false
    let greater_equal = f >= e; // => true
    let less_equal = e <= f; // => true
    let equal_to = e == f; // => false
    let not_equal_to = e != f; // => true

    /* Arithmetic Operators
    -------------------------
    a + b 	a is added to b
    a - b 	b is subtracted from a
    a / b 	a is divided by b
    a % b 	Gets remainder of a by dividing with b
    a * b 	a is multiplied with b
    */
    let (a, b) = (4, 5);

    let sum: i32 = a + b; // => 9
    let subtractions: i32 = a - b; // => -1
    let multiplication: i32 = a * b; // => 20
    let division: i32 = a / b; // => 0
    let modulus: i32 = a % b; // => 4

    /* Bitwise Operators
    ----------------------
    g & h 	Binary AND
    g | h 	Binary OR
    g ^ h 	Binary XOR
    g ~ h 	Binary one's complement
    g << h 	Binary shift left
    g >> h 	Binary shift right
    */

    let (g, h) = (0x1, 0x2);

    let bitwise_and = g & h; // => 0
    let bitwise_or = g | h; // => 3
    let bitwise_xor = g ^ h; // => 3
    let right_shift = g >> 2; // => 0
    let left_shift = h << 4; // => 32

    /*  Logical Operators
    ----------------------
    c && d 	Both are true (AND)
    c || d 	Either is true (OR)
    !c 	c is false (NOT)
    */

    let (c, d) = (true, false);

    let and = c && d; // => false
    let or = c || d; // => true
    let not = !c; // => false

    /* Compound Assignment Operator
    -------------------------------- */
    let mut k = 9;
    let mut l = k;

    k += l; // Add a value and assign, then k=9
    k -= l; // Substrate a value and assign, then k=18
    k /= l; // Divide a value and assign, then k=9
    k *= l; // Multiply a value and assign, then k=81
    k |= l; //Bitwise OR and assign, then k=89
}
