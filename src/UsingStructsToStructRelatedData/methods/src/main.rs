fn main() {
    // use method
    {
        let mut r = Rectangle {
            width: 30,
            height: 50
        };

        let a = r.area();
        println!("{}", a);

        r.setWidth(200);
        let a = r.area();
        println!("{}", a);
    }

    // parameters
    {
        let r = Rectangle {
            width: 30,
            height: 50
        };

        let r2 = Rectangle {
            width: 10,
            height: 20
        };

        let contain = r.can_hold(&r2);
        println!("r can hold r2? {}", contain);
    }

    // associated functions
    {
        let r = Rectangle::square(10);
    }

    // multi impl
    {
        let r = Rectangle::unit();
    }
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn setWidth(&mut self, width: u32) {
        self.width = width;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size}
    }
 }

 impl Rectangle {
     // add code here
     fn unit()->Rectangle{
         Rectangle { width:1, height:1}
     }
 }