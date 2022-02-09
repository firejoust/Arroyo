#[derive(Debug)]
pub enum Error {
    NonStandardChar(char)
}


pub trait Encodable {
    fn encode(&self) -> Result<String, Error>;
}

/*
**  Byte strings are encoded as follows: <string length encoded in base ten ASCII>:<string data>
**  Note that there is no constant beginning delimiter, and no ending delimiter.
*/
impl Encodable for &str {
    fn encode(&self) -> Result<String, Error> {
        let mut bstring: Vec<char> = Vec::new();
        for c in self.chars() {
            if c as u32 > 127 {
                return Err(Error::NonStandardChar(c))
            }

            bstring.push(c);
        }


        Ok(
            format!("{}:{}", bstring.len(), bstring.iter().collect::<String>())
        )
    }
}

impl Encodable for String {
    fn encode(&self) -> Result<String, Error> {
        Ok(Encodable::encode(&&self[..])?)
    }
}


/*
**  Integers are encoded as follows: i<integer encoded in base ten ASCII>e
**  The initial i and trailing e are beginning and ending delimiters.
*/
macro_rules! encode_int {
    ($int: ty) => {
        impl Encodable for $int {
            fn encode(&self) -> Result<String, Error> {
                Ok(format!("i{}e", self))
            }
        }
    };
}
encode_int!(i8);    encode_int!(u8);
encode_int!(i16);   encode_int!(u16);
encode_int!(i32);   encode_int!(u32);
encode_int!(i64);   encode_int!(u64);
encode_int!(isize); encode_int!(usize);


/*
**  Lists are encoded as follows: l<bencoded values>e
**  The initial l and trailing e are beginning and ending delimiters.
**  Lists may contain any bencoded type, including integers, strings, dictionaries, and even lists within other lists.
*/
impl Encodable for Vec<Box<dyn Encodable>> {
    fn encode(&self) -> Result<String, Error> {
        let mut bstring: Vec<char> = Vec::new();
        
        for item in self {
            bstring.extend(item.encode()?.chars())
        }

        Ok(format!("l{}e", bstring.iter().collect::<String>()))
    }
}


/*
**  Dictionaries are encoded as follows: d<bencoded string><bencoded element>e
**  The initial d and trailing e are the beginning and ending delimiters.
**  The values may be integers, strings, lists, and other dictionaries.
*/
// Dictionary encoding is seperate from encoding a regular vector as each entry needs to be sorted in
// lexicogrpahical order of the key as per specification.
pub type Dictionary = Vec<DictionaryEntry>;
pub struct DictionaryEntry {
    key: String,
    elem: Box<dyn Encodable>
}

impl DictionaryEntry {
    /// Create a new dictionary entry from a key and an encodable element
    pub fn new<T: Encodable+'static>(key: &str, elem: T) -> Self {
        Self { 
            key: key.to_string(),
            elem: Box::new(elem)
        }
    }
}

impl Encodable for DictionaryEntry {
    fn encode(&self) -> Result<String, Error> {
        Ok(
            format!("{}{}", self.key.encode()?, self.elem.encode()?)
        )
    }
}

impl Encodable for Vec<DictionaryEntry> {
    fn encode(&self) -> Result<String, Error> {
        let mut entries = Vec::new();
        for item in self {
            entries.push(item);
        }
        entries.sort_by(|a, b| a.key.cmp(&b.key));

        let mut bstring: Vec<char> = Vec::new();
        for item in entries {
            bstring.extend(item.encode()?.chars());

        }

        Ok(format!("d{}e", bstring.iter().collect::<String>()))
    }
}