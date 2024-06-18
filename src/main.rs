use std::env;

use pdf::get_pdf;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = parse_args(&args);

    get_pdf(&args.pdf_path);
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
