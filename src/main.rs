mod arguments;
mod encoding;

use encoding::bencode;

fn main() {
    let bs = bencode::ByteStr::from("Hello, world!");
    match bs {
        Ok(v) => println!("{}", v.to_string().unwrap()),
        Err(e) => println!("{}", e)
    }
    
}
