fn main() {
    // slice type does not have ownership
    {
        let s = String::from("hello world");

        let pos = first_word(&s);

        // if s is cleared, pos will invalid
        // s.clear();

        if pos < s.len() {
            // println!("first word is:{}", s[pos]);
        }
    }

    // use slice
    {
        let s = String::from("hello world");
        let len = s.len();
        let hello = &s[0..5];
        let word = &s[6..11];

        println!("{} {}", hello, word);

        let slice = &s[0..2];
        let slice = &s[..2];

        let slice = &s[3..len];
        let slice = &s[3..];
    }

    // return slice
    {
        let mut s = String::from("hello world");

        let w = first_word_2(&s);

        // Because clear needs to truncate the String,
        // it needs to get a mutable reference,
        // however, s has an immutable reference w,
        // create another mutable reference will fail

        // s.clear();

        // it is safe to use w
        println!("The first world of s is:{}", w);

        // now, all references are gone, we can mut it
        s.clear();
    }

    // String Literals are slices
    {
        // The type of s is &str: it's a slice pointing to that 
        // specific point of the binary
        // This is also way string literals are immutable
        // &str is an immutable reference
        let s = "Hello, wolrd!";
    }

    // String Slices as Parameters
    {
        let s = "hello world";
        let ss = String::from("hello world");

        // s is &str, not a String
        // first_word_2(&s);
        first_word_2(&ss);

        // &str accept String and &str
        
        first_word_3(&ss);
        first_word_3(&s);
    }


    // list slice
    {
        let a = [1,2,3,4,5];
        let slice = &a[1..3];
    }
}

// fn first_word(s: &String) -> ?

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_3(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}