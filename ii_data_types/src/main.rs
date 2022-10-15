fn main() {
    let a = 0; //By default i32

    //Unsigned types
    let b: u8 = u8::MAX; // 1 Byte
    let c: u16 = u16::MAX; // 2 Bytes
    let d: u32 = u32::MAX; // 4 Bytes
    let e: u64 = u64::MAX; // 8 bytes
    let f: u128 = u128::MAX; // 16 bytes

    //Signed types
    let b2: i8 = i8::MIN;
    let c2: i16 = i16::MIN;
    let d2: i32 = i32::MIN;
    let e2: i64 = i64::MIN;
    let f2: i128 = i128::MIN;

    //Char and String types
    let s = "Hello world";
    let ch = 'c';
    let emoji = "✋"; //Treated as string

    //Boolean type
    let v_true:bool = true;
    let v_false:bool = false;

    //Floating point types, simple and double
    let fl:f32 = 1.0; //4 bytes
    let dbl:f64 = 1.0; //8 bytes

    //Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
    println!("The value of x is: {}", x);

    //Arrays
    let a1 = [1, 2, 3, 4, 5];
    let a2 = [3; 5]; // [3, 3, 3, 3, 3]
    let a3 = [3, 4, 5, 6, 7];
    let first = a3[0];
    let second = a3[1];
    let third = a3[2];
    println!("The value of first is: {}", first);
    println!("The value of second is: {}", second);
    println!("The value of third is: {}", third);
    // {} is called interpolation and it is used to print variables inside a string or a format string. The difference between {} and {:?} is that {:?} prints the value of the variable and {} prints the type of the variable.
    // {:?} is a pretty printer for debuging purposes. The difference between {:?} and {:#?} is that {:?} prints the values of the variables, while {:#?} prints the type of the variables.
    println!("The value of a3 is: {:?}", a3); //Prints the array in a nice way
}