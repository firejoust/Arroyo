mod arguments;
mod encoding;

use encoding::bencode::{ByteStr, Int, List, Literal, Error};

fn main() -> Result<(), Error> {
    
    println!("String unwrap test:");
    match ByteStr::try_from_str("Hello, World!") {
        Ok(v) => println!("{}", v.stringify()?),
        Err(e) => println!("{:?}", e)
    };

    println!("Int unwrap test:");
    let i = Int::from(-0);
    println!("{}", i.stringify()?);

    println!("List unwrap test:");
    let arr: Vec<Box<dyn Literal>> = vec![Box::new(Int::from(5)), Box::new(Int::from(10))];
    let i = List::from(arr);
    println!("{}", i.stringify()?);

    Ok(())
}
