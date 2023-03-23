pub type Value = f64;

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
    print!("{value}");
}