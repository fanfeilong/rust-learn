fn main() {
    
    {
        let v: Vec<i32> = Vec::new();
    }

    {
        let v = vec![1,2,3];
    }

    {
        let mut v = Vec::new();

        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
    }

    {
        {
            let v = vec![1,2,3,4,5];
        }// <- v goes out of scope and is freed here
    }

    {
        let v = vec![1,2,3,4,5];
        let third: &i32 = &v[2];
        println!("{}", third);

        match v.get(2) {
            Some(third) => println!("{}", third),
            None => println!("none"),
        }
    }

    {
        let v = vec![1,2,3,4,5];

        // let dose_not_exist = &v[100]; // panic
        let dose_not_exist = v.get(100);
    }

    {
        // adding a new element onto the end of the vector 
        // might require allocating new memory and copying 
        // the old elements to the new space, if there isn’t 
        // enough room to put all the elements next to each 
        // other where the vector currently is. In that case, 
        // the reference to the first element would be 
        // pointing to deallocated memory. 
        let mut v = vec![1,2,3,4,5];
        // let first = &v[0];
        let first = v[0];
        v.push(6);
        println!("{}", first);
    }

    {
        let v = vec![100,32,57];
        for i in &v {
            println!("{}", i);
        }
    }

    {
        let v = vec![100,32,57];
        for i in v {
            println!("{}", i);
        }
    }

    {
        let mut v = vec![100,32,57];
        for i in &mut v {
            *i += 50;
            println!("{}", i);
        }
    }

    {
        #[derive(Debug)]
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String)
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12)
        ];

        for i in &row {
            println!("{:#?}", i);
        }
    }
}
