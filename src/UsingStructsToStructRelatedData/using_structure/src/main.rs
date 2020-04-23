fn main() {

    // calculate area
    {
        let width = 30;
        let height = 50;
        println!(
            "The area of the rectangle is {} square pixels.", 
            area(width, height)
        );
    }

    // use tuple
    {
        let r = (30, 50);
        println!(
            "The area of the rectangle is {} square pixels.", 
            area2(r)
        );
    }

    // use structure
    {
        let r = Rectangle{
            width: 30,
            height: 50
        };

        println!(
            "The area of the rectangle is {} square pixels.", 
            area3(&r)
        );
    }


    // derived traits
    {
        let r = Rectangle{
            width: 30,
            height: 50
        };

        // println!("{}", r);
        println!("{:?}", r);
        println!("{:#?}", r);
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(r: (u32, u32)) -> u32 {
   let (width, height) = r;
   width * height
}

fn area3(r: &Rectangle) -> u32 {
    r.width * r.height
 }