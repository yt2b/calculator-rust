use crate::token::Token;
use std::iter::Peekable;

pub struct Lexer<T>
where
    T: Iterator<Item = char>,
{
    chars: Peekable<T>,
}

impl<T> Lexer<T>
where
    T: Iterator<Item = char>,
{
    pub fn new(chars: T) -> Self {
        Lexer {
            chars: chars.peekable(),
        }
    }

    fn parse_number(&mut self) -> String {
        let mut num = String::new();
        while is_digit(self.chars.peek()) {
            num.push(self.chars.next().unwrap());
        }
        num
    }
}

impl<T> Iterator for Lexer<T>
where
    T: Iterator<Item = char>,
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while is_whitespace(self.chars.peek()) {
            self.chars.next();
        }
        match self.chars.peek() {
            c if is_digit(c) => {
                let mut num = self.parse_number();
                if let Some('.') = self.chars.peek() {
                    self.chars.next();
                    num = format!("{}.{}", num, self.parse_number());
                }
                Some(Token::Number(num.parse::<f64>().unwrap()))
            }
            _ => match self.chars.next() {
                Some('+') => Some(Token::Plus),
                Some('-') => Some(Token::Minus),
                Some('*') => Some(Token::Asterisk),
                Some('/') => Some(Token::Slash),
                Some('(') => Some(Token::LParen),
                Some(')') => Some(Token::RParen),
                Some(c) => Some(Token::Unknown(c)),
                _ => None,
            },
        }
    }
}

fn is_whitespace(c: Option<&char>) -> bool {
    match c {
        Some(c) => c.is_whitespace(),
        _ => false,
    }
}

fn is_digit(c: Option<&char>) -> bool {
    match c {
        Some(c) => c.is_ascii_digit(),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let input = "1 + 2.3 -  456 *78.901  / (a)";
        let tokens = [
            Token::Number(1.0),
            Token::Plus,
            Token::Number(2.3),
            Token::Minus,
            Token::Number(456.0),
            Token::Asterisk,
            Token::Number(78.901),
            Token::Slash,
            Token::LParen,
            Token::Unknown('a'),
            Token::RParen,
        ];
        let lexer = Lexer::new(input.chars());
        for (expected, actual) in tokens.into_iter().zip(lexer) {
            assert_eq!(expected, actual);
        }
    }
}
