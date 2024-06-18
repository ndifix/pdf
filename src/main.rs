use std::{env, fs, io::Read};

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = parse_args(&args);

    read_file(&args.pdf_path);
}

struct Arguments {
    #[allow(dead_code)]
    self_path: String,
    pdf_path: String,
}

fn parse_args(args: &Vec<String>) -> Arguments {
    if args.len() == 1 {
        panic!("No file specified");
    };

    Arguments {
        self_path: args[0].clone(),
        pdf_path: args[1].clone(),
    }
}

fn read_file(path: &str) {
    let mut file = match fs::File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("failed to open file: {e}"),
    };

    let mut buf: Vec<u8> = Vec::new();
    let _size = match file.read_to_end(&mut buf) {
        Ok(size) => size,
        Err(e) => panic!("failed to read file: {e}"),
    };
}
