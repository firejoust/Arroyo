pub mod bencode;

use bencode::{Encodable, Error};

struct Info {
    name: String,
    pieces: String,
    piece_length: u32,
    private: bool
}

struct FileInfo {
    content: Info,
    path: String,
    md5sum: String,
    length: u32
}

struct DirInfo {
    content: Info,
    files: Vec<FileInfo>
}

struct Torrent {
    created_by: String,
    creation_date: String,
    comment: String,
    encoding: String,
    announce: String,
    announce_list: Vec<String>
}

// Allow encoding of torrents using intermediate type structures
// Todo: use iterator? All properties should be encodable

impl Encodable for Info {
    fn encode(&self) -> Result<String, Error> {
        let mut result = String::new();
        result.extend(self.name.encode()?.chars());
        result.extend(self.pieces.encode()?.chars());
        result.extend(self.piece_length.encode()?.chars());
        result.extend(self.private.to_string().encode()?.chars());
        Ok(result)
    }
}

impl Encodable for FileInfo {
    fn encode(&self) -> Result<String, Error> {
        let mut result = String::new();
        result.extend(self.content.encode()?.chars());
        result.extend(self.path.encode()?.chars());
        result.extend(self.md5sum.encode()?.chars());
        result.extend(self.length.encode()?.chars());
        Ok(result)
    }
}

impl Encodable for DirInfo {
    fn encode(&self) -> Result<String, Error> {
        todo!()
    }
}

impl Encodable for Torrent {
    fn encode(&self) -> Result<String, Error> {
        todo!()
    }
}