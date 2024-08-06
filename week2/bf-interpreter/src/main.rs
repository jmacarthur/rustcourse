enum RawInstruction {
    IncPointer,
    DecPointer,
    IncByte,
    DecByte,
    Output,
    Input,
    JumpForward,
    JumpBackward,
}

impl RawInstruction {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '>' => Some(Self::IncPointer),
            '<' => Some(Self::DecPointer),
            '+' => Some(Self::IncByte),
            '-' => Some(Self::DecByte),
            '.' => Some(Self::Output),
            ',' => Some(Self::Input),
            '[' => Some(Self::JumpForward),
            ']' => Some(Self::JumpBackward),
            _ => None,
        }
    }
}

fn main() {}
