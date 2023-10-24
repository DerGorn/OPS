use std::{fmt::Display, io::BufRead, io::BufReader};

use crate::Res;

pub struct Token {
    pub token: Box<dyn TokenVariant>,
    pub position: Position,
}

pub trait TokenVariant {
    fn new(value: &str) -> Option<Self>
    where
        Self: Sized;

    fn to_string(&self) -> String;

    fn token_type(&self) -> &str;
}

//CONTROL   ////////////////////////////////////
enum ControlTokenVariants {
    ControlComma,            // ","
    ControlSemicolon,        // ";"
    ControlOpenParanthesis,  // "("
    ControlCloseParanthesis, // ")"
    ControlOpenCurly,        // "{"
    ControlCloseCurly,       // "}"
    ControlOpenSquare,       // "["
    ControlCloseSquare,      // "]"
    ControlColon,            // ":"
    ControlQuestionmark,     // "?"
    ControlLess,             // "<"
    ControlLessEqual,        // "<="
    ControlGreater,          // ">"
    ControlGreaterEqual,     // ">="
    ControlAsign,            // "="
    ControlEqual,            // "=="
    ControlOr,               // "||"
    ControlAnd,              // "&&"
    ControlBitOr,            // "|"
    ControlBitAnd,           // "&"
    ControlPlus,             // "+"
    ControlMinus,            // "-"
    ControlMultiplikation,   // "*"
    ControlPower,            // "**"
    ControlDivide,           // "/"
    ControlNot,              // "!"
    ControlNotEqual,         // "!="
}
impl TokenVariant for ControlTokenVariants {
    fn new(value: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match value {
            "," => Some(Self::ControlComma),
            ";" => Some(Self::ControlSemicolon),
            "(" => Some(Self::ControlOpenParanthesis),
            ")" => Some(Self::ControlCloseParanthesis),
            "{" => Some(Self::ControlOpenCurly),
            "}" => Some(Self::ControlCloseCurly),
            "[" => Some(Self::ControlOpenSquare),
            "]" => Some(Self::ControlCloseSquare),
            ":" => Some(Self::ControlColon),
            "?" => Some(Self::ControlQuestionmark),
            "<" => Some(Self::ControlLess),
            "<=" => Some(Self::ControlLessEqual),
            ">" => Some(Self::ControlGreater),
            ">=" => Some(Self::ControlGreaterEqual),
            "=" => Some(Self::ControlAsign),
            "==" => Some(Self::ControlEqual),
            "||" => Some(Self::ControlOr),
            "&&" => Some(Self::ControlAnd),
            "|" => Some(Self::ControlBitOr),
            "&" => Some(Self::ControlBitAnd),
            "+" => Some(Self::ControlPlus),
            "-" => Some(Self::ControlMinus),
            "*" => Some(Self::ControlMultiplikation),
            "**" => Some(Self::ControlPower),
            "/" => Some(Self::ControlDivide),
            "!" => Some(Self::ControlNot),
            "!=" => Some(Self::ControlNotEqual),
            _ => None,
        }
    }

    fn to_string(&self) -> String {
        match self {
            Self::ControlComma => String::from(","),
            Self::ControlSemicolon => String::from(";"),
            Self::ControlOpenParanthesis => String::from("("),
            Self::ControlCloseParanthesis => String::from(")"),
            Self::ControlOpenCurly => String::from("{"),
            Self::ControlCloseCurly => String::from("}"),
            Self::ControlOpenSquare => String::from("["),
            Self::ControlCloseSquare => String::from("]"),
            Self::ControlColon => String::from(":"),
            Self::ControlQuestionmark => String::from("?"),
            Self::ControlLess => String::from("<"),
            Self::ControlLessEqual => String::from("<="),
            Self::ControlGreater => String::from(">"),
            Self::ControlGreaterEqual => String::from(">="),
            Self::ControlAsign => String::from("="),
            Self::ControlEqual => String::from("=="),
            Self::ControlOr => String::from("||"),
            Self::ControlAnd => String::from("&&"),
            Self::ControlBitOr => String::from("|"),
            Self::ControlBitAnd => String::from("&"),
            Self::ControlPlus => String::from("+"),
            Self::ControlMinus => String::from("-"),
            Self::ControlMultiplikation => String::from("*"),
            Self::ControlPower => String::from("**"),
            Self::ControlDivide => String::from("/"),
            Self::ControlNot => String::from("!"),
            Self::ControlNotEqual => String::from("!="),
            _ => panic!("Forgor to implement ControlTokenVariant"),
        }
    }

    fn token_type(&self) -> &str {
        "Control"
    }
}
////////////////////////////////////////////////

