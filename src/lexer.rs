pub fn tokenize(args: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();

    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut escaped = false;

    for c in args.chars() {
        if escaped {
            current_token.push(c);
            escaped = false;
            continue;
        }

        match c {
            '\\' if !in_single_quote => {
                escaped = true;
            }
            '\'' if !in_double_quote => {
                in_single_quote = !in_single_quote;
            }
            '"' if !in_single_quote => {
                in_double_quote = !in_double_quote;
            }
            ' ' | '\t' | '\n' if !in_single_quote && !in_double_quote => {
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
            }
            _ => {
                current_token.push(c);
            }
        }
    }

    if !current_token.is_empty() {
        tokens.push(current_token);
    }
    tokens
}
