use fasta_rs::{parser, Parser};

fn main() {
    let fasta_file = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    println!("{:#?}", parser().parse(fasta_file).unwrap());
}
