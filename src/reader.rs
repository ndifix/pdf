use std::io;

use crate::{Header, Pdf, Version};

pub fn parse_file(iter: &mut std::slice::Iter<u8>) -> Result<Pdf, io::Error> {
    let lines = separate_lines(iter);
    let mut iter = lines.iter();

    let header = parse_header(&mut iter)?;

    Ok(Pdf { header })
}

fn parse_header(iter: &mut std::slice::Iter<Vec<u8>>) -> Result<Header, io::Error> {
    let version = iter.next().unwrap();

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
            return Ok(Header { version });
        } else {
            let error = io::Error::new(io::ErrorKind::InvalidData, "Unknown version");
            return Err(error);
        }
    }

    let error = io::Error::new(io::ErrorKind::InvalidData, "Header semantic");
    return Err(error);
}

fn separate_lines(iter: &std::slice::Iter<u8>) -> Vec<Vec<u8>> {
    let all: Vec<u8> = iter
        .clone()
        .cloned()
        .collect();

    let lines: Vec<Vec<u8>> = all
        .split(|c| c == &b'\n')
        .map(|l| l.to_vec())
        .collect()
        ;

    lines
}

fn _skip_somment(iter: &mut std::slice::Iter<u8>) {
    while let Some(next) = iter.next() {
        if next == &b'\n' {
            return;
        }
    }
}
