# fasta-rs

FASTA file parser implemented in Rust using [Chumsky](https://github.com/zesterer/chumsky).

## Usage

```rs
use fasta_rs::{parser, Parser, Record};

fn main() {
    let fasta_file = String::from(">...");
    let records: Vec<Record> = parser().parse(fasta_file).unwrap();
}
```

```
❯ cargo run -q -- example.fa
[
    Record {
        identifier: "gi|2765658|emb|Z78533.1|CIZ78533",
        description: "C.irapeanum 5.8S rRNA gene and ITS1 and ITS2 DNA",
        sequence: "CGTAACAAGGTTTCCGTAGGTGAACCTGCGGAAGGATCATTGATGAGACCGTGGAATAAACGATCGAGTGAATCCGGAGGACCGGTGTACTCAGCTCACCGGGGGCATTGCTCCCGTGGTGACCCTGATTTGTTGTTGGGCCGCCTCGGGAGCGTCCATGGCGGGTT",
    },
    Record {
        identifier: "gi|2765658|emb|Z78533.1|CIZ78533",
        description: "C.irapeanum 5.8S rRNA gene and ITS1 and ITS2 DNA",
        sequence: "CGTAACAAGGTTTCCGTAGGTGAACCTGCGGAAGGATCATTGATGAGACCGTGGAATAAACGATCGAGTGAATCCGGAGGACCGGTGTACTCAGCTCACCGGGGGCATTGCTCCCGTGGTGACCCTGATTTGTTGTTGGGCCGCCTCGGGAGCGTCCATGGCGGGTT",
    },
]
```
