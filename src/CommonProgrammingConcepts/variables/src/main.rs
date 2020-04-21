fn main() {
    
    
    //
    // immutable vs mutable 
    //

    // let x = 5;
    let mut x = 5;
    println!("The value of x is:{}", x);

    x = 6;
    println!("The value of x is:{}", x);


    //
    // constants
    //
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is {}", MAX_POINTS);


    //
    // shadow
    //
    let x = x+1;
    println!("The value of x is:{}", x);

    let x = x*2;
    println!("The value of x is {}", x);

    //
    // mismatched types of mut
    //
    
    // let mut spaces = "  ";
    let spaces = "  ";
    println!("The value of spaces is {}", spaces);

    let spaces = spaces.len();
    println!("The value of spaces is {}", spaces);
}