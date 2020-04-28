use std::fs;
use std::fs::File;
use std::io;
use std::error::Error;
use std::io::Read;

fn main() -> Result<(),Box<dyn Error>> {

    // recover able error
    // using Result<T,E> for recoverable errors
    {
        // file not found? report and retry
    }

    // un recover able error
    // using panic! macro that stops execution 
    // when the program encounters an unrecoverable error
    {
        // symptoms of bugs
        // access a location beyond the end of an array
    }

    {
        // panic!("crash and burn");
    }

    {
        // let v = vec![1,2,3];
        // v[99];
    }

    {
        // enum Result<T, E> {
        //     Ok(T),
        //     Err(E),
        // }
    }

    {
        // use std::fs::File;

        // let f = File::open("hello.txt");

        // let f = match f {
        //     Ok(file) => file,
        //     Err(x) => panic!("Problem opening the file: {:?}", x),
        // };
    }

    {
        // use std::fs::File;
        // use std::io::ErrorKind;

        // let f = File::open("hello.txt");

        // let f = match f {
        //     Ok(file) => file,
        //     Err(x) => match x.kind() {
        //         ErrorKind::NotFound => match File::create("hello.txt") {
        //             Ok(fc) => fc,
        //             Err(e) => panic!("Problem creating the file: {:?}", e),
        //         },
        //         other_error => {
        //             panic!("Problem openinig the file:{:?}", other_error);
        //         },
        //     }
        // };
    }

    {
        // use std::fs::File;
        // use std::io::ErrorKind;

        // let f = File::open("hello.txt").unwrap_or_else(|error| {
        //     if error.kind() == ErrorKind::NotFound {
        //         File::create("hello.txt").unwrap_or_else(|error|{
        //             panic!("Problem openinig the file:{:?}", error);
        //         })
        //     }else {
        //         panic!("Problem openinig the file:{:?}", error);
        //     }
        // });

    }

    {
        // use std::fs::File;
        // use std::io::ErrorKind;

        // let f = File::open("hello2.txt").unwrap();
    }

    {
        // use std::fs::File;
        // use std::io::ErrorKind;

        // let f = File::open("hello2.txt").expect("Failed to open hello2.txt");
    }

    {
        // let f = read_username_from_file().expect("Faile!!!!");
    }

    {
        // let f = read_username_from_file_short().expect("Faile....");
    }

    {
        // let f = read_username_from_file_short_short().expect("Faile....");
    }

    {
        // let f = read_username_from_file_short_short_short().expect("XXXXXX");
    }

    let f = File::open("xxxx")?;
    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello2.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut f = File::open("hellox.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}


fn read_username_from_file_short_short() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hellox.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_short_short_short() -> Result<String, io::Error> {
    fs::read_to_string("helloxx.txt")
}