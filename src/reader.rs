use std::io;

use crate::{Header, IndirectObject, Pdf, Version};

pub fn parse_file(iter: &mut std::slice::Iter<u8>) -> Result<Pdf, io::Error> {
    let lines = separate_lines(iter);
    let mut iter = lines.iter();

    let header = parse_header(&mut iter)?;
    parse_body(&mut iter);

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

fn parse_body(iter: &mut std::slice::Iter<Vec<u8>>) {
    let xref =  "xref".as_bytes();
    let body_lines: Vec<Vec<u8>>  = iter
        .skip_while(|c| c.starts_with(b"%"))
        .take_while(|c| c != &xref)
        .cloned()
        .collect()
        ;

    let get_objects = |body_lines: Vec<Vec<u8>>| -> Vec<Vec<Vec<u8>>> {
        let endobj = "endobj".as_bytes();
        let mut objects: Vec<Vec<Vec<u8>>> = body_lines
            .split(|line| line == endobj)
            .map(|line| line.to_vec())
            .collect()
            ;

        objects.pop();
        for object in &mut objects {
            object.push(endobj.to_vec());
        };

        return objects;
    };

    let objects = get_objects(body_lines);

    let parse_indirect_object = |obj: &Vec<Vec<u8>>| -> IndirectObject {
        let parse_first_line = |iter: &mut std::slice::Iter<Vec<u8>>| -> (u32, u32, String) {
            let def: Vec<&[u8]> = iter
                .next().unwrap()
                .split(|c| c == &b' ')
                .collect()
                ;
            let id: u32 = String::from_utf8(def[0].to_vec()).unwrap().parse().unwrap();
            let gen: u32 = String::from_utf8(def[1].to_vec()).unwrap().parse().unwrap();
            let desc = String::from_utf8(def[2].to_vec()).unwrap();

            (id, gen, desc)
        };

        let mut iter = obj.iter();
        let def = parse_first_line(&mut iter);
        IndirectObject {
            _id: def.0,
            _generation: def.1,
            _desc: def.2
        }
    };

    let _objects: Vec<IndirectObject> = objects
        .iter()
        .map(|obj| parse_indirect_object(obj))
        .collect();
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
