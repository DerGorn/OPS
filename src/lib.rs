use std::{
    any::Any,
    error::Error,
    fmt::{Debug, Display},
};

////////////////////////////////////////////////////////////////
//                                                           ///
//                      Position                             ///
//                                                           ///
////////////////////////////////////////////////////////////////
#[derive(Debug, Clone)]
pub struct Position {
    row: usize,
    column: usize,
    file: String,
}
impl Position {
    pub fn new(file: String, row: usize, column: usize) -> Position {
        Position { file, row, column }
    }
}
impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.row, self.column)
    }
}

////////////////////////////////////////////////////////////////
//                                                           ///
//                     Erro Stuff                            ///
//                                                           ///
////////////////////////////////////////////////////////////////
pub type Res<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub enum ErrorTypes {
    UnclosedBlock,
    UnclosedString,
    UnclosedStatement,
    InvalidStatement,
    UnexpectedSymbol,
    MismatchedTypes,
}
impl Display for ErrorTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::UnclosedString => "Unclosed String",
            Self::UnclosedBlock => "Unclosed Block",
            Self::UnclosedStatement => "Unclosed Statement",
            Self::InvalidStatement => "Invalid Statement",
            Self::UnexpectedSymbol => "Unexpected Symbol",
            Self::MismatchedTypes => "Mismatched Types",
            _ => panic!("Forgor to implement ErrorType"),
        };
        write!(f, "{}", message)
    }
}
pub struct CompilerError {
    message: String,
    position: Position,
    error_type: ErrorTypes,
}
impl CompilerError {
    pub fn new(error_type: ErrorTypes, message: String, position: Position) -> CompilerError {
        CompilerError {
            message,
            position,
            error_type,
        }
    }
}
impl Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {} at {}",
            self.error_type, self.message, self.position
        )
    }
}
impl Debug for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {} at {}",
            self.error_type, self.message, self.position
        )
    }
}
impl Error for CompilerError {}

pub struct ErrorHandler {
    exec_file: String,
}

impl ErrorHandler {
    pub fn new(exec_file: String) -> ErrorHandler {
        ErrorHandler { exec_file }
    }

    pub fn compiler_err<T>(error_type: ErrorTypes, message: String, position: Position) -> Res<T> {
        Err(Box::new(CompilerError::new(error_type, message, position)))
    }

    pub fn err<T>(message: &str) -> Res<T> {
        Err(message.into())
    }

    pub fn helpful_err<T>(&self, message: &str) -> Res<T> {
        Err(format!("{} For help use `{} -h`", message, self.exec_file).into())
    }
}

////////////////////////////////////////////////////////////////
//                                                           ///
//                     Token Stuff                           ///
//                                                           ///
////////////////////////////////////////////////////////////////
pub trait AToAny: 'static {
    fn as_any(&self) -> &dyn Any;
}
impl<T: 'static> AToAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait TokenVariant: AToAny {
    fn new(value: &str) -> Option<Self>
    where
        Self: Sized;

    fn to_string(&self) -> String;

    fn token_type(&self) -> &str;
}
//CONTROL   ////////////////////////////////////
#[derive(Clone)]
pub enum ControlVariants {
    Comma,            // ","
    Semicolon,        // ";"
    OpenParanthesis,  // "("
    CloseParanthesis, // ")"
    OpenCurly,        // "{"
    CloseCurly,       // "}"
    OpenSquare,       // "["
    CloseSquare,      // "]"
    Colon,            // ":"
    Questionmark,     // "?"
    Less,             // "<"
    LessEqual,        // "<="
    Greater,          // ">"
    GreaterEqual,     // ">="
    Asign,            // "="
    Equal,            // "=="
    Or,               // "||"
    And,              // "&&"
    BitOr,            // "|"
    BitAnd,           // "&"
    Plus,             // "+"
    Minus,            // "-"
    Multiplikation,   // "*"
    Power,            // "**"
    Divide,           // "/"
    Not,              // "!"
    NotEqual,         // "!="
}
impl TokenVariant for ControlVariants {
    fn new(value: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match value {
            "," => Some(Self::Comma),
            ";" => Some(Self::Semicolon),
            "(" => Some(Self::OpenParanthesis),
            ")" => Some(Self::CloseParanthesis),
            "{" => Some(Self::OpenCurly),
            "}" => Some(Self::CloseCurly),
            "[" => Some(Self::OpenSquare),
            "]" => Some(Self::CloseSquare),
            ":" => Some(Self::Colon),
            "?" => Some(Self::Questionmark),
            "<" => Some(Self::Less),
            "<=" => Some(Self::LessEqual),
            ">" => Some(Self::Greater),
            ">=" => Some(Self::GreaterEqual),
            "=" => Some(Self::Asign),
            "==" => Some(Self::Equal),
            "||" => Some(Self::Or),
            "&&" => Some(Self::And),
            "|" => Some(Self::BitOr),
            "&" => Some(Self::BitAnd),
            "+" => Some(Self::Plus),
            "-" => Some(Self::Minus),
            "*" => Some(Self::Multiplikation),
            "**" => Some(Self::Power),
            "/" => Some(Self::Divide),
            "!" => Some(Self::Not),
            "!=" => Some(Self::NotEqual),
            _ => None,
        }
    }

