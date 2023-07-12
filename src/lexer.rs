use regex::Regex;
use std::error::Error;


#[derive(Clone, Debug)]
pub struct Token {
    token_type: TokenType,
    value: String,
}
#[derive(Clone, Debug)]
enum TokenType {
    Whitespace,
    Keyword,
    GroupDivider,
    Logical,
    Numeric,
    Text,
    Variable,
    Operator,
}

impl TokenType {
    fn regex(&self) -> &'static str {
        match self {
            TokenType::Whitespace => "[\\s\\t\\n\\r]",
            TokenType::Keyword => "(var|if|else|print)",
            TokenType::GroupDivider => "(\\(|\\)|\\{|\\}|;)",
            TokenType::Logical => "true|false",
            TokenType::Numeric => "[0-9]+",
            TokenType::Text => "\'([^\']*)\'",
            TokenType::Variable => "[a-zA-Z_]+[a-zA-Z0-9_]*",
            TokenType::Operator => "(\\+|\\-|>|<|\\={1,2}|\\!|\\:{2})",
        }
    }

    fn values() -> Vec<TokenType> {
        vec![
            TokenType::Whitespace,
            TokenType::Keyword,
            TokenType::GroupDivider,
            TokenType::Logical,
            TokenType::Numeric,
            TokenType::Text,
            TokenType::Variable,
            TokenType::Operator,
        ]
    }
}

pub struct LexicalParser {
    tokens: Vec<Token>,
    source: String,
}
impl LexicalParser {
    pub fn new(source: String) -> Self {
        Self {
            tokens: vec![],
            source,
        }
    }
    fn next_token(& mut self, position: usize) -> Result<usize, Box<dyn Error>> {
    let next_token = &self.source[position..];

    for token_type in TokenType::values().into_iter() {
        let pattern = Regex::new(&format!("^{}", token_type.regex()))?;
        let captures = pattern.captures(next_token);
        
        if let Some(captures) = captures {
            if !matches!(token_type, TokenType::Whitespace) {
                // group(1) is used to get text literal without double quotes
                let value = captures.get(1).map_or_else(|| captures.get(0).unwrap().as_str(), |m| m.as_str());
                let token = Token{token_type, value: value.to_owned()};
                self.tokens.push(token);
            }
            return Ok(captures.get(0).unwrap().as_str().len());
        }
    }

    Err(format!("invalid expression: `{}`", next_token).into())
}

pub fn parse(& mut self) -> Result<Vec<Token>, Box<dyn Error>> {
    let mut position: usize = 0;
    while position < self.source.len() {
        match self.next_token(position) {
            Ok(inc) => position += inc,
            Err(err) => return Err(err)
        }
    }
    Ok(self.tokens.clone())
}
}

