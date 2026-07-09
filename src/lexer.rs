use std::mem::take;

use crate::utils::fetch_var;

pub fn tokenize(args: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut var_string = String::new();

    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut escaped = false;
    let mut is_var = false;

    let flush_variable = |var_buf: &mut String, token_buf: &mut String| {
        if !var_buf.is_empty() {
            let var_name = take(var_buf);
            let var_value = fetch_var(&var_name);
            token_buf.push_str(&var_value)
        }
    };

    for c in args.chars() {
        if escaped {
            current_token.push(c);
            escaped = false;
            continue;
        }
        if is_var && !in_single_quote {
            if c.is_alphabetic() || c == '_' {
                var_string.push(c);
                continue;
            } else {
                flush_variable(&mut var_string, &mut current_token);
                is_var = false;
            }
        }
        match c {
            '\\' if !in_single_quote => {
                escaped = true;
            }
            '\'' if !in_double_quote => {
                in_single_quote = !in_single_quote;
            }
            '$' if !in_single_quote => {
                is_var = true;
            }
            '"' if !in_single_quote => {
                in_double_quote = !in_double_quote;
            }
            ' ' | '\t' | '\n' if !in_single_quote && !in_double_quote => {
                if !current_token.is_empty() {
                    tokens.push(take(&mut current_token));
                }
            }
            _ => {
                current_token.push(c);
            }
        }
    }

    if is_var {
        flush_variable(&mut var_string, &mut current_token);
    }
    if !current_token.is_empty() {
        tokens.push(current_token);
    }
    tokens
}
