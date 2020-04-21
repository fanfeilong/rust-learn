fn main() {
    //
    // data types
    //
    // let guess = "42".parse().expect("Not a number!");
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is:{}", guess);

    //
    // scalar type
    //

    // let n: i8 = 128;
    let n: i8 = 127;
    println!("The max value of i8:{}", n);

    // let n: u8 = 256;
    let n: u8 = 255;
    println!("The max value of u8:{}", n);

    // let n: i16 = 32768;
    let n: i16 = 32767;
    println!("The max value of i16:{}", n);

    // let n: u16 = 65536;
    let n: u16 = 65535;
    println!("The max value of u16:{}", n);

    // let n: i32 = 2147483648;
    let n: i32 = 2147483647;
    println!("The max value of i32:{}", n);

    // let n: u32 = 4294967296;
    let n: u32 = 4294967295;
    println!("The max value of u32:{}", n);

    // let n: i64 = 9223372036854775808;
    let n: i64 = 9223372036854775807;
    println!("The max value of i64:{}", n);

    // let n: u64 = 18446744073709551616;
    let n: u64 = 18446744073709551615;
    println!("The max value of u64:{}", n);


    let n: i128 = 18446744073709551615;
    println!("The value of n:i128:{}", n);

    let n: u128 = 18446744073709551615;
    println!("The value of n:u128:{}", n);

    println!("The value of decimal:{}", 98_222);
    println!("The value of Hex:{}", 0xfff);
    println!("The value of Octal:{}", 0o77);
    println!("The value of Binary:{}", 0b1111_0000);
    println!("The value of Byte:{}", b'A');

    //
    // float
    //
    let d = 2.0;  
    println!("The value of f64:{}", d);

    let d: f32 = 3.0;
    println!("The value of f32:{}", d);

    //
    // boolean
    //
    let t = true;
    println!("The value of bool:{}", t);

    let f: bool = false; // with explicit type annotation
    println!("The value of bool:{}", f);

    //
    // character type
    //
    let c = 'z';
    println!("The value of c:{}", c);

    let z = 'ℤ';
    println!("The value of z:{}", z);

    let cat = '🐱';
    println!("The value of cat:{}", cat);


    //
    // Compound Types
    //

    // tuple
    let tup: (i32,f64,u8) = (500,6.4, 1);
    let (x,y,z) = tup;
    // println!("The value of tuple:{}", tup);
    println!("The value of x in tuple:{}", x);
    println!("The value of y in tuple:{}", y);
    println!("The value of z in tuple:{}", z);


    println!("The value of tup.0 in tuple:{}", tup.0);
    println!("The value of tup.1 in tuple:{}", tup.1);
    println!("The value of tup.2 in tuple:{}", tup.2);


    // Array
    let a = [1,2,3,4,5,6];
    println!("The value of the first element of array is:{}", a[0]);

    let a: [i32; 5] = [1,2,3,4,5];
    println!("The value of  the first element of array is:{}", a[0]);

    let a = [3;5];
    println!("The value of  the last element of array is:{}", a[a.len()-1]);


}