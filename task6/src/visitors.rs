use std::fmt::Display;

use anyhow::bail;

use crate::tokenizer::{Brace, NumberToken, Operation, Token, TokenVisitor, VisitResult};

enum OperationOrOpenBrace {
    Operation(Operation),
    OpenBrace,
}

impl Operation {
    fn priority(&self) -> u8 {
        match self {
            Operation::Plus => 1,
            Operation::Minus => 1,
            Operation::Star => 2,
            Operation::Slash => 2,
        }
    }
}

pub struct ParserVisitor {
    operator_stack: Vec<OperationOrOpenBrace>,
    result: Vec<Box<dyn Token>>,
}

impl TokenVisitor for ParserVisitor {
    fn visit_number(&mut self, token: &NumberToken) -> VisitResult {
        self.result.push(Box::new(token.clone()));
        Ok(())
    }

    fn visit_brace(&mut self, token: &Brace) -> VisitResult {
        match token {
            Brace::Open => {
                self.operator_stack.push(OperationOrOpenBrace::OpenBrace);
            }
            Brace::Close => {
                while self.operator_stack.last()
                    .and_then(|x|
                        if matches!(x, OperationOrOpenBrace::OpenBrace) { None } else { Some(true) }).is_some() {
                    self.pop_operation_unwrap();
                }
                if self.operator_stack.is_empty() {
                    bail!("Got unexpected close brace");
                }
                self.operator_stack.pop();
            }
        }
        Ok(())
    }

    fn visit_operation(&mut self, token: &Operation) -> VisitResult {
        while self.operator_stack.last()
            .and_then(|x|
                if let OperationOrOpenBrace::Operation(op) = x {
                    if token.priority() <= op.priority() {
                        Some(true)
                    } else {
                        None
                    }
                } else {
                    None
                }).is_some() {
            self.pop_operation_unwrap();
        }
        self.operator_stack.push(OperationOrOpenBrace::Operation(token.clone()));
        Ok(())
    }
}

impl ParserVisitor {
    fn pop_operation_unwrap(&mut self) {
        if let OperationOrOpenBrace::Operation(op) = self.operator_stack.pop().unwrap() {
            self.result.push(Box::new(op));
        } else {
            panic!("impossible condition")
        }
    }

    /// Convert tokens in infix notation to postfix.
    ///
    /// Assume correctness of input, but can catch a few issues with braces.
    pub fn infix_to_postfix(tokens: &Vec<Box<dyn Token>>) -> anyhow::Result<Vec<Box<dyn Token>>> {
        let mut visitor = ParserVisitor { operator_stack: Vec::new(), result: Vec::new() };
        let token_visitor: &mut ParserVisitor = &mut visitor;
        for token in tokens {
            token.accept(token_visitor)?
        }
        while let Some(op) = visitor.operator_stack.pop() {
            match op {
                OperationOrOpenBrace::OpenBrace => bail!("Got unexpected open brace"),
                OperationOrOpenBrace::Operation(op) => visitor.result.push(Box::new(op)),
            }
        }
        Ok(visitor.result)
    }
}

pub struct PrintVisitor {
    output: String,
    is_last: bool,
}

impl TokenVisitor for PrintVisitor {
    fn visit_number(&mut self, token: &NumberToken) -> VisitResult {
        self.write_data(token.0)
    }

    fn visit_brace(&mut self, token: &Brace) -> VisitResult {
        self.output += &format!("{}", match token {
            Brace::Open => '(',
            Brace::Close => ')',
        });

        Ok(())
    }

    fn visit_operation(&mut self, token: &Operation) -> VisitResult {
        self.write_data(PrintVisitor::operation_to_string(token))
    }
}

impl PrintVisitor {
    fn write_data<T: Display>(&mut self, data: T) -> VisitResult {
        if self.is_last {
            self.output += &format!("{}", data);
        } else {
            self.output += &format!("{} ", data);
        }

        Ok(())
    }

    fn operation_to_string(token: &Operation) -> String {
        match token {
            Operation::Plus => '+',
            Operation::Minus => '-',
            Operation::Star => '*',
            Operation::Slash => '/',
        }.to_string()
    }

    pub fn write(tokens: &Vec<Box<dyn Token>>) -> anyhow::Result<String> {
        let mut visitor = PrintVisitor { output: String::new(), is_last: false };
        for (i, token) in tokens.iter().enumerate() {
            if i == tokens.len() - 1 {
                visitor.is_last = true;
            }
            let token_visitor: &mut PrintVisitor = &mut visitor;
            token.accept(token_visitor)?
        }
        Ok(visitor.output)
    }
}

