use core::slice::IterMut;

use xpans_spe::{AxisCombo, Message, Property};

use crate::{
    message::{SYSEX_PREFIX, SYSEX_SUFFIX},
    midi::{WriteMidi, u16_to_u14},
    value::Value,
};

#[inline]
fn write_property(buf: &mut IterMut<u8>, property: Property) {
    match property {
        Property::Position(axis) => {
            write_single_byte(buf, 0);
            write_axis(buf, axis)
        }
        Property::Extent(axis) => {
            write_single_byte(buf, 1);
            write_axis(buf, axis)
        }
        _ => todo!(),
    }
}

#[inline]
fn write_axis(buf: &mut IterMut<u8>, axis_combo: AxisCombo) {
    let byte = axis_combo as u8;
    write_single_byte(buf, byte)
}

#[inline]
fn write_type<T>(buf: &mut IterMut<u8>) {
    let byte = size_of::<T>() as u8;
    write_single_byte(buf, byte)
}

#[inline]
fn write_id(writer: &mut IterMut<u8>, id: u16) {
    let id = u16_to_u14(id);
    id.write_midi(writer)
}

#[inline]
fn write_single_byte(buf: &mut IterMut<u8>, byte: u8) {
    buf.next().map(|dest| *dest = byte);
}

#[inline]
/// Writes a SPE MIDI message for the provided Source ID to the byte buffer and
/// returns the amount of bytes written.
pub fn write_message<T: Value>(buf: &mut [u8], message: &Message<T>, id: u16) -> usize {
    let mut iter = buf.iter_mut();
    write_single_byte(&mut iter, SYSEX_PREFIX);
    write_id(&mut iter, id);
    write_type::<T>(&mut iter);
    write_property(&mut iter, message.property);
    for value in message.values().iter() {
        value.write_midi(&mut iter);
    }
    write_single_byte(&mut iter, SYSEX_SUFFIX);
    let remaining = iter.len();
    buf.len() - remaining
}
