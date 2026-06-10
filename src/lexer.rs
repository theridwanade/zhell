pub fn tokenize(args: &str) -> Vec<&str> {
    // A simple example: splitting by whitespace
    args.split_whitespace().collect()
}