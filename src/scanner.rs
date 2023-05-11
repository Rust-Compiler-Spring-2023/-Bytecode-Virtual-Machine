use crate::token_type::TokenType;
pub struct Scanner {
    pub source : String,
    start : usize,
    current : usize,
    line: usize,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub _type : TokenType,
    pub lexeme: String,
    pub line : usize
}

impl Scanner {
    pub fn new() -> Self {
        Scanner{
            source: String::new(),
            start: 0,
            current: 0,
            line: 1
        }
    }

    /*
    Scans the current token
    */
    pub fn scan_token(&mut self) -> Token {
        self.skip_white_space();

        self.start = self.current;

        if self.is_at_end() { return self.make_token(TokenType::TokenEOF); }

        let curr_char: char = self.advance();

        if self.is_alpha(curr_char) { return self.identifier(); }
        if self.is_digit(curr_char) { return self.number(); }

        match curr_char {
            '(' => return self.make_token(TokenType::TokenLeftParen),
            ')' => return self.make_token(TokenType::TokenRightParen),
            '{' => return self.make_token(TokenType::TokenLeftBrace),
            '}' => return self.make_token(TokenType::TokenRightBrace),
            ';' => return self.make_token(TokenType::TokenSemicolon),
            ',' => return self.make_token(TokenType::TokenComma),
            '.' => return self.make_token(TokenType::TokenDot),
            '-' => return self.make_token(TokenType::TokenMinus),
            '+' => return self.make_token(TokenType::TokenPlus),
            '/' => return self.make_token(TokenType::TokenSlash),
            '*' => return self.make_token(TokenType::TokenStar),
            '^' => return self.make_token(TokenType::TokenCarat),
            '!' => {
                if self.matching('='){
                    return self.make_token(TokenType::TokenBangEqual);
                } else {
                    return self.make_token(TokenType::TokenBang);
                }
            }
            '=' => {
                if self.matching('='){
                    return self.make_token(TokenType::TokenEqualEqual);
                } else {
                    return self.make_token(TokenType::TokenEqual);
                }
            }
            '<' => {
                if self.matching('='){
                    return self.make_token(TokenType::TokenLessEqual);
                }
                else{
                    return self.make_token(TokenType::TokenLess);
                }
            }
            '>' => {
                if self.matching('='){
                    return self.make_token(TokenType::TokenGreaterEqual);
                } else {
                    return self.make_token(TokenType::TokenGreater);
                }
            }
            '"' => return self.string(),
            
            _ => return {
                //print!("-> {}", curr_char);
                self.error_token("Unexpected Character.")
            } 
        }
    }

    // If the current character is the desired one, we advance and return true. Otherwise, we return false to indicate it wasn’t matched. 
    fn matching(&mut self, expected : char) -> bool {
        if self.is_at_end() { return false; }
        let curr_char_index : usize = self.current;
        // If the char at the current index position is not the same as the char expected, return false
        if self.source.char_at(curr_char_index) != expected { return false; }
        else {
            self.current += 1;
            return true;
        }
    }

    // consumes the current character and returns it
    fn advance(&mut self) -> char {
        let curr_source_char: char = self.source.char_at(self.current);
        self.current += 1;

        curr_source_char
    }

    // Check if we have reached the end of our source string
    fn is_at_end(&mut self) -> bool {
        self.source.char_at(self.current) == '\0'
    }

    // Checks char at curr position. Doesn't increase the curr
    fn peek(&mut self) -> char {
        self.source.char_at(self.current)
    }

    // This is like peek() but for one character past the current one
    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() { return '\0'; }

