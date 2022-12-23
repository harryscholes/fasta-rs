pub use chumsky::{prelude::*, Error};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record {
    identifier: String,
    description: String,
    sequence: String,
}

fn dna_base<E: Error<char>>() -> impl Parser<char, char, Error = E> {
    one_of("ACGT")
}

fn dna_sequence<E: Error<char>>() -> impl Parser<char, Vec<char>, Error = E> {
    dna_base().repeated()
}

fn sequence<E: Error<char>>() -> impl Parser<char, String, Error = E> {
    dna_sequence()
        .chain(
            text::newline()
                .ignore_then(dna_sequence())
                .repeated()
                .flatten(),
        )
        .collect::<Vec<char>>()
        .collect()
}

fn identifier<E: Error<char>>() -> impl Parser<char, String, Error = E> {
    just('>')
        .ignore_then(filter(|c: &char| !c.is_ascii_whitespace()).repeated())
        .padded()
        .collect()
}

fn description<E: Error<char>>() -> impl Parser<char, String, Error = E> {
    filter(|c: &char| *c != '\n')
        .repeated()
        .then_ignore(text::newline().repeated())
        .padded()
        .collect()
}

pub fn parser() -> impl Parser<char, Vec<Record>, Error = Simple<char>> {
    identifier()
        .then(description())
        .then(sequence())
        .map(|((identifier, description), sequence)| Record {
            identifier,
            description,
            sequence,
        })
        .repeated()
        .collect()
        .then_ignore(end())
}

#[cfg(test)]
mod tests {
    use chumsky::error::Cheap;

    use super::*;

    #[test]
    fn parse_dna_base() {
        assert_eq!(dna_base::<Cheap<char>>().parse("A"), Ok('A'));
        assert_eq!(dna_base::<Cheap<char>>().parse("C"), Ok('C'));
        assert_eq!(dna_base::<Cheap<char>>().parse("G"), Ok('G'));
        assert_eq!(dna_base::<Cheap<char>>().parse("T"), Ok('T'));
        dna_base::<Cheap<char>>().parse("X").unwrap_err();
    }

    #[test]
    fn parse_dna_sequence() {
        assert_eq!(dna_sequence::<Cheap<char>>().parse("A"), Ok(vec!['A']));
        assert_eq!(dna_sequence::<Cheap<char>>().parse("T"), Ok(vec!['T']));
        assert_eq!(
            dna_sequence::<Cheap<char>>().parse("AT"),
            Ok(vec!['A', 'T'])
        );
        assert_eq!(
            dna_sequence::<Cheap<char>>().parse("ATX"),
            Ok(vec!['A', 'T'])
        );
        assert_eq!(dna_sequence::<Cheap<char>>().parse("AXT"), Ok(vec!['A']));
        assert_eq!(dna_sequence::<Cheap<char>>().parse("XAT"), Ok(vec![]));
    }

    #[test]
    fn parse_sequence() {
        assert_eq!(sequence::<Cheap<char>>().parse("A"), Ok("A".to_string()));
        assert_eq!(sequence::<Cheap<char>>().parse("C\n"), Ok("C".to_string()));
        assert_eq!(
            sequence::<Cheap<char>>().parse("A\nC"),
            Ok("AC".to_string())
        );
        assert_eq!(
            sequence::<Cheap<char>>().parse("A\nG\n"),
            Ok("AG".to_string())
        );
        assert_eq!(
            sequence::<Cheap<char>>().parse("A\nT\n\n"),
            Ok("AT".to_string())
        );
    }

    #[test]
    fn parse_identifier() {
        assert_eq!(
            identifier::<Cheap<char>>().parse(">i|d "),
            Ok("i|d".to_string())
        );
        assert_eq!(
            identifier::<Cheap<char>>().parse(">i|d description\n"),
            Ok("i|d".to_string())
        );
    }

    #[test]
    fn parse_description() {
        assert_eq!(
            description::<Cheap<char>>().parse("description"),
            Ok("description".to_string())
        );
        assert_eq!(
            description::<Cheap<char>>().parse(" multiple word description\n"),
            Ok("multiple word description".to_string())
        );
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            parser().parse(">i|d multiple word description\nA\nC\nG\nT\n\n\n"),
            Ok(vec![Record {
                identifier: "i|d".to_string(),
                description: "multiple word description".to_string(),
                sequence: "ACGT".to_string()
            }])
        );
        assert_eq!(
            parser().parse(
                ">i|d multiple word description\nAC\nGT\n>i|d multiple word description\nA\nC\nG\nT"
            ),
            Ok(vec![
                Record {
                    identifier: "i|d".to_string(),
                    description: "multiple word description".to_string(),
                    sequence: "ACGT".to_string()
                },
                Record {
                    identifier: "i|d".to_string(),
                    description: "multiple word description".to_string(),
                    sequence: "ACGT".to_string()
                }
            ])
        );
        assert_eq!(
            parser().parse(
                ">i|d multiple word description\nA\n>i|d multiple word description\nA\nC\n\n>i|d multiple word description\nA\nC\nG\n\n\n>i|d multiple word description\nA\nC\nG\nT"
            ),
            Ok(vec![
                Record {
                    identifier: "i|d".to_string(),
                    description: "multiple word description".to_string(),
                    sequence: "A".to_string()
                },
                Record {
                    identifier: "i|d".to_string(),
                    description: "multiple word description".to_string(),
                    sequence: "AC".to_string()
                },
                Record {
                    identifier: "i|d".to_string(),
                    description: "multiple word description".to_string(),
                    sequence: "ACG".to_string()
                },
                Record {
                    identifier: "i|d".to_string(),
                    description: "multiple word description".to_string(),
                    sequence: "ACGT".to_string()
                }
            ])
        );
    }
}
