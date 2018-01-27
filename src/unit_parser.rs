/// A token derived from the user input
#[derive(Debug, PartialEq)]
enum Token {
    ONE
}

/// A node in the parse tree
#[derive(Debug, PartialEq)]
struct TreeNode<T> {
    data: T,
}

/// A parse tree, assembled from a stream of Tokens
#[derive(Debug, PartialEq)]
enum ParseTree<T> {
    Empty,
    Node(TreeNode<T>),
}

/// Parses a stream of tokens into a parse tree
fn parse(token_stream: &[Token]) -> ParseTree<i32> {
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
