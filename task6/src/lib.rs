use anyhow::Result;

use crate::tokenizer::{Token, tokenize};
use crate::visitors::{CalcVisitor, ParserVisitor, PrintVisitor};

mod tokenizer;
mod visitors;

pub fn tokenize_infix(data: &str) -> Result<Vec<Box<dyn Token>>> {
    tokenize(data)
}

pub fn infix_to_postfix(data: &Vec<Box<dyn Token>>) -> Result<Vec<Box<dyn Token>>> {
    ParserVisitor::infix_to_postfix(data)
}

pub fn calc_postfix(data: &Vec<Box<dyn Token>>) -> Result<i32> {
    CalcVisitor::evaluate_postfix(data)
}

pub fn tokens_to_string(data: &Vec<Box<dyn Token>>) -> Result<String> {
    PrintVisitor::write(data)
}

pub fn run(data: &str) -> Result<(String, i32)> {
    let tokens = tokenize_infix(data)?;
    let postfix = infix_to_postfix(&tokens)?;
    let res = calc_postfix(&postfix)?;
    let postfix_repr = tokens_to_string(&postfix)?;

    Ok((postfix_repr, res))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn precedence() {
        let result = run("2 + 2 * 2");
        assert_eq!(result.unwrap(), ("2 2 2 * +".to_owned(), 6));
    }

    #[test]
    fn precedence_with_braces() {
        let result = run("(2 + 2) * 2");
        assert_eq!(result.unwrap(), ("2 2 + 2 *".to_owned(), 8));
    }

    #[test]
    fn division_with_braces() {
        let result = run("(2 + 2) / 2");
        assert_eq!(result.unwrap(), ("2 2 + 2 /".to_owned(), 2));
    }

    #[test]
    fn division_left_associative() {
        let result = run("4 / 2 / 2");
        assert_eq!(result.unwrap(), ("4 2 / 2 /".to_owned(), 1));
    }

    #[test]
    fn subtraction_left_associative() {
        let result = run("4 - 2 - 2");
        assert_eq!(result.unwrap(), ("4 2 - 2 -".to_owned(), 0));
    }
}