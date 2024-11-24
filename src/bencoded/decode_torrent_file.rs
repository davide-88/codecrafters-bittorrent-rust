use serde::de::Visitor;
use serde::{Deserialize, Deserializer};
use serde_bencode::from_bytes;
use std::fmt;
use std::result::Result;

#[derive(Debug)]
pub enum DecodeTorrentFileError {
    IoError(std::io::Error),
    SerdeError(serde_bencode::Error),
}

#[derive(Debug, Clone)]
pub struct Hashes(Vec<[u8; 20]>);
struct HashStrVistitor;

impl<'de> Visitor<'de> for HashStrVistitor {
    type Value = Hashes;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a byte string whose length is a multiple of 20")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v.len() % 20 != 0 {
            return Err(E::custom(format!(
                "byte string length is not a multiple of 20: {}",
                v.len()
            )));
        }

        Ok(Hashes(
            v.chunks_exact(20)
                .map(|chunk| chunk.try_into().expect("guaranteed to be of size 20"))
                .collect(),
        ))
    }
}

impl<'de> Deserialize<'de> for Hashes {
    fn deserialize<D>(deserializer: D) -> Result<Hashes, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(HashStrVistitor)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Torrent {
    // The URL of the tracker, which is a central server that keeps track
    // of peers partecipating int the sharing of a torrent.
    pub announce: String,
    // A dictionary that describes the file(s) of the torrent.
    // There are two possible forms:
    // one for the case of a 'single-file' torrent with no directory structure,
    // one for the case of a 'multi-file' torrent
    pub info: Info,
    // The string encoding format used to generate the pieces part of the info dictionary
    pub encoding: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Info {
    // In the single file case this is the suggested name to save the file as.
    // In the multiple file case, it's the name of the directory to save the files in.
    pub name: String,

    // number of bytes in each piece (integer)
    #[serde(rename = "piece length")]
    pub piece_length: usize,

    // string consisting of the concatenation of all 20-byte SHA1 hash values, one per piece (byte string, i.e. not urlencoded)
    pub pieces: Hashes,

    #[serde(flatten)]
    pub keys: Keys,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Keys {
    SingleFile {
        //The size of the file in bytes, for single-file torrents.
        length: usize,
        // A 32-character hexadecimal string corresponding to the MD5 sum of the file.
        md5sum: Option<String>,
    },
    MultiFiles {
        // The files list contains a list of dictionaries, one for each file.
        // For the purposes of the other keys in `Info`, the files are laid out in the order they appear in the files list.
        files: Vec<File>,
    },
}

#[derive(Debug, Clone, Deserialize)]
pub struct File {
    // The length of the file in bytes
    pub length: usize,
    // A list containing one or more string elements that together represent the path and filename.
    // Each element in the list corresponds to either a directory name or (in the case of the final element)
    // the filename. For example, a the file "dir1/dir2/file.ext" would consist of three string elements:
    // "dir1", "dir2", and "file.ext".
    pub path: Vec<String>,
    // A 32-character hexadecimal string corresponding to the MD5 sum of the file.
    pub md5sum: Option<String>,
}

pub fn decode_torrent_file(path_to_torrent_file: &str) -> Result<Torrent, DecodeTorrentFileError> {
    std::fs::read(path_to_torrent_file)
        .map_err(|e| DecodeTorrentFileError::IoError(e))
        .and_then(|torrent_content| {
            from_bytes::<Torrent>(&torrent_content)
                .map_err(|e| DecodeTorrentFileError::SerdeError(e))
        })
}