    fn to_string(&self) -> String {
        match self {
            Self::Comma => String::from(","),
            Self::Semicolon => String::from(";"),
            Self::OpenParanthesis => String::from("("),
            Self::CloseParanthesis => String::from(")"),
            Self::OpenCurly => String::from("{"),
            Self::CloseCurly => String::from("}"),
            Self::OpenSquare => String::from("["),
            Self::CloseSquare => String::from("]"),
            Self::Colon => String::from(":"),
            Self::Questionmark => String::from("?"),
            Self::Less => String::from("<"),
            Self::LessEqual => String::from("<="),
            Self::Greater => String::from(">"),
            Self::GreaterEqual => String::from(">="),
            Self::Asign => String::from("="),
            Self::Equal => String::from("=="),
            Self::Or => String::from("||"),
            Self::And => String::from("&&"),
            Self::BitOr => String::from("|"),
            Self::BitAnd => String::from("&"),
            Self::Plus => String::from("+"),
            Self::Minus => String::from("-"),
            Self::Multiplikation => String::from("*"),
            Self::Power => String::from("**"),
            Self::Divide => String::from("/"),
            Self::Not => String::from("!"),
            Self::NotEqual => String::from("!="),
            _ => panic!("Forgor to implement ControlTokenVariant"),
        }
    }

    fn token_type(&self) -> &str {
        "Control"
    }
}
////////////////////////////////////////////////
//Word   ////////////////////////////////////
#[derive(Clone)]
pub enum WordVariants {
    Func,     // "func"
    Num,      // "num"
    Str,      // "str"
    Bool,     // "bool"
    Args,     // "args"
    Return,   // "return"
    Const,    // "const"
    If,       // "if"
    Else,     // "else"
    While,    // "while"
    For,      // "for"
    Continue, // "continue"
    Break,    // "break"
    Match,    // "match"
    Def,      // "def"
}
impl TokenVariant for WordVariants {
    fn new(value: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match value {
            "func" => Some(Self::Func),
            "num" => Some(Self::Num),
            "str" => Some(Self::Str),
            "bool" => Some(Self::Bool),
            "args" => Some(Self::Args),
            "return" => Some(Self::Return),
            "const" => Some(Self::Const),
            "if" => Some(Self::If),
            "else" => Some(Self::Else),
            "while" => Some(Self::While),
            "for" => Some(Self::For),
            "continue" => Some(Self::Continue),
            "break" => Some(Self::Break),
            "match" => Some(Self::Match),
            "def" => Some(Self::Def),
            _ => None,
        }
    }

