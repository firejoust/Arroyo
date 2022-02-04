use std::string::FromUtf8Error;

#[derive(Debug)]
pub struct ByteStr {
    content: Vec<u8>
}

impl ByteStr {
    // create a ByteStr from an ascii string
    pub fn from(ascii_128_str: &str) -> Result<Self, &str> {
        let mut arr: Vec<u8> = vec![];
        // allocate chars to ASCII decimal vec
        for c in ascii_128_str.chars() {
            let decimal = c as usize;
            // char incompatible with bencode
            if decimal > 127 {
                return Err::<Self, &str>("Char does not conform with ascii-128 standard!")
            }
            arr.push(c as u8);
        }
        Ok(
            ByteStr { 
                content: arr
            }
        )
    }

    pub fn to_string(&self) -> Result<String, FromUtf8Error> {
        let arr = &self.content;
        let ascii_result = String::from_utf8(arr.to_vec());
        match ascii_result {
            Ok(trailer) => Ok(format!("{}:{}", arr.len(), trailer)),
            Err(e) => Err(e)
        }
    }
}