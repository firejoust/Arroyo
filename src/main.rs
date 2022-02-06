mod arguments;
mod encoding;

use encoding::bencode::{ByteStr, Int, Literal, Error};

fn main() -> Result<(), Error> {
    println!("String unwrap test:");
    let bs =  match ByteStr::try_from_str("Hello, World!") {
        Ok(v) => println!("{}", v.stringify()?),
        Err(e) => println!("{:?}", e)
    };


    println!("Int unwrap test:");
    let i = Int::from(-0);
    println!("{}", i.stringify()?);

    Ok(())
}
