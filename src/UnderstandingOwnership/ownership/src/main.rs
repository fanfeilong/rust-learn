fn main() {

    // 1. Each value in Rust has a variable that's called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    //
    // Variable Scope
    //
    {
                         // s is not valid here, it's not yet declared
        let s = "hello"; // s is valid from this point forward
    }
                         // this scope is now over, and s is no longer valid
    // 1. When s comes into scope, it is valid
    // 2. It remains valid until it goes out of scope


    //
    // String
    //
    {
        let s = String::from("hello");

        let mut s = String::from("hello");
        println!("The value of String is:{}", s);

        s.push_str(", world");
        println!("The value of String is:{}", s);
    }

    //
    // Move
    //
    {
        let x = 5;
        let y = x;

        let s1 = String::from("Hello");
        let s2 = s1;
        // println!("The value of s1 after assign to s2: {}", s1);
    }
    

    //
    // Clone
    //
    {
        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("The value of s1 after clone to s2: {}", s1);
        println!("The value of s2 after clone from s1: {}", s2);
    }

    //
    // Ownership and Functions
    //
    {
        let s = String::from("hello");
        takes_ownership(s);
        // println!("The value of s after take owner to function:{}", s);

        let x = 5;
        makes_copy(x);
        println!("The value of x after copy to function argument:{}", x);
    }

    //
    // Return
    //
    {
        let s1 = gives_ownership();
        println!("The value of s from func return:{}", s1);

        let s2 = String::from("hello");

        let s3 = takes_and_gives_back(s2);
        // println!("The value of s2 takes_and_gives_back:{}", s2);
        println!("The value of s3 takes_and_gives_back:{}", s3);

        let (s4, len) = calculate_length(s3);
        println!("The value of calculate_length:({}, {})", s4, len);
    }
}

fn takes_ownership(some_string: String) {
    println!("takes_ownership {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("makes_copy:{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    
    (s, length)
}