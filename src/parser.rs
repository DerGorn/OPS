use std::{any::Any, collections::HashMap, fmt::Debug};

use ops::{
    ControlVariants, ErrorHandler, ErrorTypes, Position, Res, Token, TokenTypes, TokenVariant,
    WordVariants,
};

use crate::tokenizer;

pub struct Variable {
    name: String,
    position: Position,
    var_type: String,
    value: String,
}
impl Variable {
    fn new(var_type: String, name: String, position: Position, value: String) -> Variable {
        Variable {
            name,
            position,
            var_type,
            value,
        }
    }
}
impl Debug for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Variable {{ '{}' at {} with type {} and value '{}' }}",
            self.name, self.position, self.var_type, self.value
        )
    }
}

pub struct Function {
    name: String,
    position: Position,
    parameter: Vec<Variable>,
    expression: String,
}
impl Function {
    fn new(
        name: String,
        position: Position,
        parameter: Vec<Variable>,
        expression: String,
    ) -> Function {
        Function {
            name,
            position,
            parameter,
            expression,
        }
    }
}

pub struct Scope {
    variables: HashMap<String, Variable>,
    functions: HashMap<String, Function>,
}

impl Scope {
    pub fn new() -> Self {
        let inbuild_functions: [(String, Function); 1] = [(
            String::from("print"),
            Function::new(
                String::from("print"),
                Position::new(file!().to_string(), line!() as usize, column!() as usize),
                vec![],
                "print".to_string(),
            ),
        )];
        Self {
            variables: HashMap::new(),
            functions: HashMap::from_iter(inbuild_functions),
        }
    }

    fn create_variable(
        &mut self,
        variable_type: String,
        variable_name: String,
        name_position: Position,
    ) {
        self.variables.insert(
            variable_name.clone(),
            Variable::new(
                variable_type,
                variable_name,
                name_position,
                String::from(""),
            ),
        );
    }

    fn asign_variable(
        &mut self,
        variable_name: Token,
        value_type: String,
        value: String,
    ) -> Res<()> {
        match self.variables.get_mut(&variable_name.token.to_string()) {
            Some(var) => {
                if var.var_type == value_type {
                    var.value = value;
                    Ok(())
                } else {
                    ErrorHandler::compiler_err(
                        ErrorTypes::MismatchedTypes,
                        format!(
                            "expected {}-type but found a {}-type",
                            var.var_type, value_type,
                        ),
                        variable_name.position,
                    )
                }
            }
            None => ErrorHandler::compiler_err(
                ErrorTypes::UnexpectedSymbol,
                format!(
                    "expected a variable name, but found no '{}' in this scope",
                    variable_name.token.to_string()
                ),
                variable_name.position,
            ),
        }
    }

    pub fn parse_block(&mut self, tokens: Vec<Token>) -> Res<String> {
        let mut is_in_block = 0;
        let statements: Vec<&[Token]> = tokens
            .split(|token| match token.token {
                TokenTypes::Control(ControlVariants::OpenCurly) => {
                    is_in_block += 1;
                    false
                }
                TokenTypes::Control(ControlVariants::CloseCurly) => {
                    is_in_block -= 1;
                    if is_in_block < 0 {
                        panic!(
                            "{}",
                            ErrorHandler::compiler_err::<bool>(
                                ErrorTypes::UnexpectedSymbol,
                                String::from("no opened block to close"),
                                token.position.clone()
                            )
                            .unwrap_err()
                        )
                    }
                    false
                }
                TokenTypes::Control(ControlVariants::Semicolon) => is_in_block == 0,
                _ => false,
            })
            .collect();
        if statements
            .last()
            .and_then(|stat| if stat.len() > 0 { Some(true) } else { None })
            .is_some()
        {
            return ErrorHandler::compiler_err(
                ErrorTypes::UnclosedStatement,
                String::from("expected ';', but found end of block"),
                statements.last().unwrap().first().unwrap().position.clone(),
            );
        }
        for stat in statements {
            if stat.len() > 0 {
                self.parse_statement(stat)?;
            }
        }
        println!("Scope: {:?}", self.variables);
        Ok(String::from(""))
    }

    fn parse_statement(&mut self, tokens: &[Token]) -> Res<String> {
        match &tokens.first().unwrap().token {
            TokenTypes::Control(ControlVariants::OpenCurly) => {
                if match tokens.last().unwrap().token {
                    TokenTypes::Control(ControlVariants::CloseCurly) => true,
                    _ => false,
                } {
                    Scope::new().parse_block(
                        tokens
                            .get(1..tokens.len() - 1)
                            .or_else(|| Some(&[]))
                            .unwrap()
                            .to_vec(),
                    )
                } else {
                    println!("{:?}", tokens);
                    ErrorHandler::compiler_err(
                        ErrorTypes::UnclosedBlock,
                        String::from("expected '}', but encountered ';'"),
                        tokens.first().unwrap().position.clone(),
                    )
                }
            }
            TokenTypes::Word(WordVariants::Str)
            | TokenTypes::Word(WordVariants::Bool)
            | TokenTypes::Word(WordVariants::Num) => self.parse_variable_instantiation(tokens),
            TokenTypes::Name(_) => self.parse_name_expression(tokens),
            x if x.is_literal() => self.parse_expression(tokens).and_then(|(vt, v)| Ok(v)),
            _ => ErrorHandler::compiler_err(
                ErrorTypes::UnexpectedSymbol,
                format!(
                    "cannot start statement with '{}'",
                    tokens.first().unwrap().token.to_string()
                ),
                tokens.first().unwrap().position.clone(),
            ),
        }
    }

