
// Decided to put the TokenType in it's own mod.
// Looks less messy than all of it being in scanner.rs
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType{
    // Single-character tokens
    TokenLeftParen, TokenRightParen,
    TokenLeftBrace, TokenRightBrace,
    TokenComma, TokenDot, TokenMinus, TokenPlus,
    TokenSemicolon, TokenSlash, TokenStar,

    // One or two character tokens
    TokenBang, TokenBangEquals,
    TokenEqual, TokenEqualEqual,
    TokenGreater, TokenGreaterEqual,
    TokenLess, TokenLessEqual,

    // Literals
    TokenIdentifier, TokenString, TokenNumber,

    //keywords
    TokenAnd, TokenClass, TokenElse, TokenFalse, 
    TokenFor, TokenFun, TokenIf, TokenNil, TokenOr, 
    TokenPrint, TokenReturn, TokenSuper, TokenThis,
    TokenTrue, TokenVar, TokenWhile,

    TokenError, TokenEOF, Undefined
}