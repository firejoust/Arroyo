pub trait Literal {
    fn stringify(&self) -> Result<String, Error>;
}


#[derive(Debug)]
pub enum Error {
    FromUtf8,
    NonStandardChar(char)
}

/*
**  Bencode data structures
**  (https://wiki.theory.org/index.php/BitTorrentSpecification#Bencoding)
*/

/*
**  Byte strings are encoded as follows: <string length encoded in base ten ASCII>:<string data>
**  Note that there is no constant beginning delimiter, and no ending delimiter. 
*/

pub struct ByteStr {
    content: Vec<u8>
}

impl Literal for ByteStr {

    // create a String representation from ASCII sequence
    fn stringify(&self) -> Result<String, Error> {
        let arr = &self.content;
        let ascii_result = String::from_utf8(arr.to_vec());
        match ascii_result {
            Ok(trailer) => Ok(format!("{}:{}", arr.len(), trailer)),
            Err(_) => Err(Error::FromUtf8)
        }
    }
}

impl ByteStr {

    // create a ByteStr from a string slice
    pub fn try_from_str(ascii_128_str: &str) -> Result<Self, Error> {
        let mut arr: Vec<u8> = vec![];
        // allocate chars to ASCII decimal vec
        for c in ascii_128_str.chars() {
            let decimal = c as usize;
            // char incompatible with bencode
            if decimal > 127 {
                return Err(Error::NonStandardChar(c))
            }
            arr.push(c as u8);
        }
        Ok(
            ByteStr { 
                content: arr
            }
        )
    }
}

/*
**  Integers are encoded as follows: i<integer encoded in base ten ASCII>e
**  The initial i and trailing e are beginning and ending delimiters. 
*/

pub struct Int {
    content: isize
}

impl Literal for Int {

    fn stringify(&self) -> Result<String, Error> {
        Ok(format!("i{}e", self.content))
    }
}

impl From<isize> for Int {
    fn from(content: isize) -> Int {
        Int {
            content
        }
    }
}