//Word   ////////////////////////////////////
enum WordTokenVariants {
    WordFunc,     // "func"
    WordNum,      // "num"
    WordStr,      // "str"
    WordBool,     // "bool"
    WordArgs,     // "args"
    WordReturn,   // "return"
    WordConst,    // "const"
    WordIf,       // "if"
    WordElse,     // "else"
    WordWhile,    // "while"
    WordFor,      // "for"
    WordContinue, // "continue"
    WordBreak,    // "break"
    WordMatch,    // "match"
    WordDef,      // "def"
    WordPrint,    // "print"
}
impl TokenVariant for WordTokenVariants {
    fn new(value: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match value {
            "func" => Some(Self::WordFunc),
            "num" => Some(Self::WordNum),
            "str" => Some(Self::WordStr),
            "bool" => Some(Self::WordBool),
            "args" => Some(Self::WordArgs),
            "return" => Some(Self::WordReturn),
            "const" => Some(Self::WordConst),
            "if" => Some(Self::WordIf),
            "else" => Some(Self::WordElse),
            "while" => Some(Self::WordWhile),
            "for" => Some(Self::WordFor),
            "continue" => Some(Self::WordContinue),
            "break" => Some(Self::WordBreak),
            "match" => Some(Self::WordMatch),
            "def" => Some(Self::WordDef),
            "print" => Some(Self::WordPrint),
            _ => None,
        }
    }

    fn to_string(&self) -> String {
        match self {
            Self::WordFunc => String::from("func"),
            Self::WordNum => String::from("num"),
            Self::WordStr => String::from("str"),
            Self::WordBool => String::from("bool"),
            Self::WordArgs => String::from("args"),
            Self::WordReturn => String::from("return"),
            Self::WordConst => String::from("const"),
            Self::WordIf => String::from("if"),
            Self::WordElse => String::from("else"),
            Self::WordWhile => String::from("while"),
            Self::WordFor => String::from("for"),
            Self::WordContinue => String::from("continue"),
            Self::WordBreak => String::from("break"),
            Self::WordMatch => String::from("match"),
            Self::WordDef => String::from("def"),
            Self::WordPrint => String::from("print"),
            _ => panic!("Forgor to implement WordTokenVariants"),
        }
    }

    fn token_type(&self) -> &str {
        "Word"
    }
}
////////////////////////////////////////////////

//Name   ////////////////////////////////////
struct NameTokenVariants {
    name: String,
}
impl TokenVariant for NameTokenVariants {
    fn new(value: &str) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Self {
            name: value.to_string(),
        })
    }

    fn to_string(&self) -> String {
        self.name.clone()
    }

    fn token_type(&self) -> &str {
        "Name"
    }
}
////////////////////////////////////////////////

//String   ////////////////////////////////////
struct StringTokenVariants {
    str: String,
}
impl TokenVariant for StringTokenVariants {
    fn new(value: &str) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Self {
            str: format!("\"{}\"", value),
        })
    }

    fn to_string(&self) -> String {
        self.str.clone()
    }

    fn token_type(&self) -> &str {
        "String"
    }
}
////////////////////////////////////////////////

//Boolean   ////////////////////////////////////
struct BooleanTokenVariants {
    bool: bool,
}
impl TokenVariant for BooleanTokenVariants {
    fn new(value: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match value {
            "true" => Some(true),
            "false" => Some(false),
            _ => None,
        }
        .and_then(|b| Some(Self { bool: b }))
    }

    fn to_string(&self) -> String {
        String::from(if self.bool { "true" } else { "false" })
    }

    fn token_type(&self) -> &str {
        "Boolean"
    }
}
////////////////////////////////////////////////

//Number   ////////////////////////////////////
struct NumberTokenVariants {
    num: f64,
    str: String,
}
impl TokenVariant for NumberTokenVariants {
    fn new(value: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match value.parse() {
            Ok(num) => Some(Self {
                num,
                str: String::from(value),
            }),
            Err(_) => None,
        }
    }

    fn to_string(&self) -> String {
        self.str.clone()
    }

    fn token_type(&self) -> &str {
        "Number"
    }
}
////////////////////////////////////////////////

pub struct Position {
    row: usize,
    column: usize,
    file: String,
}
impl Position {
    fn new(file: String, row: usize, column: usize) -> Position {
        Position { file, row, column }
    }
}
impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.row, self.column)
    }
}

fn create_token(token_value: String, pos: Position, is_in_string: bool) -> Token {
    let token_variant: Box<dyn TokenVariant> = match ControlTokenVariants::new(&token_value) {
        Some(t) => Box::new(t),
        None => match WordTokenVariants::new(&token_value) {
            Some(t) => Box::new(t),
            None => {
                if is_in_string {
                    match StringTokenVariants::new(&token_value) {
                        Some(t) => Box::new(t),
                        None => panic!(
                            "Somehow the StringTokenVariant failed at '{}'",
                            &token_value
                        ),
                    }
                } else {
                    match BooleanTokenVariants::new(&token_value) {
                        Some(t) => Box::new(t),
                        None => match NumberTokenVariants::new(&token_value) {
                            Some(t) => Box::new(t),
                            None => match NameTokenVariants::new(&token_value) {
                                Some(t) => Box::new(t),
                                None => panic!(
                                    "Somehow the NameTokenVariant failed at '{}'",
                                    &token_value
                                ),
                            },
                        },
                    }
                }
            }
        },
    };
    Token {
        token: token_variant,
        position: pos,
    }
}

fn is_enum_token_variant(token_value: String) -> bool {
    ControlTokenVariants::new(&token_value).is_some()
        || WordTokenVariants::new(&token_value).is_some()
}

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
            tokens.push(create_token(
                current_token_value,
                current_position,
                is_in_string,
            ));
        };
    for (row, line) in source.lines().enumerate() {
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
    Ok(tokens)
}
