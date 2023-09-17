#![rustfmt::skip]
/// @generated rust packets from test.
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::convert::{TryFrom, TryInto};
use std::cell::Cell;
use std::fmt;
use std::result::Result;
use pdl_runtime::{DecodeError, EncodeError, Packet};
/// Private prevents users from creating arbitrary scalar values
/// in situations where the value needs to be validated.
/// Users can freely deref the value, but only the backend
/// may create it.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Private<T>(T);
impl<T> std::ops::Deref for Private<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FooData {
    a: u8,
    b: u32,
    c: u8,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Foo {
    #[cfg_attr(feature = "serde", serde(flatten))]
    foo: FooData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FooBuilder {
    pub a: u8,
    pub b: u32,
    pub c: u8,
}
impl FooData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 4
    }
    fn parse(bytes: &[u8]) -> Result<Self, DecodeError> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self, DecodeError> {
        if bytes.get().remaining() < 4 {
            return Err(DecodeError::InvalidLengthError {
                obj: "Foo",
                wanted: 4,
                got: bytes.get().remaining(),
            });
        }
        let chunk = bytes.get_mut().get_u32();
        let a = (chunk & 0x3) as u8;
        let b = ((chunk >> 2) & 0xff_ffff);
        let c = ((chunk >> 26) & 0x3f) as u8;
        Ok(Self { a, b, c })
    }
    fn write_to<T: BufMut>(&self, buffer: &mut T) -> Result<(), EncodeError> {
        if self.a > 0x3 {
            return Err(EncodeError::InvalidScalarValue {
                packet: "Foo",
                field: "a",
                value: self.a as u64,
                maximum_value: 0x3,
            });
        }
        if self.b > 0xff_ffff {
            return Err(EncodeError::InvalidScalarValue {
                packet: "Foo",
                field: "b",
                value: self.b as u64,
                maximum_value: 0xff_ffff,
            });
        }
        if self.c > 0x3f {
            return Err(EncodeError::InvalidScalarValue {
                packet: "Foo",
                field: "c",
                value: self.c as u64,
                maximum_value: 0x3f,
            });
        }
        let value = (self.a as u32) | (self.b << 2) | ((self.c as u32) << 26);
        buffer.put_u32(value);
        Ok(())
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        4
    }
}
impl Packet for Foo {
    fn encoded_len(&self) -> usize {
        self.get_size()
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        self.foo.write_to(buf)
    }
}
impl TryFrom<Foo> for Bytes {
    type Error = EncodeError;
    fn try_from(packet: Foo) -> Result<Self, Self::Error> {
        packet.encode_to_bytes()
    }
}
impl TryFrom<Foo> for Vec<u8> {
    type Error = EncodeError;
    fn try_from(packet: Foo) -> Result<Self, Self::Error> {
        packet.encode_to_vec()
    }
}
impl Foo {
    pub fn parse(bytes: &[u8]) -> Result<Self, DecodeError> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self, DecodeError> {
        let data = FooData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(foo: FooData) -> Result<Self, DecodeError> {
        Ok(Self { foo })
    }
    pub fn get_a(&self) -> u8 {
        self.foo.a
    }
    pub fn get_b(&self) -> u32 {
        self.foo.b
    }
    pub fn get_c(&self) -> u8 {
        self.foo.c
    }
    fn write_to(&self, buffer: &mut impl BufMut) -> Result<(), EncodeError> {
        self.foo.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.foo.get_size()
    }
}
impl FooBuilder {
    pub fn build(self) -> Foo {
        let foo = FooData {
            a: self.a,
            b: self.b,
            c: self.c,
        };
        Foo::new(foo).unwrap()
    }
}
impl From<FooBuilder> for Foo {
    fn from(builder: FooBuilder) -> Foo {
        builder.build().into()
    }
}
