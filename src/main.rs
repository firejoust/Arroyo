mod arguments;
mod encoding;

use encoding::bencode::{ByteStr, Int, Literal};

fn main() {
    println!("String unwrap test:");
    let bs = ByteStr::from_str("Hello, world!");
    match bs {
        Ok(v) => println!("{}", v.stringify().unwrap()),
        Err(e) => println!("{}", e)
    }

    println!("Int unwrap test:");
    let i = Int::from_isize(-0);
    println!("{}", i.stringify().unwrap())
}
