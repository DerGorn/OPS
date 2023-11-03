use std::{io::BufRead, io::BufReader};

use ops::{is_enum_token_variant, ErrorHandler, ErrorTypes, Position, Res, Token};

pub fn tokenizer<R>(source: BufReader<R>, file_name: &str) -> Res<Vec<Token>>
where
    R: std::io::Read,
{
    let mut tokens: Vec<Token> = vec![];

    let mut current_token_value: String = String::from("");
    let mut is_in_string: bool = false;
    let mut is_new_token: bool = true;
    let mut add_token =
        |current_token_value: String, row: usize, col: usize, is_in_string: bool| {
            let current_position = Position::new(file_name.to_string(), row, col);
            tokens.push(Token::new(
                current_token_value,
                current_position,
                is_in_string,
            ));
        };
    let mut string_start_row = 0;
    let mut string_start_col = 0;
    let lines = source.lines();
    for (row, line) in lines.enumerate() {
        let line = line?;
        for (col, c) in line.char_indices() {
            if c.is_whitespace() && !is_in_string {
                if !is_new_token && &current_token_value != "" {
                    add_token(current_token_value, row + 1, col + 1, is_in_string);
                    current_token_value = String::from("");
                    is_new_token = true;
                }
                continue;
            }
            if c == '"' {
                if is_in_string {
                    add_token(current_token_value, row + 1, col + 2, is_in_string);
                    is_in_string = false;
                    current_token_value = String::from("");
                    is_new_token = true;
                } else {
                    is_in_string = true;
                    string_start_row = row + 1;
                    string_start_col = col + 2;
                }
                continue;
            }
            current_token_value.push(c);
            is_new_token = false;
            if !is_in_string {
                if is_enum_token_variant(current_token_value.clone()) {
                    add_token(current_token_value, row + 1, col + 2, is_in_string);
                    current_token_value = String::from("");
                    is_new_token = true;
                } else if is_enum_token_variant(c.to_string()) {
                    current_token_value.pop();
                    add_token(current_token_value, row + 1, col + 1, is_in_string);
                    add_token(c.to_string(), row + 1, col + 2, is_in_string);
                    current_token_value = String::from("");
                    is_new_token = true;
                }
            }
        }
        if !is_in_string && !is_new_token {
            add_token(current_token_value, row + 1, line.len() + 1, is_in_string);
            current_token_value = String::from("");
            is_new_token = true;
        }
    }
    if is_in_string {
        let current_position =
            Position::new(file_name.to_string(), string_start_row, string_start_col);
        ErrorHandler::compiler_err(
            ErrorTypes::UnclosedString,
            String::from("expected '\"', but found end of file"),
            current_position,
        )
    } else {
        Ok(tokens)
    }
}
