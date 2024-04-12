fn main() {
    // let json_string = r#"{
    //     "key1": true,
    //     "key2": false,
    //     "key3": null,
    //     "key4": "value",
    //     "key5": 101
    //   }"#;

    let json_string = r#"{
        "key": "value",
        "key-n": 101,
        "key-o": {},
        "key-l": []
      }"#;
    println!("Size of the string: {}", json_string.len());
    println!("String representation: {}", json_string);

    let lexer = Lexer::new(json_string.to_string());

    println!("Lexer {:?}", lexer);
}

#[derive(Debug)]
struct Lexer {
    input: String,
    tokens: Vec<Token>,
    position: usize,
    read_position: usize,
    current_char: char,
}

impl Lexer {
    fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input,
            tokens: Vec::new(),
            position: 0,
            read_position: 0,
            current_char: '0',
        };

        lexer.tokenize();

        lexer
    }

    fn tokenize(&mut self) {
        while self.position < self.input.len() {
            self.next_token()
        }

        self.add_token(SyntaxKind::EOF, "".to_string());
    }

    fn add_token(&mut self, kind: SyntaxKind, value: String) {
        self.tokens.push(Token {
            syntax_kind: kind,
            value,
            position: self.position,
        });
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.current_char = '\0';
        } else {
            self.current_char = self.input.chars().nth(self.read_position).unwrap();
        }
    }

    fn next(&mut self) {
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) {
        self.read_char();
        match self.current_char {
            '{' => {
                self.add_token(SyntaxKind::LeftCurlyBracket, self.current_char.to_string());
                self.next();
            }
            '}' => {
                self.add_token(SyntaxKind::RigthCurlyBracket, self.current_char.to_string());
                self.next();
            }
            '[' => {
                self.add_token(SyntaxKind::LeftSquareBracket, self.current_char.to_string());
                self.next();
            }
            ']' => {
                self.add_token(
                    SyntaxKind::RightSquareBracket,
                    self.current_char.to_string(),
                );
                self.next();
            }
            ':' => {
                self.add_token(SyntaxKind::Colon, self.current_char.to_string());
                self.next();
            }
            '"' => {
                self.scan_string();
                self.next();
            }
            't' | 'f' => {
                self.scan_boolean();
                self.next();
            }
            'n' => {
                self.scan_null();
                self.next();
            }
            '0'..='9' => {
                self.scan_number();
                self.next();
            }
            ',' => {
                self.add_token(SyntaxKind::Comma, self.current_char.to_string());
                self.next();
            }
            ' ' => {
                self.next();
            }
            '\0' => {
                self.next();
            }
            '\n' => {
                self.next();
            }
            _ => todo!(),
        }
    }

    fn scan_string(&mut self) {
        let mut value = String::new();

        self.next();
        self.read_char();

        while self.current_char != '"' {
            value.push(self.current_char);
            self.next();
            self.read_char();
        }

        self.add_token(SyntaxKind::String, value);
    }

    fn scan_boolean(&mut self) {
        let mut value = String::new();

        while self.current_char.is_alphabetic() {
            value.push(self.current_char);
            self.next();
            self.read_char();
        }

        self.add_token(SyntaxKind::Boolean, value);
    }

    fn scan_null(&mut self) {
        let mut value = String::new();

        while self.current_char.is_alphabetic() {
            value.push(self.current_char);
            self.next();
            self.read_char();
        }

        self.add_token(SyntaxKind::Null, value);
    }

    fn scan_number(&mut self) {
        let mut value = String::new();

        while self.current_char.is_digit(10) || self.current_char == '.' {
            value.push(self.current_char);
            self.next();
            self.read_char();
        }

        match value.parse::<f64>() {
            Ok(number) => self.add_token(SyntaxKind::Float, number.to_string()),
            Err(_) => self.add_token(SyntaxKind::Error, value),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Token {
    syntax_kind: SyntaxKind,
    value: String,
    position: usize,
}

#[derive(Debug, PartialEq)]
enum SyntaxKind {
    LeftCurlyBracket,
    RigthCurlyBracket,
    Float,
    String,
    Null,
    LeftSquareBracket,
    RightSquareBracket,
    Comma,
    Colon,
    EOF,
    Boolean,
    Error,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_lexer_creation() {}
}
