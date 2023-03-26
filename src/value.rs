#[derive(Clone, Copy, PartialEq)]
pub enum ValueType{
    ValBool,
    ValNil,
    ValNumber
}

#[derive(Clone, Copy)]
pub union As {
    pub boolean: bool,
    pub number: f64
}

#[derive(Clone, Copy)]
pub struct Value{
    pub _type: ValueType,
    pub _as: As
}

pub fn is_bool(value: Value) -> bool{
    value._type == ValueType::ValBool
}

pub fn is_nil(value: Value) -> bool{
    value._type == ValueType::ValNil
}

pub fn is_number(value: Value) -> bool{
    value._type == ValueType::ValNumber
}

pub fn as_bool(value: Value) -> bool{
    unsafe{value._as.boolean}
}

pub fn as_number(value: Value) -> f64{
    unsafe{value._as.number}
}

pub fn bool_val(value: bool) -> Value{
    Value{_type: ValueType::ValBool, _as: As { boolean: value }}
}

pub fn nil_val() -> Value{
   Value{_type:ValueType::ValNil, _as : As { number: 0.0}}
}

pub fn number_val(value: f64) -> Value{
   Value{_type:ValueType::ValNumber, _as:As { number: value }}
}

#[derive(Clone)]
pub struct ValueArray {
    pub values : Vec<Value>
}

impl ValueArray {
    pub fn new() -> Self {
        ValueArray{
            values : Vec::new()
        }
    }

    pub fn write_value_array(&mut self, value: Value) {
        self.values.push(value);
    }

    pub fn free_value_array(&mut self) {  // TODO: check
        self.values = Vec::new();
    }
}

pub fn print_value(value: Value) {
    // unsafe{
    //     match value._as{
    //         As{boolean} => print!("{}", boolean),
    //         As{number} => print!("{}", number)
    //     }
    // }

    print!("{}", as_number(value));
}