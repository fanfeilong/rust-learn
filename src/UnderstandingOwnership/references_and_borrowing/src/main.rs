fn main() {

    // references
    {
        let s1 = String::from("hello");

        let len = calculate_length(&s1);

        println!("The length of '{}' is {}", s1, len);

        let (s2, len) = calculate_length_2(&s1);
        println!("The length of '{}' is {}, return:{}", s1, len, s2);
    }

    // borrowing
    {
        // let s = String::from("hello");
        // change(&s);

        let mut s = String::from("hello");
        change(&mut s);
    }

    // mut reference only once
    // 
    // data race:
    // ============
    // * Two or more pointer access the same data at the same time
    // * At least one of the pointer is being used to write the data
    // * There is no mechanism being used to synchronize access to the data
    //
    // data race cause undefined behavior and can be difficult to 
    // diagnose and fix when you're trying to track them down at runtime
    // Rust prevents this problem form happening because it won't 
    // even compile code with data races!
    //
    {
        let mut s = String::from("hello");

        // let r1 = &mut s;
        // let r2 = &mut s;
        // println!("{}, {}", r1, r2);

        let r1 = &mut s;
        println!("{}", r1);
    }

    // use curly brackets to create new scope,
    // allowing for multiple mutable references, 
    // just not simultaneous ones:
    {
        let mut s = String::from("hello");

        {
            let r1 = &mut s;
        } // r1 goes out of scope here, so we can make a new reference with no problems.

        let r2 = &mut s;
    }

    // however, we can have immutable references
    {
        let mut s = String::from("hello");

        let r1 = &s;
        let r2 = &s;
        println!("{}, {}", r1, r2);
    }

    // but, we can not have multi mut references + immutable references
    {
        let mut s = String::from("hello");

        let r1 = &s;
        let r2 = &s;

        // users of an immutable reference don't 
        // expect the values to suddenly change out 
        // from under them!

        // let r3 = &mut s;
        // println!("{}, {}", r1, r2, r3);
    }

    // a reference's scope starts from where it is introduced
    // and continues through the last time that reference is used
    {
        let mut s = String::from("hello");

        let r1 = &s;
        let r2 = &s;

        println!("{}, {}", r1, r2);
        let r3 = &mut s;
        println!("{}", r3);
    }

    //
    // Dangling references
    //
    {
        // let reference_to_nothing = dangle();
        let reference_to_string = no_dangle();
    }


    //
    // The Rules of References
    // =======================
    // * At any given time, you can have either one mutable reference
    //   or any number of immutable references
    // * References must always be valid
    //
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// fn calculate_length_2(s: &String) -> (String, usize) {
fn calculate_length_2(s: &String) -> (&String, usize) {
    (s, s.len())
}

// fn change(s: &String) {
fn change(s: &mut String) {
    s.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
    
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}