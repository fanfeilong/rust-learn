// #[cfg(front_of_house)]
mod front_of_house {

    pub mod hosting {
        pub fn add_to_wait_list(){
            assert_eq!(2 + 2, 4);
        }

        #[test]
        fn seat_at_table(){

        }
    }

    pub mod serving {
        fn take_order(){

        }

        fn serve_order(){

        }

        fn take_payment(){

        }

        pub fn eat_at_restaurant(){

            let order1 = back_of_house::Appetizer::Salad;

            let mut meal = back_of_house::Breakfast::summer("Rye");

            meal.toast = String::from("Wheat");

            println!("I'd like {} toast please", meal.toast);
        }

        mod back_of_house {

            pub enum Appetizer {
                Soup,
                Salad,
            }

            #[derive(Debug)]
            pub struct  Breakfast {
                pub toast: String,
                seasonal_fruit: String,
            }

            impl Breakfast {
                pub fn summer(toast: &str) -> Breakfast {
                    Breakfast{
                        toast: String::from(toast),
                        seasonal_fruit: String::from("peaches")
                    }
                }
            }
            
            fn fix_incorrect_order(){
                cook_order();
                super::serve_order();
                super::super::hosting::add_to_wait_list();
                // super::super::hosting::seat_at_table();
            }

            fn cook_order(){

            }
        }
    }
}

// use crate::front_of_house::hosting;
use front_of_house::hosting;
use front_of_house::hosting::add_to_wait_list;

use std::collections::HashMap;

// use std::fmt;
// use std::io::Result as IOResult;

// fn function1() -> fmt::Result {

// }

// fn function2() -> IOResult<()> {
//    IOResult{

//    }
// }

pub use crate::front_of_house::serving;

use rand::Rng;

use std::io;
// use std::cmp::Ordering;
// use std::{cmp::Ordering, io};
use std::{self, cmp::Ordering,};

use std::collections::*;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_wait_list();

    // Relative path
    front_of_house::hosting::add_to_wait_list();

    // use
    hosting::add_to_wait_list();

    // use to fn
    add_to_wait_list();

    crate::front_of_house::serving::eat_at_restaurant();

    let mut map = HashMap::new();
    map.insert(1,2);

    let secret_number = rand::thread_rng().gen_range(1,101);
}