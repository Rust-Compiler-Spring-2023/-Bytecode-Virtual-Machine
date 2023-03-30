use std::fmt::{Display, Formatter, Error};

pub type number = f64;

#[derive(Clone, PartialEq)]
pub enum Value{
    Bool(bool),
    Number(number),
    String(String),
    Nil
}

// Convert bool to Value::Bool(bool)
impl From<bool> for Value{
    fn from(_bool: bool) -> Self{
        Value::Bool(_bool)
    }
}

// Convert number to Value::Number(number)
impl From<number> for Value{
    fn from(_number: number) -> Self {
        Value::Number(_number)
    }
}

// Convert String to Value::String(String)
impl From<String> for Value{
    fn from(_string: String) -> Self {
        Value::String(_string)
    }
}

// Convert Value::Bool(bool) to bool
impl From<Value> for bool{
    fn from(_value: Value) -> Self {
        match _value{
            Value::Bool(_bool) => _bool,
            _ => panic!()
        }
    }
}

// Convert Value::Number(number) to number
impl From<Value> for number{
    fn from(_value: Value) -> Self {
        match _value{
            Value::Number(_number) => _number,
            _ => panic!()
        }
    }
}

// Convert Value::String(_string) to String
impl From<Value> for String{
    fn from(_value: Value) -> Self {
        match _value{
            Value::String(_string) => _string,
            _ => panic!()
        }
    }
}

impl Display for Value{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", match self{
            Value::Number(_number) => _number.to_string(),
            Value::Bool(_bool) => _bool.to_string(),
            Value::String(_string) => _string.to_string(),
            Value::Nil => "nil".to_string()
        })
    }
}

pub fn is_number(_value : Value) -> bool{
    if let Value::Number(_num) = _value{
        return true;
    }
    else{
        false
    }
}

pub fn is_nil(_value: Value) -> bool {
    if let Value::Nil = _value {
        return true;
    }
    else {false}
}

pub fn is_bool(_value: Value) -> bool {
    if let Value::Bool(_bool) = _value{
        return true;
    }
    false
}

impl Value{    
    // If the Value is False or Nil return false, else return true
    pub fn is_falsey(&self) -> bool{
        match self{
            Value::Bool(_bool) => *_bool,
            Value::Nil => false,
            _ => true
        }
    }
}