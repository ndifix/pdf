use std::io;

use crate::{Header, Pdf, Version};

pub fn parse_file(iter: &mut std::slice::Iter<u8>) -> Result<Pdf, io::Error> {
    let header = parse_header(iter)?;

    Ok(Pdf { header })
}

fn parse_header(iter: &mut std::slice::Iter<u8>) -> Result<Header, io::Error> {
    let version: Vec<u8> = iter
        .take_while(|c| c.is_ascii_graphic())
        .cloned()
        .collect();

    if let Ok(str) = String::from_utf8(version.clone()) {
        let version = match str.as_str() {
            "%PDF-1.0" => Version::V1_0,
            "%PDF-1.1" => Version::V1_1,
            "%PDF-1.2" => Version::V1_2,
            "%PDF-1.3" => Version::V1_3,
            "%PDF-1.4" => Version::V1_4,
            "%PDF-1.5" => Version::V1_5,
            "%PDF-1.6" => Version::V1_6,
            "%PDF-1.7" => Version::V1_7,
            _ => Version::Unknown,
        };

        if version != Version::Unknown {
            skip_somment(iter);
            return Ok(Header { version });
        } else {
            let error = io::Error::new(io::ErrorKind::InvalidData, "Unknown version");
            return Err(error);
        }
    }

    let error = io::Error::new(io::ErrorKind::InvalidData, "Header semantic");
    return Err(error);
}

fn skip_somment(iter: &mut std::slice::Iter<u8>) {
    while let Some(next) = iter.next() {
        if next == &b'\n' {
            return;
        }
    }
}
