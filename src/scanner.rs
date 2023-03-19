use crate::token_type::TokenType;
pub struct Scanner{
    source : String,
    start : usize,
    current : usize,
    line: usize,
}

pub struct Token{
    pub _type : TokenType,
    pub lexeme: String,
    pub line : usize
}

impl Scanner{
    pub fn new(source: String) -> Self{
        Scanner{
            source: source,
            start: 0,
            current: 0,
            line: 1
        }
    }

    pub fn scan_token(&mut self) -> Token{
        self.skip_white_space();

        self.start = self.current;

        if self.is_at_end() {return self.make_token(TokenType::TokenEof);}

        let curr_char : char = self.advance();

        if self.is_digit(curr_char) {return self.number();}

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
            '!' => {
                if self.matching('='){
                    return self.make_token(TokenType::TokenBangEquals);
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
            
            _ => return self.error_token("Unexpected Character.") 
        }
    }

    // If the current character is the desired one, we advance and return true. Otherwise, we return false to indicate it wasn’t matched. 
    fn matching(&mut self, expected : char) -> bool{
        if self.is_at_end() {return false;}
        let curr_char_index : usize = self.current;
        // If the char at the current index position is not the same as the char expected, return false
        if self.source.char_at(curr_char_index) != expected {return false;}
        else{
            self.current += 1;
            true
        }
    }

    // consumes the current character and returns it
    fn advance(&mut self) -> char {
        let curr_source_char: char = self.source.char_at(self.current);
        self.current += 1;
        return curr_source_char;
    }

    // Check if we have reached the end of our source string
    fn is_at_end(&mut self) -> bool{
        return self.current == self.source.len();
    }

    // Checks char at curr position. Doesn't increase the curr
    fn peek(&mut self) -> char{
        self.source.char_at(self.current)
    }

    // This is like peek() but for one character past the current one
    fn peek_next(&mut self) -> Option<char>{
        if self.is_at_end() {return None;}
        else{
            return Some(self.source.char_at(self.current + 1));
        } 
    }

    fn is_digit(&self, c : char) -> bool{
        return c >= '0' && c <= '9'
    }

    // Gets the token for a number
    fn number(&mut self) -> Token{
        let mut peek: char = self.peek();
        while self.is_digit(peek) {self.advance(); peek = self.peek();}
        // Needed to get the next char due to Option<char>
        if let Some(next_char) = self.peek_next(){
            // Look for fractional part
            if self.peek() == '.' && self.is_digit(next_char){
                // consume the "."
                self.advance();

                peek = self.peek();
                while self.is_digit(peek) {self.advance(); peek = self.peek();}
            }
        }
        return self.make_token(TokenType::TokenNumber);
    }

    // Gets the token for a string
    fn string(&mut self) -> Token{
        while self.peek() != '"' && self.is_at_end() {
            if self.peek() == '\n'{self.line += 1;}
            self.advance();
        }

        if self.is_at_end() {return self.error_token("Unterminated String.");}

        // The closing quote
        self.advance();

        return self.make_token(TokenType::TokenString);
    }   

    // This advances the scanner past any leading whitespace
    fn skip_white_space(&mut self){
        loop{
            let curr_char : char = self.peek();
            match curr_char{
                ' ' => {self.advance();},
                '\r' => {self.advance();},
                '\t' => {self.advance();},
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    if let Some(char) = self.peek_next(){
                        if char == '/'{
                            // A comment goes until the end of the line
                            while self.peek() != '\n' && !self.is_at_end() {
                                self.advance();
                            }
                        }
                    } else{
                        return;
                    }
                }
                _ => return
            }
        }
    }

    // Makes the token, with the corresponding 
    fn make_token(&self, _type : TokenType) -> Token{
        Token { 
            _type: _type,
            lexeme: self.source.substring(self.start, self.current), 
            line: self.line 
        }
    }

    fn error_token(&self, message : &str) -> Token{
        Token{
            _type: TokenType::TokenError,
            lexeme: message.to_string(),
            line: self.line
        }
    }
}

trait StringUtils{
    // Trait and implementation for a method for String that returns
    // a substring, which begins at the specified begin_index and extends
    // to the character at index end_index - 1
    fn substring(&self, begin_index: usize, end_index: usize) -> Self;
    // Gets the character in a position
    fn char_at(&mut self, index_pos: usize) -> char;
}


impl StringUtils for String{
    fn substring(&self, begin_index: usize, end_index: usize) -> Self {
        if begin_index + (end_index - begin_index) > self.len(){
            panic!("substring(): index out of bounds");
        }
        self.chars().skip(begin_index).take(end_index - begin_index).collect()
    }

    fn char_at(&mut self, index_pos: usize) -> char {
        let curr_source_char : char =  match self.chars().nth(index_pos){
            Some(x) => x,
            None => {
                println!("advance(): char not accessible by index. Returning empty space");
                ' '
            }
        };
        return curr_source_char;
    }
}



