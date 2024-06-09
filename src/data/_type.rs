pub enum Type {
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    UInt128,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    String(u16),
    Bool,
}

impl Type {
    pub fn size_in_bits(&self) -> u64 {
        match self {
            Self::UInt8 => 8,
            Self::UInt16 => 16,
            Self::UInt32 => 32,
            Self::UInt64 => 64,
            Self::UInt128 => 128,
            Self::Int8 => 8,
            Self::Int16 => 16,
            Self::Int32 => 32,
            Self::Int64 => 64,
            Self::Int128 => 128,
            Self::String(length) => (length * 8) as u64,
            Self::Bool => 1
        }
    }
}