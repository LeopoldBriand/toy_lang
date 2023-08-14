use std::fmt;

pub struct LexicalError{
    pub line: i32,
    pub col: i32,
    pub message: String,
}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lexical error at l.{}, c.{}; {}", self.line, self.col, self.message)
    }
}

impl fmt::Debug for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

pub struct SyntaxError{
    pub line: i32,
    pub col: i32,
    pub message: String,
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Syntax error at l.{}, c.{}; {}", self.line, self.col, self.message)
    }
}

impl fmt::Debug for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}