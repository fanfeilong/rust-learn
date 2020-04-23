fn main() {
    {
        let coin = Coin::Quarter(UsState::Alabama);
        let v = value_in_cents(coin);
        println!("{}", v);
    }

    {
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
    }

    {
        let some_u8_value  =1;
        match some_u8_value {
            1 => println!("one"),
            _ => (),
        }
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Lucky Quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i+1),
        None => None,
    }
}

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(i) => i+1,
//         // None => None,
//     }
// }