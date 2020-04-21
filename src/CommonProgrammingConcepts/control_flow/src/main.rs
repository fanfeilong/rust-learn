fn main() {
    println!("Hello, world!");

    let v = control();
    println!("The value of control is:{}", v);

    let v = control2();
    println!("The value of control2 is:{}", v);

    let v = func_loop();
    println!("The value of func_loop is:{}", v);

    let v = func_while();
    println!("The value of func_while is:{}", v);

    func_for();
}

fn control()->i32 {
    let num = 10;
    let y = num;

    if num < 100 {
        println!("less then 1000");
        y - 1
    } else if num == 100 {
        println!("equal then 1000");
        y
    } else {
        println!("great then 1000");
        y + 1
    }
}

fn control2()->i32 {
    let num = 10;
    let y = num;

    let z = if num < 100 {
        println!("less then 1000");
        y - 1
    } else if num == 100 {
        println!("equal then 1000");
        y
    } else {
        println!("great then 1000");
        y + 1
    };

    z
}

fn func_loop()->i32{

    let mut i = 0;
    let count = 10;
    loop {
        print!(".");
        i += 1;
        if i > 10 {
            break;
        }
    }

    loop {
        print!(".");
        i += 1;
        if i > 10 {
            break i+10;
        }
    }
}

fn func_while()->i32{

    let mut i = 0;
    let count = 10;
    while  i < 10 {
        print!(".");
        i += 1;
    }

    while  i < 100 {
        print!(".");
        i += 1;
        if i > 20 {
            break;
        }
    }

    10
}

fn func_for()->i32{
    let a = [10,20];
    for e in a.iter() {
        println!("element in array:{}", e);
    }

    10
}