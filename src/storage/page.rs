use std::{
    fs::File,
    io::{self, Error, Read, Seek, Write},
    os::unix::fs::FileExt,
};

use crate::{
    data::{_type::Type, object::Object},
    storage::value::Value,
};

use super::line::Line;

pub struct Page<'a> {
    index: usize,
    path: String,
    object: &'a Object<'a>,
    total: u64,
}

impl<'a> Page<'a> {
    pub fn new(index: usize, object: &'a Object<'a>, total: u64) -> Self {
        let path = format!("pages/{}-{}.page", object.name(), index);

        Self {
            index,
            path,
            object,
            total,
        }
    }

    /// Creates the actual file on disk for the page.
    pub fn create(&self) -> Result<File, std::io::Error> {
        File::create_new(&self.path)
    }

    pub fn read(&self, index: u64) -> Result<String, Error> {
        if index > self.total {
            panic!("Index shouldn't be larger than than the total size of the collection")
        }

        let offset = self.object.size_in_bits() * index;

        let mut line = Line::new();

        let mut file = File::open(&self.path)?;

        file.seek(io::SeekFrom::Start(offset))?;

        for (name, _type) in self.object.fields().into_iter() {
            let result: Result<(), Error>;
            let value = match _type {
                &Type::UInt8 | &Type::Int8 => {
                    let mut bytes = [0u8; 1];
                    result = file.read_exact(&mut bytes);

                    match _type {
                        &Type::UInt8 => Value::UInt8(bytes),
                        &Type::Int8 => Value::Int8(bytes),
                        _ => unreachable!(),
                    }
                }
                &Type::UInt16 | &Type::Int16 => {
                    let mut bytes = [0u8; 2];
                    result = file.read_exact(&mut bytes);

                    match _type {
                        &Type::UInt16 => Value::UInt16(bytes),
                        &Type::Int16 => Value::Int16(bytes),
                        _ => unreachable!(),
                    }
                }
                &Type::UInt32 | &Type::Int32 => {
                    let mut bytes = [0u8; 4];
                    result = file.read_exact(&mut bytes);

                    match _type {
                        &Type::UInt32 => Value::UInt32(bytes),
                        &Type::Int32 => Value::Int32(bytes),
                        _ => unreachable!(),
                    }
                }
                &Type::UInt64 | &Type::Int64 => {
                    let mut bytes = [0u8; 8];
                    result = file.read_exact(&mut bytes);

                    match _type {
                        &Type::UInt64 => Value::UInt64(bytes),
                        &Type::Int64 => Value::Int64(bytes),
                        _ => unreachable!(),
                    }
                }
                &Type::UInt128 | &Type::Int128 => {
                    let mut bytes = [0u8; 16];
                    result = file.read_exact(&mut bytes);

                    match _type {
                        &Type::UInt128 => Value::UInt128(bytes),
                        &Type::Int128 => Value::Int128(bytes),
                        _ => unreachable!(),
                    }
                }
                &Type::String(length) => {
                    let mut bytes = vec![0u8; length as usize];
                    result = file.read_exact(&mut bytes);

                    Value::String(bytes)
                }
                &Type::Bool => {
                    let mut bytes = [0u8; 1];
                    result = file.read_exact(&mut bytes);

                    Value::Bool(bytes)
                }
            };

            result.expect("Couldn't read field value from page.");

            line.add_field(name.to_string(), value);
        }

        Ok(line.to_string())
    }

    pub fn write(&mut self, line: Line) -> Result<String, Error> {
        let mut file = File::create(&self.path)?;

        // TODO: Use this and such.
        let offset = (self.total) * self.object.size_in_bits();
        file.seek(io::SeekFrom::Start(offset))?;

        for i in 0..line.fields().len() {
            let (name, value) = &line.fields()[i];
            let _type = &self.object.fields()[i].1;

            let result: io::Result<()>;

            match value {
                &Value::UInt8(bytes) => {
                    result = file.write_all(&bytes);
                }
                &Value::UInt16(bytes) => {
                    result = file.write_all(&bytes);
                }
                &Value::UInt32(bytes) => {
                    result = file.write_all(&bytes);
                }
                &Value::UInt64(bytes) => {
                    result = file.write_all(&bytes);
                }
                &Value::UInt128(bytes) => {
                    result = file.write_all(&bytes);
                }
                &Value::Int8(bytes) => {
                    result = file.write_all(&bytes);
                }
                &Value::Int16(bytes) => {
                    result = file.write_all(&bytes);
                }
                &Value::Int32(bytes) => {
                    result = file.write_all(&bytes);
                }
                &Value::Int64(bytes) => {
                    result = file.write_all(&bytes);
                }
                &Value::Int128(bytes) => {
                    result = file.write_all(&bytes);
                }
                Value::String(bytes) => match _type {
                    &Type::String(length) => {
                        let padding_length = length as usize - bytes.len();

                        let mut buf = bytes.clone();

                        if padding_length > 0 {
                            buf.extend(vec![0u8; padding_length]);
                        }

                        result = file.write_all(&buf)
                    }
                    _ => unreachable!(),
                },
                _ => todo!(),
            }
            result.expect(format!("Couldn't properly write value for {} field.", name).as_str())
        }

        self.total += 1;

        Ok(line.to_string())
    }
}

#[test]
fn test_page_write() {
    let mut users = Object::new("users");
    users.add_field(String::from("id"), Type::UInt128);
    users.add_field(String::from("username"), Type::String(255));

    let mut page = Page::new(0, &users, 0);

    page.create().unwrap();

    let mut line = Line::new();
    let bytes = 1u128.to_be_bytes();
    line.add_field(String::from("id"), Value::UInt128(bytes));
    let bytes = "JohnDoe69420".as_bytes().to_vec();
    line.add_field(String::from("username"), Value::String(bytes));

    println!("Line: {}", page.write(line).unwrap());

    println!("Line: {}", page.read(0).unwrap());
}
