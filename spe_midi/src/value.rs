use num_traits::AsPrimitive;

use crate::midi::{ReadMidi, WriteMidi};

#[allow(private_bounds)]
pub trait Value: Sealed {}

trait Sealed
where
    Self: FromValue<f32> + FromValue<f64> + ReadMidi + WriteMidi + Default + Copy,
{
}

impl Sealed for f32 {}
impl Sealed for f64 {}
impl<T> Value for T where T: Sealed {}

pub trait FromValue<V> {
    fn from_value(value: V) -> Self;
}

impl<T, V> FromValue<V> for T
where
    T: 'static + Copy,
    V: 'static + AsPrimitive<T> + Copy,
{
    #[inline]
    fn from_value(value: V) -> Self {
        value.as_()
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum ValueType {
    Float16,
    Float32,
    Float64,
    Float128,
}

impl ValueType {
    #[inline]
    pub fn try_from_byte(byte: u8) -> Option<ValueType> {
        match byte {
            2 => Some(ValueType::Float16),
            4 => Some(ValueType::Float32),
            8 => Some(ValueType::Float64),
            16 => Some(ValueType::Float128),
            _ => None,
        }
    }
}

#[cfg(test)]
#[test]
fn type_enum_works() {
    let s16 = size_of::<u16>() as u8;
    let b16 = ValueType::try_from_byte(s16);
    assert_eq!(Some(ValueType::Float16), b16);
    let s32 = size_of::<f32>() as u8;
    let b32 = ValueType::try_from_byte(s32);
    assert_eq!(Some(ValueType::Float32), b32);
    let s64 = size_of::<f64>() as u8;
    let b64 = ValueType::try_from_byte(s64);
    assert_eq!(Some(ValueType::Float64), b64);
    let s128 = size_of::<u128>() as u8;
    let b128 = ValueType::try_from_byte(s128);
    assert_eq!(Some(ValueType::Float128), b128);

    for i in 1..128 {
        let i = (i * 2) - 1;
        let v = ValueType::try_from_byte(i);
        assert_eq!(None, v)
    }
}
