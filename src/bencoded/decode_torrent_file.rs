use serde::Deserialize;
use serde_bencode::from_bytes;
use serde_bytes::ByteBuf;
use std::result::Result;

#[derive(Debug)]
pub enum DecodeTorrentFileError {
    IoError(std::io::Error),
    SerdeError(serde_bencode::Error),
}

/**
* A torrent file (also known as metainfo file) cotains a bencoded dictoruary with the
* following keys and values:
* - announce: the URL of the tracker, which is a central server that keeps track
*   of peers partecipating int the sharing of a torrent.
* - info: a dictionary with the following keys and values:
*   - length: the size of the file in bytes, for single-file torrents.
*   - name: suggested name to save the file/directory as
*   - piece length: the number of bytes in each piece
*   - pieces: concatenated SHA-1 hashes of each piece
*
* Note: .torrent files contain bytes that aren't valid UTF-8 characters.
* You'll run into probles if you try to read the file as a string. Use &[u8] or Vec<u8> instead.
*
* Note: The info dictionary looks slightly different for multi-file torrents.
* For this challange, we'll only implement support for single-file torrents.
*
* In this stage, we'll focus on extracting the tracker URL and the lenght of the file (in bytes).
*/

#[derive(Debug, Clone, Deserialize)]
pub struct Torrent {
    pub announce: String,
    pub info: Info,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Info {
    pub length: u64,
    pub name: String,
    #[serde(rename = "piece length")]
    pub piece_length: u64,
    pub pieces: ByteBuf,
}

pub fn decode_torrent_file(path_to_torrent_file: &str) -> Result<Torrent, DecodeTorrentFileError> {
    std::fs::read(path_to_torrent_file)
        .map_err(|e| DecodeTorrentFileError::IoError(e))
        .and_then(|torrent_content| {
            from_bytes::<Torrent>(&torrent_content)
                .map_err(|e| DecodeTorrentFileError::SerdeError(e))
        })
}

/*
#[cfg(test)]
mod tests {
    use super::decode_bencoded_value;

    #[test]
    fn parse_string() {
        let result = decode_bencoded_value(&"5:hello");
        assert_eq!(result, serde_json::Value::String(String::from("hello")));
    }

    #[test]
    fn parse_integer() {
        let result = decode_bencoded_value(&"i52e");
        assert_eq!(
            result,
            serde_json::Value::Number(serde_json::Number::from(52))
        );
    }

    #[test]
    fn parse_list() {
        let result = decode_bencoded_value(&"l5:helloi52ee");
        assert_eq!(
            result,
            serde_json::Value::Array(vec![
                serde_json::Value::String(String::from("hello")),
                serde_json::Value::Number(serde_json::Number::from(52))
            ])
        );
    }

    #[test]
    fn parse_nested_list() {
        let result = decode_bencoded_value(&"l5:helloli52e3:treee");
        assert_eq!(
            result,
            serde_json::Value::Array(vec![
                serde_json::Value::String(String::from("hello")),
                serde_json::Value::Array(vec![
                    serde_json::Value::Number(serde_json::Number::from(52)),
                    serde_json::Value::String(String::from("tre"))
                ]),
            ])
        );
    }

    #[test]
    fn parse_dictionary() {
        let test_cases = vec![
            (
                "d3:foo3:bar5:helloi52ee",
                serde_json::Value::Object(serde_json::Map::from_iter(vec![
                    (
                        String::from("foo"),
                        serde_json::Value::String(String::from("bar")),
                    ),
                    (
                        String::from("hello"),
                        serde_json::Value::Number(serde_json::Number::from(52)),
                    ),
                ])),
            ),
            (
                "d3:barli25el3:fooi-43ee5:helloee",
                serde_json::Value::Object(serde_json::Map::from_iter(vec![(
                    String::from("bar"),
                    serde_json::Value::Array(vec![
                        serde_json::Value::Number(serde_json::Number::from(25)),
                        serde_json::Value::Array(vec![
                            serde_json::Value::String(String::from("foo")),
                            serde_json::Value::Number(serde_json::Number::from(-43)),
                        ]),
                        serde_json::Value::String(String::from("hello")),
                    ]),
                )])),
            ),
        ];
        test_cases.iter().for_each(|(input, expected)| {
            let result = decode_bencoded_value(input);
            assert_eq!(result, *expected);
        });
    }
}
*/
