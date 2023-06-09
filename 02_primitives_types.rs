fn main() {
    // bool, (true / false)
    let _is_morning = true;
    let _is_user_logged_in = false;

    //--------------------------------------------------------

    // char, single character
    let _single_letter = 'A';
    let _single_number = '2';

    //--------------------------------------------------------

    // f32, 32-bits floats
    let _pi_1: f32 = 3.1415912345678901234567890; // 3.1415913
    println!("{}", _pi_1);

    // f64, 64-bits floats
    let _pi_2: f64 = 3.1415912345678901234567890; // 3.14159123456789

    println!("{}", _pi_2);

    //--------------------------------------------------------

    // i128,  signed 128 bits integers
    // between:  -340282366920938463463374607431768211455 & 340282366920938463463374607431768211455
    let _integer_128: i128 = -123456789012345678901234567890123456789;
    println!("_integer_128 = {} ", _integer_128);

    // i64,  signed 64 bits integers
    let _integer_64: i64 = -1234567890123456789;
    println!("_integer_64 = {}", _integer_64);

    // i32,  signed 32 bits integers: between -2147483648 & 2147483647
    let _integer_32: i32 = -2147483647;
    println!("_integer_32 = {}", _integer_32);

    // i16,  signed 16 bits integers -32768 & 32767
    let _integer_16: i16 = -32768;
    println!("_integer_16 = {}", _integer_16);

    // i8 ,  signed 8 bits integers between 128 & 127
    let _integer_8: i8 = -127;
    println!("_integer_8 = {}", _integer_8);

    //----------------------------------------------------

    // u128, unsigned 128 bits integers
    // between:  0 to 340282366920938463463374607431768211455
    let u_integer_128: u128 = 123456789012345678901234567890123456789;
    println!("u_integer_128 = {} ", u_integer_128);

    // u64, unsigned 64 bits integers
    let _integer_64: u64 = 1234567890123456789;
    println!("_integer_64 = {}", _integer_64);

    // u32, unsigned 32 bits integers: 0 to  2147483647
    let u_integer_32: u32 = 2147483647;
    println!("u_integer_32 = {}", u_integer_32);

    // u16, unsigned 16 bits integers from 0 to 32767
    let u_integer_16: u16 = 32768;
    println!("u_integer_16 = {}", u_integer_16);

    // u8, unsigned 8 bits integers between 0 and 127
    let u_integer_8: u8 = 127;
    println!("u_integer_8 = {}", u_integer_8);

    //--------------------------------------------------------

    // isize                 pointer-sized signed integer

    // usize                 pointer-sized unsigned integers
}
