use core::slice::Iter;

use xpans_spe::{AxisCombo, Message, Property};

use crate::{
    message::SYSEX_PREFIX,
    midi::{ReadMidi, u14_to_u16},
    value::{Value, ValueType},
};

#[inline]
fn read_axis(buf: &mut Iter<u8>) -> Option<AxisCombo> {
    let axis = AxisCombo::try_from_byte(*buf.next()?)?;
    Some(axis)
}

#[inline]
fn read_property(buf: &mut Iter<u8>) -> Option<Property> {
    let prop_byte = *buf.next()?;
    match prop_byte {
        0 => {
            let axis = read_axis(buf)?;
            let prop = Property::Position(axis);
            Some(prop)
        }
        1 => {
            let axis = read_axis(buf)?;
            let prop = Property::Extent(axis);
            Some(prop)
        }
        _ => todo!(),
    }
}

#[inline]
fn read_value<T: Value>(buf: &mut Iter<u8>, typ: ValueType) -> Option<T> {
    match typ {
        ValueType::Float32 => Some(T::from_value(f32::read_midi(buf))),
        ValueType::Float64 => Some(T::from_value(f64::read_midi(buf))),
        _ => todo!(),
    }
}

#[inline]
fn read_type(buf: &mut Iter<u8>) -> Option<ValueType> {
    let typ = ValueType::try_from_byte(*buf.next()?)?;
    Some(typ)
}

#[inline]
fn read_id(reader: &mut Iter<u8>) -> Option<u16> {
    let id = u16::read_midi(reader);
    Some(u14_to_u16(id))
}

#[inline]
fn verify_start_byte(buf: &mut Iter<u8>) -> Option<()> {
    (*(buf.next()?) == SYSEX_PREFIX).then_some(())
}

/// Reads a SPE MIDI message from the byte buffer and returns the Source ID of
/// the message that was read. Returns None if an error was encountered.
#[inline]
pub fn read_message<T: Value>(buf: &[u8]) -> Option<(u16, Message<T>)> {
    let mut iter = buf.iter();
    verify_start_byte(&mut iter)?;
    let id = read_id(&mut iter)?;

    let typ = read_type(&mut iter)?;

    let property = read_property(&mut iter)?;

    let mut values = [T::default(); 4];
    for i in 0..property.value_count() {
        let value = read_value(&mut iter, typ)?;
        values[i] = value;
    }
    let msg = Message::new(property, values);
    Some((id, msg))
}
