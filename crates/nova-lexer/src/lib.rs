use nova_ast::{Span, Unit};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Keywords
    Scene,
    Camera,
    Environment,
    Model,
    Mesh,
    Material,
    Light,
    Animate,
    Interaction,
    Hover,
    Click,
    Scroll,
    Page,
    Section,
    Heading,
    Text,
    Button,
    Container,
    Card,
    Nav,
    Link,
    Component,
    State,
    Open,
    Navigate,
    True,
    False,
    Animation,
    At,

    // Symbols & Operators
    OpenBrace,     // {
    CloseBrace,    // }
    OpenParen,     // (
    CloseParen,    // )
    Colon,         // :
    Equal,         // =
    PlusEqual,     // +=
    MinusEqual,    // -=
    StarEqual,     // *=
    Dot,           // .
    Comma,         // ,
    FatArrow,      // =>
    Arrow,         // ->
    Plus,          // +
    Minus,         // -
    Star,          // *
    Slash,         // /
    PercentSymbol, // %

    // Literals
    Identifier(String),
    Number(f64),
    UnitNumber(f64, Unit),
    StringLiteral(String),
    ColorLiteral(String),

    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

pub struct Lexer<'a> {
    _input: &'a str,
    chars: Vec<char>,
    pos: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            _input: input,
            chars: input.chars().collect(),
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    fn peek(&self) -> Option<char> {
        if self.pos < self.chars.len() {
            Some(self.chars[self.pos])
        } else {
            None
        }
    }

    fn peek_next(&self) -> Option<char> {
        if self.pos + 1 < self.chars.len() {
            Some(self.chars[self.pos + 1])
        } else {
            None
        }
    }

    fn advance(&mut self) -> Option<char> {
        if self.pos < self.chars.len() {
            let ch = self.chars[self.pos];
            self.pos += 1;
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            Some(ch)
        } else {
            None
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else if ch == '/' && self.peek_next() == Some('/') {
                // Line comment
                while let Some(c) = self.peek() {
                    if c == '\n' {
                        break;
                    }
                    self.advance();
                }
            } else if ch == '/' && self.peek_next() == Some('*') {
                // Block comment
                self.advance();
                self.advance();
                while let Some(c) = self.peek() {
                    if c == '*' && self.peek_next() == Some('/') {
                        self.advance();
                        self.advance();
                        break;
                    }
                    self.advance();
                }
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace_and_comments();

        let start_line = self.line;
        let start_col = self.column;
        let span = Span::new(start_line, start_col);

        let ch = match self.peek() {
            Some(c) => c,
            None => {
                return Token {
                    kind: TokenKind::Eof,
                    span,
                }
            }
        };

        // Identifiers or Keywords
        if ch.is_alphabetic() || ch == '_' {
            let mut ident = String::new();
            while let Some(c) = self.peek() {
                if c.is_alphanumeric() || c == '_' {
                    ident.push(c);
                    self.advance();
                } else {
                    break;
                }
            }

            let kind = match ident.as_str() {
                "scene" => TokenKind::Scene,
                "camera" => TokenKind::Camera,
                "environment" => TokenKind::Environment,
                "model" => TokenKind::Model,
                "mesh" => TokenKind::Mesh,
                "material" => TokenKind::Material,
                "light" => TokenKind::Light,
                "animate" => TokenKind::Animate,
                "interaction" => TokenKind::Interaction,
                "hover" => TokenKind::Hover,
                "click" => TokenKind::Click,
                "scroll" => TokenKind::Scroll,
                "page" => TokenKind::Page,
                "section" => TokenKind::Section,
                "heading" => TokenKind::Heading,
                "text" => TokenKind::Text,
                "button" => TokenKind::Button,
                "container" => TokenKind::Container,
                "card" => TokenKind::Card,
                "nav" => TokenKind::Nav,
                "link" => TokenKind::Link,
                "component" => TokenKind::Component,
                "state" => TokenKind::State,
                "open" => TokenKind::Open,
                "navigate" => TokenKind::Navigate,
                "true" => TokenKind::True,
                "false" => TokenKind::False,
                "animation" => TokenKind::Animation,
                "at" => TokenKind::At,
                _ => TokenKind::Identifier(ident),
            };

            return Token { kind, span };
        }

        // Numbers, Units, Percentages
        if ch.is_ascii_digit() {
            let mut num_str = String::new();
            while let Some(c) = self.peek() {
                if c.is_ascii_digit() || c == '.' {
                    num_str.push(c);
                    self.advance();
                } else {
                    break;
                }
            }

            let val: f64 = num_str.parse().unwrap_or(0.0);

            // Check for units immediately attached
            let mut unit_str = String::new();
            while let Some(c) = self.peek() {
                if c.is_alphabetic() || c == '%' {
                    unit_str.push(c);
                    self.advance();
                } else {
                    break;
                }
            }

            let kind = if !unit_str.is_empty() {
                let unit = match unit_str.as_str() {
                    "deg" => Unit::Deg,
                    "rad" => Unit::Rad,
                    "s" => Unit::Sec,
                    "ms" => Unit::Ms,
                    "px" => Unit::Px,
                    "%" => Unit::Percent,
                    _ => Unit::None,
                };
                if unit == Unit::None {
                    TokenKind::Number(val)
                } else {
                    TokenKind::UnitNumber(val, unit)
                }
            } else {
                TokenKind::Number(val)
            };

            return Token { kind, span };
        }

        // Strings: "..." or './...'
        if ch == '"' || ch == '\'' {
            let quote = ch;
            self.advance(); // consume opening quote
            let mut s = String::new();
            while let Some(c) = self.peek() {
                if c == quote {
                    self.advance(); // consume closing quote
                    break;
                }
                s.push(c);
                self.advance();
            }
            return Token {
                kind: TokenKind::StringLiteral(s),
                span,
            };
        }

        // Hex Colors: #080808 or #fff
        if ch == '#' {
            self.advance(); // consume '#'
            let mut hex = String::from("#");
            while let Some(c) = self.peek() {
                if c.is_ascii_hexdigit() {
                    hex.push(c);
                    self.advance();
                } else {
                    break;
                }
            }
            return Token {
                kind: TokenKind::ColorLiteral(hex),
                span,
            };
        }

        // Symbols & Operators
        self.advance();
        let kind = match ch {
            '{' => TokenKind::OpenBrace,
            '}' => TokenKind::CloseBrace,
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,
            ':' => TokenKind::Colon,
            ',' => TokenKind::Comma,
            '.' => TokenKind::Dot,
            '%' => TokenKind::PercentSymbol,
            '+' => {
                if self.peek() == Some('=') {
                    self.advance();
                    TokenKind::PlusEqual
                } else {
                    TokenKind::Plus
                }
            }
            '-' => {
                if self.peek() == Some('=') {
                    self.advance();
                    TokenKind::MinusEqual
                } else if self.peek() == Some('>') {
                    self.advance();
                    TokenKind::Arrow
                } else {
                    TokenKind::Minus
                }
            }
            '*' => {
                if self.peek() == Some('=') {
                    self.advance();
                    TokenKind::StarEqual
                } else {
                    TokenKind::Star
                }
            }
            '/' => TokenKind::Slash,
            '=' => {
                if self.peek() == Some('>') {
                    self.advance();
                    TokenKind::FatArrow
                } else {
                    TokenKind::Equal
                }
            }
            _ => TokenKind::Identifier(ch.to_string()),
        };

        Token { kind, span }
    }

    pub fn tokenize_all(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            let is_eof = tok.kind == TokenKind::Eof;
            tokens.push(tok);
            if is_eof {
                break;
            }
        }
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let code = r#"
            scene Product {
                camera main {
                    position: (0, 2, 8)
                    fov: 45
                }
                model phone {
                    source: "./phone.glb"
                    animate {
                        rotation.y += 0.5deg
                    }
                }
            }
        "#;

        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize_all();

        assert_eq!(tokens[0].kind, TokenKind::Scene);
        assert_eq!(tokens[1].kind, TokenKind::Identifier("Product".into()));
        assert_eq!(tokens[2].kind, TokenKind::OpenBrace);
    }
}
