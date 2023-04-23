#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum Precedence {
    PrecNone = 0,
    PrecAssignment,   // =
    PrecOr,           // or
    PrecAnd,          // and
    PrecEquality,     // == !=
    PrecComparison,   // < > <= >=
    PrecTerm,         // + -
    PrecFactor,       // * /
    PrecUnary,        // ! -
    PrecCall,         // . ()
    PrecPrimary
}

impl From<usize> for Precedence {
    fn from(value: usize) -> Self {
        match value {
            0 => Precedence::PrecNone,
            1 => Precedence::PrecAssignment,
            2 => Precedence::PrecOr,
            3 => Precedence::PrecAnd,
            4 => Precedence::PrecEquality,
            5 => Precedence::PrecComparison,
            6 => Precedence::PrecTerm,
            7 => Precedence::PrecFactor,
            8 => Precedence::PrecUnary,
            9 => Precedence::PrecCall,
            10 => Precedence::PrecPrimary,
            _ => panic!("{value} can't be converted into Precedence")
        }
    }
}

impl Precedence {
    pub fn next(self) -> Self {
        if self == Precedence::PrecPrimary {
            panic!("There is no next precedence after PrecPrimary");
        }
        let curr_precedence: usize = self as usize;
        
        (curr_precedence + 1).into()
    }
    
    #[allow(dead_code)]
    pub fn previous(self) -> Self {
        if self == Precedence::PrecNone {
            panic!("There is no previous precedence before PrecNone");
        }
        let curr_precedence: usize = self as usize;
        
        (curr_precedence - 1).into()
    }
}