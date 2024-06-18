use std::{fs, io::Read};
mod reader;

pub struct Pdf {
    header: Header,
}

struct Header {
    version: Version,
}

#[derive(PartialEq, Debug)]
enum Version {
    Unknown,
    V1_0,
    V1_1,
    V1_2,
    V1_3,
    V1_4,
    V1_5,
    V1_6,
    V1_7,
}

pub fn show_pdf(pdf: &Pdf) {
    println!("version: {:?}", pdf.header.version);
}

pub fn get_pdf(path: &str) -> Pdf {
    let mut file = match fs::File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("failed to open file {path}: {e}"),
    };

    let mut buf: Vec<u8> = Vec::new();
    let _length = match file.read_to_end(&mut buf) {
        Ok(len) => len,
        Err(e) => panic!("Failed to read file: {e}"),
    };

    let mut iter = buf.iter();

    let pdf = match reader::parse_file(&mut iter) {
        Ok(pdf) => pdf,
        Err(e) => panic!("Failed to parse file: {e}"),
    };

    return pdf;
}
