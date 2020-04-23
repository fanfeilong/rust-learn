fn main() {
    // create an instance of User struct
    {
        let mut user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };
    
        println!("The value of user1.username is:{}", user1.username);
    
        user1.email = String::from("changed@example.com");
    
        println!("The value of user1.email is:{}", user1.email);
    }
    
    // create structure instance from function
    {
        let email = String::from("someone@example.com");
        let username = String::from("someone123");
        let user1 = build_user(email, username);
        // let user2 = build_user(email, String::from("someone123"));
        let user2 = build_user(String::from("someone@example.com"), String::from("someone123"));

        println!("active:{}", user2.active);

        let user3 = User{
            email: String::from("another@example.com"),
            username: String::from("anotherusername123"),
            ..user1
        };
        println!("username:{}", user1.username);
        println!("username:{}", user3.username);

        // let user4 = User{
        //     email: String::from("another@example.com"),
        //     ..user1 // user1.username will be move
        // };
        // println!("username:{}", user1.username);
        // println!("username:{}", user4.username);
    }

    // tuple structure
    {
        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);

        let black = Color(0,0,0);
        let origin = Point(0,0,0);
        let white = (255,255,255);

        let (r, g, b) = white;
        println!("r:{}, g:{}, b:{}", r, g, b);

        let Color(r,g,b) = black;
        println!("r:{}, g:{}, b:{}", r, g, b);
    }

    // Unit structure
    {
        struct  Unit {}

        let u = Unit{};
    }
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}


fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username,
        active: true,
        sign_in_count: 1,
    }
}