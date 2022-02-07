mod arguments;
mod encoding;

use encoding::bencode::{ByteStr, Int, List, Dict, ListEntry, DictEntry, Literal, Error};

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
    let arr: Vec<ListEntry> = vec![Box::new(Int::from(5)), Box::new(Int::from(10))];
    let l = List::from(arr);
    println!("{}", l.stringify()?);

    println!("Dict unwrap test:");
    let arr: Vec<DictEntry> = vec![(ByteStr::try_from_str("TestKey")?, Box::new(i))];
    let d = Dict::from(arr);
    println!("{}", d.stringify()?);

    Ok(())
}
