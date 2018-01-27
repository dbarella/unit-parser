//! This module is designed to convert a token stream into a parse tree.
//!
//! Say that we have the following expression:
//!
//! 2 kg * 4 m / 8 s^2
//!
//! We would like to reduce this expression to:
//!
//! 1 kg * m / s^2
//!
//! Or, even better:
//!
//! 1 N

extern crate num;

/// Delimeter tokens, e.g:
/// (1 + 2) s
/// ^     ^-------- RIGHT_PAREN
/// |
/// ---------- LEFT_PAREN
#[derive(Debug, PartialEq)]
enum Delimeter {
    LEFT_PAREN,
    RIGHT_PAREN,
}

/// Operator tokens, e.g:
/// 1 s + 2 s
///     ^----- PLUS token
#[derive(Debug, PartialEq)]
enum Operator {
    PLUS,
    MINUS,
    MUL,
    DIV,
    POW
}

/// Unit tokens, e.g:
/// 1 s
///   ^----- SECONDS token
#[derive(Debug, PartialEq)]
enum Unit {
    KILOGRAMS,
    METERS,
    SECONDS,
    NEWTONS,
}

/// Data for the parse tree.
#[derive(Debug, PartialEq)]
struct NumericValue {
    number: num::BigInt,
    unit: Unit,
}

/// A token derived from the user input.
#[derive(Debug, PartialEq)]
enum Token {
    Delimeter(Delimeter),
    Number(num::BigInt),
    Operator(Operator),
    Unit(Unit),
}

/// A parse tree, assembled from a stream of Tokens.
#[derive(Debug, PartialEq)]
enum ParseTree<T> {
    Node(T),
    Empty,
}

/// Parses a stream of tokens into a parse tree.
fn parse(token_stream: &[Token]) -> ParseTree<Token> {
    ParseTree::Empty
}

#[cfg(test)]
mod tests {
    use unit_parser::{parse, ParseTree};

    #[test]
    fn parse_empty_stream_does_nothing() {
        assert_eq!(parse(&[]), ParseTree::Empty);
    }
}