        self.source.char_at(self.current + 1)
    }

    fn is_digit(&self, c : char) -> bool {
        (c >= '0') && (c <= '9')
    }

    fn is_alpha(&self, c : char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c == '_')
    }

    /*
    Continue until end of identifier and return the identifier type
    */
    fn identifier(&mut self) -> Token {
        let mut peek = self.peek();
        while self.is_alpha(peek) || self.is_digit(peek) { self.advance(); peek = self.peek(); };
        let identify_type: TokenType = self.identifier_type();
        
        self.make_token(identify_type)
    }

    /*
    Match the lexeme with the correct identifier type
    */
    fn identifier_type(&mut self) -> TokenType {
        match self.source.char_at(self.start){
            'a' => return self.check_keyword(1,2,"nd", TokenType::TokenAnd),
            'c' => return self.check_keyword(1, 4, "lass", TokenType::TokenClass),
            'e' => return self.check_keyword(1, 3, "lse", TokenType::TokenElse),
            'f' => {
                if self.current - self.start > 1{
                    match self.source.char_at(self.start + 1){
                        'a' => return self.check_keyword(2, 3, "lse", TokenType::TokenFalse),
                        'o' => return self.check_keyword(2, 1, "r", TokenType::TokenFor),
                        'u' => return self.check_keyword(2, 1, "n", TokenType::TokenFun),
                        _ => return TokenType::TokenIdentifier
                    }
                }
            }
            'i' => return self.check_keyword(1, 1, "f", TokenType::TokenIf),
            'n' => return self.check_keyword(1, 2, "il", TokenType::TokenNil),
            'o' => return self.check_keyword(1, 1, "r", TokenType::TokenOr),
            'p' => return self.check_keyword(1, 4, "rint", TokenType::TokenPrint),
            'r' => return self.check_keyword(1, 5, "eturn", TokenType::TokenReturn),
            's' => return self.check_keyword(1, 4, "uper", TokenType::TokenSuper),
            't' => {
                if self.current - self.start > 1 {
                    match self.source.char_at(self.start + 1){
                        'h' => return self.check_keyword(2, 2, "is", TokenType::TokenThis),
                        'r' => return self.check_keyword(2, 2, "ue", TokenType::TokenTrue),
                        _ => return TokenType::TokenIdentifier
                    }
                }
            }
            'v' => return self.check_keyword(1, 2, "ar", TokenType::TokenVar),
            'w' => return self.check_keyword(1, 4, "hile", TokenType::TokenWhile),
            'x' => {
                if self.current - self.start > 1 {
                    match self.source.char_at(self.start + 1){
                        'o' => return self.check_keyword(2, 1, "r", TokenType::TokenBangEqual),
                        'a' => return self.check_keyword(2, 2, "nd", TokenType::TokenEqualEqual),
                        _ => return TokenType::TokenIdentifier
                    }
                }
            }
            _ => return TokenType::TokenIdentifier
        }

        TokenType::TokenIdentifier
    }

    /*
    Checks if the keyword watches
    */
    fn check_keyword(&mut self, start : usize, length : usize, rest : &str, _type : TokenType) -> TokenType {
        // Check if length of keyword matches lexeme
        if self.current - self.start != start + length {
            return TokenType::TokenIdentifier;
        }
        let lexeme = self.source.substring(self.start + start, self.current);

        if lexeme == rest.to_string() {
            return _type;
        }

        TokenType::TokenIdentifier
    }

    // Gets the token for a number
    fn number(&mut self) -> Token {
        let mut peek: char = self.peek();
        while self.is_digit(peek) {self.advance(); peek = self.peek();}
            // Look for fractional part
            let peek_next = self.peek_next();
            if self.peek() == '.' && self.is_digit(peek_next){
                // consume the "."
                self.advance();

                peek = self.peek();
                while self.is_digit(peek) {self.advance(); peek = self.peek();}
            }

        self.make_token(TokenType::TokenNumber)
    }

    // Gets the token for a string
    fn string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n'{self.line += 1;}
            self.advance();
        }

        if self.is_at_end() { return self.error_token("Unterminated String."); }
        self.advance(); // The closing quote

        self.make_token(TokenType::TokenString)
    }   

    // This advances the scanner past any leading whitespace
    fn skip_white_space(&mut self) {
        loop {
            let curr_char : char = self.peek();
            match curr_char{
                ' ' => { self.advance(); },
                '\r' => { self.advance(); },
                '\t' => { self.advance(); },
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    if self.peek_next() == '/' {
                        // A comment goes until the end of the line
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return
                    }
                }
                _ => return
            }
        }
    }

    // Makes the token, with the corresponding data
    fn make_token(&self, _type: TokenType) -> Token {
        Token { 
            _type: _type,
            lexeme: self.source.substring(self.start, self.current), 
            line: self.line 
        }
    }

    // Makes error token, with the corresponding data
    fn error_token(&self, message: &str) -> Token {
        Token {
            _type: TokenType::TokenError,
            lexeme: message.to_string(),
            line: self.line
        }
    }
}

/*
CITE: https://stackoverflow.com/questions/37157926/is-there-a-method-like-javascripts-substr-in-rust
*/
trait StringUtils {
    fn substring(&self, begin_index: usize, end_index: usize) -> Self;
    fn char_at(&mut self, index_pos: usize) -> char;
}


impl StringUtils for String {
    /* 
    Returns a substring, which begins at the specified begin_index and extends
    to the character at index end_index - 1
    */
    fn substring(&self, begin_index: usize, end_index: usize) -> Self {
        if begin_index + (end_index - begin_index) > self.len() {
            panic!("substring(): index out of bounds");
        }
        self.chars().skip(begin_index).take(end_index - begin_index).collect()
    }

    /*
    Gets the character in index position
    */
    fn char_at(&mut self, index_pos: usize) -> char {
        let curr_source_char : char =  match self.chars().nth(index_pos) {
            Some(x) => x,
            None => {
                println!("advance(): char not accessible by index. Returning empty space. Index: {}", index_pos);
                return ' '
            }
        };

        curr_source_char
    }
}
