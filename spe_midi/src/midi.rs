use core::slice::{Iter, IterMut};

pub trait ReadMidi: Sized {
    fn read_midi(buf: &mut Iter<u8>) -> Self;
}

macro_rules! impl_midi_reader {
    ($t:ty, $decoder:ident) => {
        impl ReadMidi for $t {
            #[inline]
            fn read_midi(buf: &mut Iter<u8>) -> Self {
                const BYTES: usize = midi_bytes::<$t>();
                let mut int = 0;
                for (b, i) in buf.zip(0..BYTES) {
                    int |= <$t>::from(*b) << (7 * i);
                }
                let result = <$t>::from_le(int);
                result
            }
        }
    };
}

impl_midi_reader!(u16, midi_to_u16);
impl_midi_reader!(u32, midi_to_u32);
impl_midi_reader!(u64, midi_to_u64);
impl_midi_reader!(u128, midi_to_u128);

impl ReadMidi for f32 {
    #[inline]
    fn read_midi(buf: &mut Iter<u8>) -> Self {
        let bits = u32::read_midi(buf);
        f32::from_bits(bits)
    }
}
impl ReadMidi for f64 {
    #[inline]
    fn read_midi(buf: &mut Iter<u8>) -> Self {
        let bits = u64::read_midi(buf);
        f64::from_bits(bits)
    }
}

pub trait WriteMidi {
    fn write_midi(self, buf: &mut IterMut<u8>);
}

macro_rules! impl_midi_writer {
    ($t:ty, $encoder:ident) => {
        impl WriteMidi for $t {
            #[inline]
            fn write_midi(self, buf: &mut IterMut<u8>) {
                const BYTES: usize = midi_bytes::<$t>();
                let value = self.to_le();
                for (byte, i) in buf.zip(0..BYTES) {
                    *byte = ((value >> (7 * i)) & 127) as u8;
                }
            }
        }
    };
}

impl_midi_writer!(u16, u16_to_midi);
impl_midi_writer!(u32, u32_to_midi);
impl_midi_writer!(u64, u64_to_midi);
impl_midi_writer!(u128, u128_to_midi);

impl WriteMidi for f32 {
    #[inline]
    fn write_midi(self, buf: &mut IterMut<u8>) {
        self.to_bits().write_midi(buf)
    }
}
impl WriteMidi for f64 {
    #[inline]
    fn write_midi(self, buf: &mut IterMut<u8>) {
        self.to_bits().write_midi(buf)
    }
}
const U7_MASK: u16 = 2u16.pow(7) - 1;
const U14_MASK: u16 = 2u16.pow(14) - 1;

#[inline]
pub(crate) fn u14_to_u16(v: u16) -> u16 {
    let b1 = v & U7_MASK;
    let b2 = v & (U7_MASK << 8);
    (b2 >> 1) | b1
}

#[inline]
pub(crate) fn u16_to_u14(v: u16) -> u16 {
    let v = v & U14_MASK;
    let b1 = v & U7_MASK;
    let b2 = (v & !U7_MASK) << 1;
    b1 | b2
}

#[inline]
const fn midi_bytes<T>() -> usize {
    let native_bits = size_of::<T>() * 8;
    (native_bits / 7) + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn u16_round_trip() {
        for i in 0x00..0xFFFF {
            let mut buf = [0u8; midi_bytes::<u16>()];
            i.write_midi(&mut buf.iter_mut());
            let back = u16::read_midi(&mut buf.iter());
            assert_eq!(i, back);
        }
    }
    #[test]
    fn f32_round_trip() {
        for i in 0x00u16..0xFFFF {
            let i = f32::from(i);
            let mut buf = [0u8; midi_bytes::<f32>()];
            i.write_midi(&mut buf.iter_mut());
            let back = f32::read_midi(&mut buf.iter());
            assert_eq!(i, back);
        }
    }
    #[test]
    fn f64_round_trip() {
        for i in 0x00u16..0xFFFF {
            let i = f64::from(i);
            let mut buf = [0u8; midi_bytes::<f64>()];
            i.write_midi(&mut buf.iter_mut());
            let back = f64::read_midi(&mut buf.iter());
            assert_eq!(i, back);
        }
    }
    #[test]
    fn u32_round_trip() {
        for i in 0x00..0xFFFF {
            let i = i * 0xFFFF;
            let mut buf = [0u8; midi_bytes::<u32>()];
            i.write_midi(&mut buf.iter_mut());
            let back = u32::read_midi(&mut buf.iter());
            assert_eq!(i, back);
            assert_eq!(i, back);
        }
    }
    #[test]
    fn u64_round_trip() {
        for i in 0x00..0xFFFF {
            let i = i * 0xFFFFFFFF;
            let mut buf = [0u8; midi_bytes::<u64>()];
            i.write_midi(&mut buf.iter_mut());
            let back = u64::read_midi(&mut buf.iter());
            assert_eq!(i, back);
            assert_eq!(i, back);
        }
    }
    #[test]
    fn u128_round_trip() {
        for i in 0x0000..0xFFFF {
            let i = i * 0xFFFFFFFFFFFFFFFF;
            let mut buf = [0u8; midi_bytes::<u128>()];
            i.write_midi(&mut buf.iter_mut());
            let back = u128::read_midi(&mut buf.iter());
            assert_eq!(i, back);
            assert_eq!(i, back);
        }
    }
    const U14_MAX: u16 = 2u16.pow(14) - 1;
    #[test]
    fn u16_u14_round_trip() {
        for i in u16::MIN..U14_MAX {
            let conv = u16_to_u14(i);
            let back = u14_to_u16(conv);
            assert_eq!(i, back);
        }
    }

    #[test]
    fn u16_to_u14_valid() {
        const MASK_VALID_U14: u16 = 0b_01111111_01111111;
        for i in u16::MIN..U14_MAX {
            let conv = u16_to_u14(i);
            let masked = conv & MASK_VALID_U14;
            assert_eq!(conv, masked);
        }
    }
}
