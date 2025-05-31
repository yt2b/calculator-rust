use crate::token::Token;
use std::iter::Peekable;

pub struct Parser<T>
where
    T: Iterator<Item = Token>,
{
    tokens: Peekable<T>,
}

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    pub fn new(tokens: T) -> Self {
        Parser {
            tokens: tokens.peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<f64, String> {
        let value = self.parse_expression()?;
        match self.tokens.next() {
            Some(token) => Err(format!("Invalid token {:?}", token)),
            None => Ok(value),
        }
    }

    fn parse_expression(&mut self) -> Result<f64, String> {
        let mut left_value = self.parse_term()?;
        while let Some(Token::Plus | Token::Minus) = self.tokens.peek() {
            let token = self.tokens.next();
            let right_value = self.parse_term()?;
            if let Some(Token::Plus) = token {
                left_value += right_value;
            } else {
                left_value -= right_value;
            }
        }
        Ok(left_value)
    }

    fn parse_term(&mut self) -> Result<f64, String> {
        let mut left_value = self.parse_factor()?;
        while let Some(Token::Asterisk | Token::Slash) = self.tokens.peek() {
            let token = self.tokens.next();
            let right_value = self.parse_factor()?;
            if let Some(Token::Asterisk) = token {
                left_value *= right_value;
            } else {
                left_value /= right_value;
            }
        }
        Ok(left_value)
    }

    fn parse_factor(&mut self) -> Result<f64, String> {
        match self.tokens.next() {
            Some(Token::LParen) => {
                let value = self.parse_expression()?;
                if let Some(Token::RParen) = self.tokens.peek() {
                    self.tokens.next();
                    Ok(value)
                } else {
                    Err("RParen not found".to_string())
                }
            }
            Some(Token::Number(value)) => Ok(value),
            Some(token) => Err(format!("Invalid token {:?}", token)),
            None => Err("Invalid syntax".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn parse(exp: &str) -> Result<f64, String> {
        let lexer = Lexer::new(exp.chars());
        let mut parser = Parser::new(lexer);
        parser.parse()
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse("1"), Ok(1.0));
        assert_eq!(parse("2.1 + 6 / 2"), Ok(5.1));
        assert_eq!(parse("(6.4 + 3.6) * (10 - 2)"), Ok(80.0));
        assert_eq!(parse("1 2.0"), Err("Invalid token Number(2.0)".to_string()));
        assert_eq!(parse("3 + "), Err("Invalid syntax".to_string()));
        assert_eq!(parse("4- / "), Err("Invalid token Slash".to_string()));
        assert_eq!(parse("(12 + 345"), Err("RParen not found".to_string()));
    }
}
