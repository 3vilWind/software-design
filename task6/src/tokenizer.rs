use std::fmt::Debug;
use anyhow::bail;

#[derive(Debug, PartialEq, Clone)]
pub struct NumberToken(pub i32);

#[derive(Debug, PartialEq, Clone)]
pub enum Brace {
    Open,
    Close,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operation {
    Plus,
    Minus,
    Star,
    Slash,
}

pub type VisitResult = anyhow::Result<()>;

pub trait Token: Debug {
    fn accept(&self, visitor: &mut dyn TokenVisitor) -> VisitResult;
}

pub trait TokenVisitor {
    fn visit_number(&mut self, token: &NumberToken) -> VisitResult;
    fn visit_brace(&mut self, token: &Brace) -> VisitResult;
    fn visit_operation(&mut self, token: &Operation) -> VisitResult;
}

impl Token for NumberToken {
    fn accept(self: &NumberToken, visitor: &mut dyn TokenVisitor) -> VisitResult {
        visitor.visit_number(self)
    }
}

impl Token for Brace {
    fn accept(&self, visitor: &mut dyn TokenVisitor) -> VisitResult {
        visitor.visit_brace(self)
    }
}

impl Token for Operation {
    fn accept(&self, visitor: &mut dyn TokenVisitor) -> VisitResult {
        visitor.visit_operation(self)
    }
}

trait State {
    fn try_consume_char(self: Box<Self>, char: CharData) -> anyhow::Result<(Box<dyn State>, bool, Option<Box<dyn Token>>)>;
}

struct StartState;

struct NumberState(String);

#[derive(Clone, Copy)]
enum CharData {
    Char(char),
    Eof,
}

impl State for StartState {
    fn try_consume_char(self: Box<Self>, char: CharData) -> anyhow::Result<(Box<dyn State>, bool, Option<Box<dyn Token>>)> {
        match char {
            CharData::Char(char) => {
                let mut consumed: bool = true;
                let mut state: Box<dyn State> = self;

                let token: Option<Box<dyn Token>> = if char == '+' {
                    Some(Box::new(Operation::Plus))
                } else if char == '-' {
                    Some(Box::new(Operation::Minus))
                } else if char == '*' {
                    Some(Box::new(Operation::Star))
                } else if char == '/' {
                    Some(Box::new(Operation::Slash))
                } else if char == '(' {
                    Some(Box::new(Brace::Open))
                } else if char == ')' {
                    Some(Box::new(Brace::Close))
                } else if char.is_digit(10) {
                    state = Box::new(NumberState("".to_owned()));
                    consumed = false;
                    None
                } else if char.is_whitespace() {
                    None
                } else {
                    bail!("Got unexpected char {}", char)
                };
                Ok((state, consumed, token))
            }
            CharData::Eof => Ok((self, true, None))
        }
    }
}

impl State for NumberState {
    fn try_consume_char(mut self: Box<Self>, char: CharData) -> anyhow::Result<(Box<dyn State>, bool, Option<Box<dyn Token>>)> {
        match char {
            CharData::Char(char) => {
                if char.is_digit(10) {
                    self.0.push(char);
                    Ok((self, true, None))
                } else {
                    Ok((Box::new(StartState), false, Some(Box::new(NumberToken(self.0.parse::<i32>().unwrap())))))
                }
            }
            CharData::Eof => {
                let token: Box<dyn Token> = Box::new(NumberToken(self.0.parse::<i32>().unwrap()));
                Ok((self, true, Some(token)))
            }
        }
    }
}

pub fn tokenize(data: &str) -> anyhow::Result<Vec<Box<dyn Token>>> {
    let mut result = Vec::<Box<dyn Token>>::new();
    let mut state: Box<dyn State> = Box::new(StartState);
    for char in data
        .chars()
        .into_iter()
        .map(|x| CharData::Char(x))
        .chain(vec![CharData::Eof].into_iter()) {
        loop {
            let (new_state, consumed, token) = state.try_consume_char(char)?;
            state = new_state;
            if let Some(token) = token {
                result.push(token);
            }
            if consumed {
                break;
            }
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    impl PartialEq for dyn Token {
        fn eq(&self, other: &Self) -> bool {
            format!("{:?}", self) == format!("{:?}", other)
        }
    }

    #[test]
    fn simple_plus() {
        let result = tokenize("5 + 5");
        let expected: Vec<Box<dyn Token>> = vec![
            Box::new(NumberToken(5)),
            Box::new(Operation::Plus),
            Box::new(NumberToken(5)),
        ];
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn plus_multiply_with_braces() {
        let result = tokenize("(2 + 2) * 2");
        let expected: Vec<Box<dyn Token>> = vec![
            Box::new(Brace::Open),
            Box::new(NumberToken(2)),
            Box::new(Operation::Plus),
            Box::new(NumberToken(2)),
            Box::new(Brace::Close),
            Box::new(Operation::Star),
            Box::new(NumberToken(2)),
        ];
        assert_eq!(result.unwrap(), expected);
    }
}
