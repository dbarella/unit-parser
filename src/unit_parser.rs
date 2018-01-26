fn parse() -> i64 {
    42
}

#[cfg(test)]
mod tests {
    use unit_parser;

    #[test]
    fn parse_does_nothing() {
        assert_eq!(unit_parser::parse(), 42);
    }
}
