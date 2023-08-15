use regex::Regex;

use crate::errors::LexicalError;

#[derive(Clone, Debug)]
pub struct Token {
    pub pos: (i32, i32),
    pub token_type: TokenType,
    pub value: String,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenType {
    Whitespace,
    Keyword,
    GroupDivider,
    StartOfBlock,
    EndOfBlock,
    EndOfStatement,
    Logical,
    Numeric,
    Text,
    Identifier,
    Operator,
}

impl TokenType {
    fn regex(&self) -> &'static str {
        match self {
            TokenType::Whitespace => "[\\s\\t\\n\\r]",
            TokenType::Keyword => "(var|if|else|print)",
            TokenType::GroupDivider => "(\\(|\\))",
            TokenType::StartOfBlock => "(\\{)",
            TokenType::EndOfBlock => "(\\})",
            TokenType::EndOfStatement => "(;)",
            TokenType::Numeric => "[0-9]+",
            TokenType::Text => "\'([^\']*)\'",
            TokenType::Logical => "(true|false)",
            TokenType::Identifier => "[a-zA-Z_]+[a-zA-Z0-9_]*",
            TokenType::Operator => "(\\+|\\-|&&|\\|\\||>|<|\\={1,2}|\\!|\\/|\\*|\\%)",
        }
    }

    fn values() -> Vec<TokenType> {
        vec![
            TokenType::Whitespace,
            TokenType::Keyword,
            TokenType::GroupDivider,
            TokenType::StartOfBlock,
            TokenType::EndOfBlock,
            TokenType::EndOfStatement,
            TokenType::Numeric,
            TokenType::Text,
            TokenType::Logical,
            TokenType::Identifier,
            TokenType::Operator,
        ]
    }
}

#[derive(Debug)]
pub struct LexicalParser {
    tokens: Vec<Token>,
    source: Vec<String>,
}
impl LexicalParser {
    pub fn new(source: Vec<String>) -> Self {
        Self {
            tokens: vec![],
            source,
        }
    }
    fn next_token(&mut self, current_line:String, line_position: usize, col_position: usize) -> Result<usize, String> {
        let next_token = &current_line[col_position..];

        for token_type in TokenType::values().into_iter() {
            match Regex::new(&format!("^{}", token_type.regex())) {
                Ok(pattern) => {
                    let captures = pattern.captures(next_token);
                    if let Some(captures) = captures {
                        if !matches!(token_type, TokenType::Whitespace) {
                            // group(1) is used to get text literal without double quotes
                            let value = captures
                                .get(1)
                                .map_or_else(|| captures.get(0).unwrap().as_str(), |m| m.as_str());
                            let token = Token {
                                pos: (line_position as i32, col_position as i32),
                                token_type,
                                value: value.to_owned(),
                            };
                            self.tokens.push(token);
                        }
                        match captures.get(0) {
                            Some(capture) => return Ok(capture.as_str().len()),
                            None => return Err("Unknown Token".to_owned()),
                        }
                    }
                }
                Err(_) => return Err("Unknown Token".to_owned()),
            }
        }

        Err(format!("invalid expression: `{}`", next_token).into())
    }

    pub fn parse(&mut self) -> Result<Vec<Token>, LexicalError> {
        for (pos, line) in self.source.clone().iter().enumerate() {
            let mut col: usize = 0;
            while col < line.len() {
                match self.next_token(line.clone(), pos, col) {
                    Ok(inc) => col += inc,
                    Err(err) => {
                        return Err(LexicalError {
                            line: pos as i32,
                            col: col as i32,
                            message: err,
                        })
                    },
                }
            }
        }
        Ok(self.tokens.clone())
    }
}
