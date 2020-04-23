fn main() {

    // basic 
    {
        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;

        println!("{:?}", four);
        println!("{:?}", six);
    }

    // use
    {
        route(IpAddrKind::V4);
    }

    // compound
    {
        let home = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1")
        };

        let loopback = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from("::1")
        };
    }

    // advance enum
    {
        let home = IpAddrA::V4(String::from("127.0.0.1"));
        let loopback = IpAddrA::V6(String::from("::1"));
    }

    // different field type
    {
        let home = IpAddrB::V4(127,0,0,1);
        let loopback = IpAddrB::V6(String::from("::1"));
    }

    // any sub type
    {
        let home = IpAddrC::V4(Ipv4Addr{});
        let loopback = IpAddrC::V6(Ipv6Addr{});
    }

    // union type
    {
        let m = Message::Write(String::from("hello"));
        m.call();
    }

    // option
    {
        let some_number = Some(5);
        let some_string = Some("a string");
        let absent_number: Option<i32> = None;
    }

    {
        let x: i8 = 5;
        let y: Option<i8> = Some(5);

        let sum = x + y;
    }
}


#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind){

}

#[derive(Debug)]
struct  IpAddr {
    kind: IpAddrKind,
    address: String
}

#[derive(Debug)]
enum  IpAddrA {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum  IpAddrB {
    V4(u8,u8,u8,u8),
    V6(String),
}

struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddrC {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

enum Message{
    Quit,                       // no data associated
    Move { x: i32, y: i32 },    // includes an anonymous struct
    Write(String),              // includes a single string
    ChangeColor(i32, i32, i32)  // includes three i32 values
}

impl Message {
    fn call(&self){
        // what is self?
    }
}