pub struct CalcVisitor {
    stack: Vec<i32>,
}

impl TokenVisitor for CalcVisitor {
    fn visit_number(&mut self, token: &NumberToken) -> VisitResult {
        self.stack.push(token.0);
        Ok(())
    }

    fn visit_brace(&mut self, _: &Brace) -> VisitResult {
        bail!("Got brace in postfix notation");
    }

    fn visit_operation(&mut self, token: &Operation) -> VisitResult {
        let right = self.stack.pop();
        let left = self.stack.pop();
        if let (Some(left), Some(right)) = (left, right) {
            self.stack.push(match token {
                Operation::Plus => left + right,
                Operation::Minus => left - right,
                Operation::Star => left * right,
                Operation::Slash => left / right,
            });
        } else {
            bail!("Not enough operands for {}", PrintVisitor::operation_to_string(token))
        }
        Ok(())
    }
}

impl CalcVisitor {
    pub fn evaluate_postfix(tokens: &Vec<Box<dyn Token>>) -> anyhow::Result<i32> {
        let mut visitor = CalcVisitor { stack: Vec::new() };
        let token_visitor: &mut CalcVisitor = &mut visitor;
        for token in tokens {
            token.accept(token_visitor)?
        }
        if visitor.stack.len() != 1 {
            bail!("Unexpected EOF")
        }

        Ok(visitor.stack.pop().unwrap())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn infix_to_postfix_simple() {
        let input: Vec<Box<dyn Token>> = vec![
            Box::new(NumberToken(5)),
            Box::new(Operation::Plus),
            Box::new(NumberToken(5)),
        ];
        let result = ParserVisitor::infix_to_postfix(&input);
        let expected: Vec<Box<dyn Token>> = vec![
            Box::new(NumberToken(5)),
            Box::new(NumberToken(5)),
            Box::new(Operation::Plus),
        ];
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn infix_to_postfix_hard() {
        let input: Vec<Box<dyn Token>> = vec![
            Box::new(NumberToken(2)),
            Box::new(Operation::Plus),
            Box::new(NumberToken(2)),
            Box::new(Operation::Star),
            Box::new(NumberToken(2)),
        ];
        let result = ParserVisitor::infix_to_postfix(&input);
        let expected: Vec<Box<dyn Token>> = vec![
            Box::new(NumberToken(2)),
            Box::new(NumberToken(2)),
            Box::new(NumberToken(2)),
            Box::new(Operation::Star),
            Box::new(Operation::Plus),
        ];
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn infix_to_postfix_non_associative() {
        let input: Vec<Box<dyn Token>> = vec![
            Box::new(NumberToken(4)),
            Box::new(Operation::Minus),
            Box::new(NumberToken(2)),
            Box::new(Operation::Slash),
            Box::new(NumberToken(2)),
        ];
        let result = ParserVisitor::infix_to_postfix(&input);
        let expected: Vec<Box<dyn Token>> = vec![
            Box::new(NumberToken(4)),
            Box::new(NumberToken(2)),
            Box::new(NumberToken(2)),
            Box::new(Operation::Slash),
            Box::new(Operation::Minus),
        ];
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn infix_to_postfix_braces() {
        let input: Vec<Box<dyn Token>> = vec![
            Box::new(Brace::Open),
            Box::new(NumberToken(2)),
            Box::new(Operation::Plus),
            Box::new(NumberToken(2)),
            Box::new(Brace::Close),
            Box::new(Operation::Star),
            Box::new(NumberToken(2)),
        ];
        let result = ParserVisitor::infix_to_postfix(&input);
        let expected: Vec<Box<dyn Token>> = vec![
            Box::new(NumberToken(2)),
            Box::new(NumberToken(2)),
            Box::new(Operation::Plus),
            Box::new(NumberToken(2)),
            Box::new(Operation::Star),
        ];
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn print_simple() {
        let input: Vec<Box<dyn Token>> = vec![
            Box::new(NumberToken(5)),
            Box::new(Operation::Plus),
            Box::new(NumberToken(5)),
        ];
        assert_eq!(PrintVisitor::write(&input).unwrap(), "5 + 5");
    }

    #[test]
    fn evaluate_simple() {
        let input: Vec<Box<dyn Token>> = vec![
            Box::new(NumberToken(2)),
            Box::new(NumberToken(2)),
            Box::new(NumberToken(2)),
            Box::new(Operation::Star),
            Box::new(Operation::Plus),
        ];
        let res = CalcVisitor::evaluate_postfix(&input);
        assert_eq!(res.unwrap(), 6);
    }
}