    fn to_string(&self) -> String {
        match self {
            Self::Func => String::from("func"),
            Self::Num => String::from("num"),
            Self::Str => String::from("str"),
            Self::Bool => String::from("bool"),
            Self::Args => String::from("args"),
            Self::Return => String::from("return"),
            Self::Const => String::from("const"),
            Self::If => String::from("if"),
            Self::Else => String::from("else"),
            Self::While => String::from("while"),
            Self::For => String::from("for"),
            Self::Continue => String::from("continue"),
            Self::Break => String::from("break"),
            Self::Match => String::from("match"),
            Self::Def => String::from("def"),
            _ => panic!("Forgor to implement WordTokenVariants"),
        }
    }

    fn token_type(&self) -> &str {
        "Word"
    }
}
////////////////////////////////////////////////
//Name   ////////////////////////////////////
#[derive(Clone)]
pub struct NameVariants {
    name: String,
}
impl TokenVariant for NameVariants {
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
#[derive(Clone)]
pub struct StringVariants {
    str: String,
}
impl TokenVariant for StringVariants {
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
        "str"
    }
}
////////////////////////////////////////////////
//Boolean   ////////////////////////////////////
#[derive(Clone)]
pub struct BooleanVariants {
    bool: bool,
}
impl TokenVariant for BooleanVariants {
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
        "bool"
    }
}
////////////////////////////////////////////////
//Number   ////////////////////////////////////
#[derive(Clone)]
pub struct NumberVariants {
    num: f64,
    str: String,
}
impl TokenVariant for NumberVariants {
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
        "num"
    }
}
////////////////////////////////////////////////

pub fn is_enum_token_variant(token_value: String) -> bool {
    ControlVariants::new(&token_value).is_some() || WordVariants::new(&token_value).is_some()
}

#[derive(Clone)]
pub enum TokenTypes {
    Control(ControlVariants),
    Word(WordVariants),
    Name(NameVariants),
    String(StringVariants),
    Number(NumberVariants),
    Boolean(BooleanVariants),
}
impl TokenTypes {
    fn new(token_value: &str, is_in_string: bool) -> Self {
        match ControlVariants::new(&token_value) {
            Some(t) => Self::Control(t),
            None => match WordVariants::new(&token_value) {
                Some(t) => Self::Word(t),
                None => {
                    if is_in_string {
                        match StringVariants::new(&token_value) {
                            Some(t) => Self::String(t),
                            None => panic!(
                                "Somehow the StringTokenVariant failed at '{}'",
                                &token_value
                            ),
                        }
                    } else {
                        match BooleanVariants::new(&token_value) {
                            Some(t) => Self::Boolean(t),
                            None => match NumberVariants::new(&token_value) {
                                Some(t) => Self::Number(t),
                                None => match NameVariants::new(&token_value) {
                                    Some(t) => Self::Name(t),
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
        }
    }

    fn as_token_variant(&self) -> &dyn TokenVariant {
        match self {
            Self::Control(c) => c,
            Self::Word(w) => w,
            Self::Name(na) => na,
            Self::String(s) => s,
            Self::Number(nu) => nu,
            Self::Boolean(b) => b,
        }
    }

    pub fn is_literal(&self) -> bool {
        match self {
            TokenTypes::Number(_) => true,
            TokenTypes::Boolean(_) => true,
            TokenTypes::String(_) => true,
            _ => false,
        }
    }

    pub fn to_string(&self) -> String {
        self.as_token_variant().to_string()
    }

    pub fn token_type(&self) -> &str {
        self.as_token_variant().token_type()
    }
}

#[derive(Clone)]
pub struct Token {
    pub token: TokenTypes,
    pub position: Position,
}
impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ token: '{}', position: '{}' }}",
            self.token.to_string(),
            self.position
        )
    }
}
impl Token {
    pub fn new(token_value: String, pos: Position, is_in_string: bool) -> Token {
        Token {
            token: TokenTypes::new(&token_value, is_in_string),
            position: pos,
        }
    }
}