    fn parse_variable_instantiation(&mut self, tokens: &[Token]) -> Res<String> {
        let mut tokens = tokens.to_vec();
        let variable_type = tokens.remove(0);
        let (variable_name, name_position) = match tokens
            .first()
            .and_then(|tok| Some((tok.token.clone(), tok.position.clone())))
        {
            Some((TokenTypes::Name(n), pos)) => (n.to_string(), pos),
            Some((tok, pos)) => {
                return ErrorHandler::compiler_err(
                    ErrorTypes::InvalidStatement,
                    format!("expected a name but found '{}'", tok.to_string()),
                    pos,
                )
            }
            None => {
                return ErrorHandler::compiler_err(
                    ErrorTypes::InvalidStatement,
                    String::from("expected a name but found nothing"),
                    variable_type.position.clone(),
                )
            }
        };
        match tokens
            .get(1)
            .and_then(|tok| Some((tok.token.clone(), tok.position.clone())))
        {
            Some((TokenTypes::Control(ControlVariants::Asign), _)) => (),
            Some((tok, pos)) => {
                return ErrorHandler::compiler_err(
                    ErrorTypes::InvalidStatement,
                    format!("expected '=' but found '{}'", tok.to_string()),
                    pos,
                )
            }
            None => {
                return ErrorHandler::compiler_err(
                    ErrorTypes::InvalidStatement,
                    String::from("expected '=' but found nothing"),
                    name_position,
                )
            }
        };
        self.create_variable(
            variable_type.token.to_string(),
            variable_name,
            name_position,
        );
        self.parse_variable_asignment(&tokens)
    }

    fn parse_variable_asignment(&mut self, tokens: &[Token]) -> Res<String> {
        let tokens = tokens.to_vec();
        let variable_name = &tokens[0];
        let (value_type, value) = match tokens.get(2..) {
            Some([]) | None => {
                return ErrorHandler::compiler_err(
                    ErrorTypes::InvalidStatement,
                    String::from("expected an expression, but found nothing"),
                    tokens[1].position.clone(),
                )
            }
            Some(toks) => self.parse_expression(toks),
        }?;
        self.asign_variable(variable_name.clone(), value_type, value)?;
        Ok(String::from(""))
    }

    fn parse_name_expression(&mut self, tokens: &[Token]) -> Res<String> {
        let variable_name = &tokens[0];
        match tokens
            .get(1)
            .and_then(|tok| Some((tok.token.clone(), tok.position.clone())))
        {
            Some((TokenTypes::Control(ControlVariants::Asign), _)) => {
                self.parse_variable_asignment(tokens)
            }
            Some((TokenTypes::Control(ControlVariants::OpenParanthesis), _)) => {
                self.parse_function_call(tokens)
            }
            Some((tok, pos)) => ErrorHandler::compiler_err(
                ErrorTypes::InvalidStatement,
                format!("expected '=' but found '{}'", tok.to_string()),
                pos,
            ),
            None => ErrorHandler::compiler_err(
                ErrorTypes::InvalidStatement,
                String::from("expected '=' but found nothing"),
                variable_name.position.clone(),
            ),
        }
    }

    fn parse_function_call(&self, tokens: &[Token]) -> Res<String> {
        let variable_name = &tokens[0];
        let input = match tokens.get(2..tokens.len() - 1) {
            Some(toks) => self.parse_expression(toks),
            None => ErrorHandler::compiler_err(
                ErrorTypes::InvalidStatement,
                String::from("expected function parameters, but found nothing"),
                tokens[1].position.clone(),
            ),
        }?;
        Ok(format!(""))
    }

    fn parse_expression(&self, tokens: &[Token]) -> Res<(String, String)> {
        println!("Parsing Expression: {:?}", tokens);
        let (value_type, value) = match tokens.len() {
            //TODO:
            //MAYBE DO A FUCKING RUNTIME INSTEAD OF A COMPILER
            1 => match tokens[0].token.clone() {
                x if x.is_literal() => Ok((x.token_type().to_owned(), x.to_string())),
                _ => ErrorHandler::compiler_err(
                    ErrorTypes::InvalidStatement,
                    format!(
                        "expected literal but found '{}'",
                        tokens[0].token.to_string()
                    ),
                    tokens[0].position.clone(),
                ),
            },
            x if x & 1 == 1 => {
                let (left_type, left_value) = self.parse_expression(tokens.get(0..1).unwrap())?;
                let operator = &tokens[1];
                let right = tokens.get(2..).unwrap();
                let (right_type, right_value) = self.parse_expression(right)?;
                match operator.token.clone() {
                    TokenTypes::Control(ControlVariants::Comma) => Ok((
                        format!("{}, {}", left_type, right_type),
                        format!("{}, {}", left_value, right_value),
                    )),
                    op => {
                        if right_type != left_type {
                            ErrorHandler::compiler_err(
                                ErrorTypes::MismatchedTypes,
                                format!(
                                    "expected {}-type but found {}-type",
                                    left_type, right_type
                                ),
                                right.last().unwrap().position.clone(),
                            )
                        } else {
                            Ok((
                                left_type,
                                format!("{} {} {}", left_value, op.to_string(), right_value),
                            ))
                        }
                    }
                }
            }
            _ => ErrorHandler::compiler_err(
                ErrorTypes::InvalidStatement,
                format!(
                    "expected expression but found '{}'",
                    tokens
                        .iter()
                        .fold(String::from(""), |mut str: String, tok: &Token| {
                            str.push_str(tok.token.to_string().as_str());
                            str
                        })
                ),
                tokens.last().unwrap().position.clone(),
            ),
        }?;
        Ok((value_type, value))
    }
}
