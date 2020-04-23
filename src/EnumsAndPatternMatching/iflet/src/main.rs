fn main() {
   {
       let some_u8_value = Some(3);
       match some_u8_value {
           Some(3) => println!("three"),
           _ => (),
       }
   }

   {
        let some_u8_value = Some(4);
        if let Some(3) = some_u8_value {
            println!("three");
        } else {
            println!("other");
        }
   }
}