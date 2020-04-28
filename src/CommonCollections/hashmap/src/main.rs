use std::collections::HashMap;

fn main() {
    // Hash<K,V>
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);        
        scores.insert(String::from("Yellow"), 50);
        
        match scores.get("Blue") {
            Some(d) => println!("{}", d),
            None => println!("none"),
        }
    }

    // Processing a Series of items with iterators
    {
        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10,50];
        let mut scores: HashMap<_,_> = teams.into_iter().zip(
            initial_scores.into_iter()
        ).collect();
    }

    // ownership
    {
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);

        // println!("{}{}", field_name, field_value);
    }

    // get
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let team_name = String::from("Blue");
        let score = scores.get(&team_name);

        for (key, value) in & scores {
            println!("{}{}", key, value);
        }
    }

    // overwrite
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 50);

        println!("{:?}", scores);
    }


    // entry
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);

        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);

        println!("{:?}", scores);
    }

    // update if old
    {
        let text = "hello world wonderful world";

        let mut  map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            * count += 1;
        }

        println!("{:?}", map);
    }
}
