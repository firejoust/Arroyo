mod arguments;
mod encoding;

use encoding::bencode::{Error, Encodable, DictionaryEntry, Dictionary};

fn main() -> Result<(), Error> {

    println!("String unwrap test:");
    println!("{}", String::from("Hello, World!").encode()?);

    println!("Int unwrap test:");
    println!("{}", (-0 as isize).encode()?);

    println!("List unwrap test:");
    let list: Vec<Box<dyn Encodable>> = vec![Box::new(1 as isize), Box::new("owo")];
    println!("{}", list.encode()?);

    println!("Dict unwrap test:");
    let dict: Dictionary = vec![
        DictionaryEntry::new("Key1", "Elem1"),
        DictionaryEntry::new("Key2", 2 as u8),
        DictionaryEntry::new("List1", list)
    ];
    println!("{}", dict.encode()?);


    Ok(())
}
