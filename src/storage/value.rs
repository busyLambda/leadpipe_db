
#[derive(Debug)]
pub enum Value {
    UInt8([u8; 1]),
    UInt16([u8; 2]),
    UInt32([u8; 4]),
    UInt64([u8; 8]),
    UInt128([u8; 16]),
    Int8([u8; 1]),
    Int16([u8; 2]),
    Int32([u8; 4]),
    Int64([u8; 8]),
    Int128([u8; 16]),
    String(Vec<u8>),
    Bool([u8; 1]),
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            &Self::UInt8(bytes) => {
                u8::from_be_bytes(bytes).to_string()
            },
            &Self::UInt16(bytes) => {
                u16::from_be_bytes(bytes).to_string()
            },
            &Self::UInt32(bytes) => {
                u32::from_be_bytes(bytes).to_string()
            },
            &Self::UInt64(bytes) => {
                u64::from_be_bytes(bytes).to_string()
            },
            &Self::UInt128(bytes) => {
                u128::from_be_bytes(bytes).to_string()
            },
            Self::String(bytes) => {
                String::from_utf8_lossy(&bytes).to_string()
            }
            _ => todo!()
        }
    }
}