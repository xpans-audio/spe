#![no_std]
mod traits;
pub use traits::*;

/// Describes the cartesian axis or axes of the property updated by a message.
#[repr(u8)]
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub enum AxisCombo {
    X = 0,
    Y = 1,
    XY = 2,
    Z = 3,
    XZ = 4,
    YZ = 5,
    XYZ = 6,
}
impl AxisCombo {
    /// Moves or copies relevant values to the beginning of an array.
    fn rearrange_values<T: Copy>(&self, triplet: [T; 3]) -> [T; 3] {
        let mut triplet = triplet;
        match self {
            Self::Y | Self::YZ => triplet.rotate_left(1),
            Self::Z => triplet[0] = triplet[2],
            Self::XZ => triplet[1] = triplet[2],
            _ => (),
        }
        triplet
    }
    /// Attempts to convert a byte to an `AxisCombo`.
    pub fn try_from_byte(byte: u8) -> Option<Self> {
        let axis = match byte {
            0 => Self::X,
            1 => Self::Y,
            2 => Self::XY,
            3 => Self::Z,
            4 => Self::XZ,
            5 => Self::YZ,
            6 => Self::XYZ,
            _ => {
                return None;
            }
        };
        Some(axis)
    }
    /// Attempts to convert an array of `bool` to an AxisCombo. Each bool
    /// in the array is `true` if the corresponding axis has changed and is
    /// included in the message.
    pub fn from_changes(changes: [bool; 3]) -> Option<Self> {
        let mut byte = 0u8;
        for (i, b) in changes.iter().enumerate() {
            byte |= u8::from(*b) << i;
        }
        byte = byte.checked_sub(1)?;
        Self::try_from_byte(byte)
    }
    fn value_count(&self) -> usize {
        match self {
            Self::X | Self::Y | Self::Z => 1,
            Self::XY | Self::XZ | Self::YZ => 2,
            AxisCombo::XYZ => 3,
        }
    }
}

#[test]
#[cfg(test)]
fn axis_combo_change_conversion() {
    for i in 1u8..8 {
        let mut changes = [false; 3];
        for bit in 0..3 {
            changes[bit] = ((i >> bit as u8) & 1) != 0;
        }
        let axis = AxisCombo::from_changes(changes).unwrap();
        assert_eq!(axis as u8, i - 1)
    }
}

/// Describes the property updated by a message.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Property {
    Position(AxisCombo),
    Extent(AxisCombo),
}
impl Property {
    pub fn value_count(&self) -> usize {
        match self {
            Self::Position(axis) | Self::Extent(axis) => axis.value_count(),
        }
    }
}

/// A SPE message describing a change in properties at one point in time.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Message<T> {
    pub property: Property,
    /// Only values relevant to the property need to be stored.
    pub values: [T; 4],
}

impl<T: Default + Copy> Message<T> {
    /// Create a new SPE message using a Property and array of values.
    ///
    /// `N` must be no more than four.
    pub fn new<const N: usize>(property: Property, values: [T; N]) -> Self {
        let mut buf = [T::default(); 4];
        let value_count = property.value_count();
        let used_values = &values[..value_count];
        buf[..value_count].copy_from_slice(used_values);
        Self {
            property,
            values: buf,
        }
    }
    /// Create a new SPE position message.
    pub fn position(axis: &AxisCombo, position: [T; 3]) -> Message<T> {
        let values = axis.rearrange_values(position);
        Self::new(Property::Position(*axis), values)
    }
    /// Create a new SPE extent message.
    pub fn extent(axis: &AxisCombo, extent: [T; 3]) -> Message<T> {
        let values = axis.rearrange_values(extent);
        Self::new(Property::Extent(*axis), values)
    }
    pub fn pos_x(x: T) -> Message<T> {
        Self::new(Property::Position(AxisCombo::X), [x])
    }
    pub fn pos_y(y: T) -> Message<T> {
        Message::new(Property::Position(AxisCombo::Y), [y])
    }
    pub fn pos_z(z: T) -> Message<T> {
        Message::new(Property::Position(AxisCombo::Z), [z])
    }
    pub fn pos_xy(x: T, y: T) -> Message<T> {
        Message::new(Property::Position(AxisCombo::XY), [x, y])
    }
    pub fn pos_xz(x: T, z: T) -> Message<T> {
        Message::new(Property::Position(AxisCombo::XZ), [x, z])
    }
    pub fn pos_yz(y: T, z: T) -> Message<T> {
        Message::new(Property::Position(AxisCombo::YZ), [y, z])
    }
    pub fn pos_xyz(x: T, y: T, z: T) -> Message<T> {
        Message::new(Property::Position(AxisCombo::XYZ), [x, y, z])
    }
    pub fn ext_x(x: T) -> Message<T> {
        Message::new(Property::Extent(AxisCombo::X), [x])
    }
    pub fn ext_y(y: T) -> Message<T> {
        Message::new(Property::Extent(AxisCombo::Y), [y])
    }
    pub fn ext_z(z: T) -> Message<T> {
        Message::new(Property::Extent(AxisCombo::Z), [z])
    }
    pub fn ext_xy(x: T, y: T) -> Message<T> {
        Message::new(Property::Extent(AxisCombo::XY), [x, y])
    }
    pub fn ext_xz(x: T, z: T) -> Message<T> {
        Message::new(Property::Extent(AxisCombo::XZ), [x, z])
    }
    pub fn ext_yz(y: T, z: T) -> Message<T> {
        Message::new(Property::Extent(AxisCombo::YZ), [y, z])
    }
    pub fn ext_xyz(x: T, y: T, z: T) -> Message<T> {
        Message::new(Property::Extent(AxisCombo::XYZ), [x, y, z])
    }
    pub fn values(&self) -> &[T] {
        &self.values[..self.property.value_count()]
    }
}
