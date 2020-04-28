fn main() {
    // When Rustaceans refer to “strings” in Rust, 
    // they usually mean the String and the string 
    // slice &str types, not just one of those types.


    {
        let mut s = String::new();
    }

    {
        let data = "initial contents";
        let s = data.to_string();

        println!("{}", s);

        println!("{}", "hello".to_string());
    }

    {
        let s = String::from("hello你好");
        println!("{}", s);
    }

    {
        let ss = "foo";
        let mut s = String::from(ss);
        let ss = "bar";
        s.push_str(ss);
        s.push('x'); // push is not push_str
        println!("{}", s);
    }

    {
        let s1 = String::from("Hello,");
        let s2 = String::from("world!");
        let s3 = s1+&s2; // s1 has been moved here and can no longer be used
        // println!("{}{}{}",s1,s2,s3);
        println!("{}{}",s2,s3);

        // the + operator uses the add method
        // fn add(self, s:&str) -> String {}
        // This isn’t the exact signature that’s 
        // in the standard library: in the standard library, 
        // add is defined using generics. 

        // But wait—the type of &s2 is &String, not &str, 
        // as specified in the second parameter to add. 
        // So why does Listing 8-18 compile?

        // The reason we’re able to use &s2 in the call 
        // to add is that the compiler can coerce 
        // the &String argument into a &str. 
        // When we call the add method, 
        // Rust uses a deref coercion, 
        // which here turns &s2 into &s2[..]. 
        // We’ll discuss deref coercion in more depth 
        // in Chapter 15. Because add does not 
        // take ownership of the s parameter, 
        // s2 will still be a valid String 
        // after this operation.
    }

    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        // let s = s1+"-"+&s2+"-"+&s3;
        let s = format!("{}-{}-{}", s1, s2, s3);
    }

    {
        let s1 = String::from("hello");

        // The error and the note tell the story:
        // Rust strings don’t support indexing. 
        // let h = s1[0];
    }

    {
        // A String is a wrapper over a Vec<u8>
        // content can be alphabet or chinese
        // hello[0] is meaning nothing
        let hello = String::from("Hola");


        // A final reason Rust doesn’t allow us to index 
        // into a String to get a character is 
        // that indexing operations are expected to always 
        // take constant time (O(1)). But it isn’t possible 
        // to guarantee that performance with a String, 
        // because Rust would have to walk through the contents 
        // from the beginning to the index to determine 
        // how many valid characters there were.
    }

    // slicing Strings
    {
        let hello = "你好niao";
        // let s = &hello[0..1];
        // let s = &hello[0..2];
        // let s = &hello[0..4];
        let s = &hello[0..3];
        println!("{}", s);

        let s = &hello[3..6];
        println!("{}", s);

        let s = &hello[6..7];
        println!("{}", s);
    }

    {
        let hello = "你好niao";
        for c in hello.chars() { // auto split
            println!("{}", c);
        }
    }

    {
        let hello = "你好nihao";
        for b in hello.bytes() {
            println!("{}", b);
        }
    }
}
