use crate::token::Token;

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    next_pos: usize,
    ch: u8,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            pos: 0,
            next_pos: 0,
            ch: 0,
        };

        lexer.read_char();

        return lexer;
    }

    fn read_char(&mut self) {
        if self.next_pos >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.next_pos];
        }
        self.pos = self.next_pos;
        self.next_pos += 1;
    }

    fn nextch(&mut self) -> u8 {
        if self.next_pos >= self.input.len() {
            return 0;
        } else {
            return self.input.as_bytes()[self.next_pos];
        }
    }

    fn nextch_is(&mut self, ch: u8) -> bool {
        self.nextch() == ch
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.ch {
                b' ' | b'\t' => {
                    self.read_char();
                }
                _ => {
                    break;
                }
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            b'=' => {
                if self.nextch_is(b'=') {
                    self.read_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'!' => {
                if self.nextch_is(b'=') {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::Bang
                }
            }
            b'/' => Token::Slash,
            b'*' => Token::Asterisk,
            b'<' => {
                if self.nextch_is(b'=') {
                    self.read_char();
                    Token::LessThanEqual
                } else {
                    Token::LessThan
                }
            }
            b'>' => {
                if self.nextch_is(b'=') {
                    self.read_char();
                    Token::GreaterThanEqual
                } else {
                    Token::GreaterThan
                }
            }
            b'(' => Token::Lparen,
            b')' => Token::Rparen,
            b'{' => Token::Lbrace,
            b'}' => Token::Rbrace,
            b'[' => Token::Lbracket,
            b']' => Token::Rbracket,
            b'.' => Token::Dot,
            b',' => Token::Comma,
            b';' => Token::Semicolon,
            b':' => Token::Colon,
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                return self.consume_identifier();
            }
            b'0'..=b'9' => {
                return self.consume_number();
            }
            b'"' => {
                return self.consume_string();
            }
            b'\'' => {
                return self.consume_char();
            }
            b'\n' => {
                if self.nextch_is(b'\n') {
                    Token::Blank
                } else {
                    self.read_char();
                    return self.next_token();
                }
            }
            0 => Token::Eof,
            _ => Token::Illegal,
        };

        self.read_char();

        return tok;
    }

    fn consume_identifier(&mut self) -> Token {
        let start_pos = self.pos;

        loop {
            match self.ch {
                b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' => {
                    self.read_char();
                }
                _ => {
                    break;
                }
            }
        }

        let literal = &self.input[start_pos..self.pos];

        match literal {
            "fun" => Token::Func,
            "let" => Token::Let,
            "true" => Token::Bool(true),
            "false" => Token::Bool(false),
            "if" => Token::If,
            "else" => Token::Else,
            "pub" => Token::Pub,
            "import" => Token::Import,
            "return" => Token::Return,
            _ => Token::Ident(String::from(literal)),
        }
    }

    fn consume_number(&mut self) -> Token {
        let start_pos = self.pos;
        let mut is_float = false;

        loop {
            match self.ch {
                b'0'..=b'9' | b'.' => {
                    if self.ch == b'.' {
                        is_float = true;
                    }
                    self.read_char();
                }
                _ => {
                    break;
                }
            }
        }

        let literal = &self.input[start_pos..self.pos];

        if is_float {
            Token::Float(literal.parse::<f64>().unwrap())
        } else {
            Token::Int(literal.parse::<i64>().unwrap())
        }
    }

    fn consume_string(&mut self) -> Token {
        self.read_char();

        let start_pos = self.pos;

        loop {
            match self.ch {
                b'"' | 0 => {
                    let literal = &self.input[start_pos..self.pos];
                    self.read_char();
                    return Token::String(literal.to_string());
                }
                _ => {
                    self.read_char();
                }
            }
        }
    }

    fn consume_char(&mut self) -> Token {
        self.read_char();

        let literal = self.ch as char;
        self.read_char();
        self.read_char();

        Token::Char(literal)
    }
}
