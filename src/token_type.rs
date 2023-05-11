
// Decided to put the TokenType in it's own mod.
// Looks less messy than all of it being in scanner.rs
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType{
    // Single-character tokens
    TokenLeftParen, TokenRightParen,
    TokenLeftBrace, TokenRightBrace,
    TokenComma, TokenDot, TokenSemicolon, 
    TokenSlash, TokenStar,
    // TokenTernary, 

    // One or two character tokens
    TokenBang, TokenBangEqual,
    TokenEqual, TokenEqualEqual,
    TokenGreater, TokenGreaterEqual,
    TokenLess, TokenLessEqual,
    TokenCarat, TokenCaratEqual,
    TokenPlus, TokenPlusEqual,
    TokenMinus, TokenMinusEqual,

    // Literals
    TokenIdentifier, TokenString, TokenNumber,

    // Keywords
    TokenAnd, TokenClass, TokenElse, TokenFalse, 
    TokenFor, TokenFun, TokenIf, TokenNil, TokenOr, 
    TokenPrint, TokenReturn, TokenSuper, TokenThis,
    TokenTrue, TokenVar, TokenWhile,

    // Miscellaneous
    TokenError, TokenEOF, Undefined
}