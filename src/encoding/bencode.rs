#[derive(Debug)]
pub enum Error {
    FromUtf8,
    NonStandardChar(char)
}

pub trait Literal {
    fn stringify(&self) -> Result<String, Error>;
}

// non primative bencode types
pub type ListEntry = Box<dyn Literal>;
pub type DictEntry = (ByteStr, Box<dyn Literal>);

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
        let mut content: Vec<u8> = vec![];

        // allocate chars to ASCII decimal vec
        for c in ascii_128_str.chars() {
            let decimal = c as usize;
            // char incompatible with bencode
            if decimal > 127 {
                return Err(Error::NonStandardChar(c))
            }
            content.push(c as u8);
        }

        Ok(ByteStr { content })
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
        Int { content }
    }
}

/*
**  Lists are encoded as follows: l<bencoded values>e
**  The initial l and trailing e are beginning and ending delimiters.
**  Lists may contain any bencoded type, including integers, strings, dictionaries, and even lists within other lists.
*/

pub struct List {
    content: Vec<ListEntry>
}

impl Literal for List {
    fn stringify(&self) -> Result<String, Error> {
        let mut ascii = String::new();
        for i in self.content.iter() {
            let result = i.stringify();
            match result {
                Ok(v) => ascii.push_str(v.as_str()),
                Err(e) => return Err(e)
            }
        }
        Ok(format!("l{}e", ascii))
    }
}

impl From<Vec<ListEntry>> for List {
    fn from(content: Vec<ListEntry>) -> Self {
        List { content }
    }
}

/*
**  Dictionaries are encoded as follows: d<bencoded string><bencoded element>e
**  The initial d and trailing e are the beginning and ending delimiters.
**  The values may be integers, strings, lists, and other dictionaries.
*/

pub struct Dict {
    content: Vec<DictEntry>
}

impl Literal for Dict {
    fn stringify(&self) -> Result<String, Error> {
        let mut ascii = String::new();
        for (key, value) in self.content.iter() {
            match key.stringify() {
                Ok(v) => ascii.push_str(v.as_str()),
                Err(e) => return Err(e)
            }
            match value.stringify() {
                Ok(v) => ascii.push_str(v.as_str()),
                Err(e) => return Err(e)
            }
        }
        Ok(format!("d{}e", ascii))
    }
}

impl From<Vec<DictEntry>> for Dict {
    fn from(content: Vec<DictEntry>) -> Self {
        Dict { content }
    }
}