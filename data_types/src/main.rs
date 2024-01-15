fn main() {
    // integer type

    // signed
    let i: i16 = -20;
    // unsigned
    let u: u16 = 20;

    // float type

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // numeric operations

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // boolean type

    let t = true;

    let f: bool = false; // with explicit type annotation

    // char type
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // accessing tuple
    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    // accessing each element
    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    // the array type
    let a = [1, 2, 3, 4, 5];

    // useful when we know the exact element of the array
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // specific the element and type
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // initial value with specific value
    // [3, 3, 3, 3, 3]
    let a = [3; 5];

    // accessing value
    let index0 = a[0];
    let index1 = a[1];
}
