use crate::term::Term;

#[derive(Debug, Clone, PartialEq)]
enum Token {
    LParen,
    RParen,
    Lambda,
    Ident(String),
}

struct Lexer {
    chars: Vec<char>,
    pos: usize,
}

impl Lexer {
    fn new(input: &str) -> Self {
        Lexer {
            chars: input.chars().collect(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.chars.get(self.pos).copied();
        if c.is_some() {
            self.pos += 1;
        }
        c
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        loop {
            self.skip_whitespace();
            match self.peek() {
                None => break,
                Some('(') => {
                    self.advance();
                    tokens.push(Token::LParen);
                }
                Some(')') => {
                    self.advance();
                    tokens.push(Token::RParen);
                }
                Some('\\') | Some('\u{03bb}') => {
                    self.advance();
                    tokens.push(Token::Lambda);
                }
                Some(c) if is_ident_start(c) => {
                    let mut name = String::new();
                    while let Some(c) = self.peek() {
                        if is_ident_char(c) {
                            name.push(c);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    if name == "lambda" {
                        tokens.push(Token::Lambda);
                    } else {
                        tokens.push(Token::Ident(name));
                    }
                }
                Some(c) => {
                    return Err(format!(
                        "Unexpected character '{}' at position {}",
                        c, self.pos
                    ));
                }
            }
        }
        Ok(tokens)
    }
}

fn is_ident_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_ident_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '\''
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.pos);
        if tok.is_some() {
            self.pos += 1;
        }
        tok
    }

    fn expect(&mut self, expected: &Token) -> Result<(), String> {
        match self.advance() {
            Some(tok) if tok == expected => Ok(()),
            Some(tok) => Err(format!("Expected {:?}, got {:?}", expected, tok)),
            None => Err(format!("Expected {:?}, got end of input", expected)),
        }
    }

    /// term ::= ident
    ///        | '(' lambda '(' ident+ ')' term ')'
    ///        | '(' term+ ')'
    fn parse_term(&mut self) -> Result<Term, String> {
        match self.peek() {
            None => Err("Unexpected end of input".to_string()),
            Some(Token::Ident(_)) => {
                if let Some(Token::Ident(name)) = self.advance().cloned() {
                    Ok(Term::Var(name))
                } else {
                    unreachable!()
                }
            }
            Some(Token::LParen) => {
                self.advance(); // consume '('
                match self.peek() {
                    Some(Token::Lambda) => self.parse_abstraction(),
                    _ => self.parse_application(),
                }
            }
            Some(tok) => Err(format!("Unexpected token: {:?}", tok)),
        }
    }

    /// After '(' and seeing Lambda token:
    /// lambda '(' ident+ ')' term ')'
    fn parse_abstraction(&mut self) -> Result<Term, String> {
        self.advance(); // consume lambda token
        self.expect(&Token::LParen)?;

        // Parse one or more parameter names
        let mut params = Vec::new();
        loop {
            match self.peek() {
                Some(Token::Ident(_)) => {
                    if let Some(Token::Ident(name)) = self.advance().cloned() {
                        params.push(name);
                    }
                }
                Some(Token::RParen) => break,
                other => {
                    return Err(format!(
                        "Expected parameter name or ')', got {:?}",
                        other
                    ))
                }
            }
        }
        if params.is_empty() {
            return Err("Lambda must have at least one parameter".to_string());
        }
        self.expect(&Token::RParen)?; // close param list

        let body = self.parse_term()?;
        self.expect(&Token::RParen)?; // close abstraction

        // Desugar multi-param: (λ (x y z) body) → (λ (x) (λ (y) (λ (z) body)))
        let mut result = body;
        for param in params.into_iter().rev() {
            result = Term::Abs(param, Box::new(result));
        }
        Ok(result)
    }

    /// After '(' for application: term+ ')'
    /// Single term in parens: (term) = term
    /// Multiple terms: (t1 t2 t3 ...) = (((t1 t2) t3) ...)
    fn parse_application(&mut self) -> Result<Term, String> {
        let mut terms = Vec::new();
        loop {
            match self.peek() {
                Some(Token::RParen) => {
                    self.advance();
                    break;
                }
                None => return Err("Unclosed parenthesis".to_string()),
                _ => {
                    terms.push(self.parse_term()?);
                }
            }
        }
        if terms.is_empty() {
            return Err("Empty parentheses".to_string());
        }
        if terms.len() == 1 {
            return Ok(terms.into_iter().next().unwrap());
        }
        // Left-associative application
        let mut result = terms.remove(0);
        for t in terms {
            result = Term::App(Box::new(result), Box::new(t));
        }
        Ok(result)
    }
}

/// Parse a CIS352-syntax lambda calculus term
pub fn parse(input: &str) -> Result<Term, String> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize()?;
    if tokens.is_empty() {
        return Err("Empty input".to_string());
    }
    let mut parser = Parser::new(tokens);
    let term = parser.parse_term()?;
    if parser.pos < parser.tokens.len() {
        return Err(format!(
            "Unexpected tokens after term: {:?}",
            &parser.tokens[parser.pos..]
        ));
    }
    Ok(term)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_var() {
        assert_eq!(parse("x").unwrap(), Term::var("x"));
    }

    #[test]
    fn test_parse_abs() {
        let result = parse("(\\ (x) x)").unwrap();
        assert_eq!(result, Term::abs("x", Term::var("x")));
    }

    #[test]
    fn test_parse_abs_lambda_char() {
        let result = parse("(\u{03bb} (x) x)").unwrap();
        assert_eq!(result, Term::abs("x", Term::var("x")));
    }

    #[test]
    fn test_parse_app() {
        let result = parse("(f x)").unwrap();
        assert_eq!(result, Term::app(Term::var("f"), Term::var("x")));
    }

    #[test]
    fn test_parse_multi_param() {
        let result = parse("(\\ (x y) x)").unwrap();
        assert_eq!(
            result,
            Term::abs("x", Term::abs("y", Term::var("x")))
        );
    }

    #[test]
    fn test_parse_multi_app() {
        let result = parse("(f x y)").unwrap();
        assert_eq!(
            result,
            Term::app(Term::app(Term::var("f"), Term::var("x")), Term::var("y"))
        );
    }

    #[test]
    fn test_parse_nested() {
        let result = parse("((\\ (x) x) y)").unwrap();
        assert_eq!(
            result,
            Term::app(Term::abs("x", Term::var("x")), Term::var("y"))
        );
    }

    #[test]
    fn test_parse_complex() {
        let result = parse("(\\ (f) (\\ (x) (f (x x))))").unwrap();
        let expected = Term::abs(
            "f",
            Term::abs(
                "x",
                Term::app(Term::var("f"), Term::app(Term::var("x"), Term::var("x"))),
            ),
        );
        assert_eq!(result, expected);
    }
}
