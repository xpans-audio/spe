use crate::{AxisCombo, Message, Property};

/// Sets the position of an audio source
pub trait SetPosition<V> {
    fn set_pos_x(&mut self, x: V);
    fn set_pos_y(&mut self, y: V);
    fn set_pos_z(&mut self, z: V);
    fn set_pos_xy(&mut self, x: V, y: V) {
        self.set_pos_x(x);
        self.set_pos_y(y);
    }
    fn set_pos_xz(&mut self, x: V, z: V) {
        self.set_pos_x(x);
        self.set_pos_z(z);
    }
    fn set_pos_yz(&mut self, y: V, z: V) {
        self.set_pos_y(y);
        self.set_pos_z(z);
    }
    fn set_pos_xyz(&mut self, x: V, y: V, z: V) {
        self.set_pos_x(x);
        self.set_pos_y(y);
        self.set_pos_z(z);
    }
}

/// Sets the extent of an audio source
pub trait SetExtent<V> {
    fn set_ext_x(&mut self, x: V);
    fn set_ext_y(&mut self, y: V);
    fn set_ext_z(&mut self, z: V);
    fn set_ext_xy(&mut self, x: V, y: V) {
        self.set_ext_x(x);
        self.set_ext_y(y);
    }
    fn set_ext_xz(&mut self, x: V, z: V) {
        self.set_ext_x(x);
        self.set_ext_z(z);
    }
    fn set_ext_yz(&mut self, y: V, z: V) {
        self.set_ext_y(y);
        self.set_ext_z(z);
    }
    fn set_ext_xyz(&mut self, x: V, y: V, z: V) {
        self.set_ext_x(x);
        self.set_ext_y(y);
        self.set_ext_z(z);
    }
}

/// For types that want to modify their properties based on SPE messages.
///
/// This trait is auto-implemented for types that implement
/// `SetPosition` and `SetExtent`.
pub trait ApplyMessage<T> {
    /// Change self's properties to that of the given message.
    fn apply_message(&mut self, message: Message<T>);
}
impl<T, V> ApplyMessage<V> for T
where
    T: SetPosition<V> + SetExtent<V>,
    V: Default + Copy,
{
    fn apply_message(&mut self, message: Message<V>) {
        match message.property {
            Property::Position(axis) => apply_pos(self, &axis, &message.values()),
            Property::Extent(axis) => apply_ext(self, &axis, &message.values()),
        }
    }
}

fn apply_pos<T, V>(target: &mut T, axis: &AxisCombo, values: &[V])
where
    T: SetPosition<V>,
    V: Copy,
{
    match axis {
        AxisCombo::X => target.set_pos_x(values[0]),
        AxisCombo::Y => target.set_pos_y(values[0]),
        AxisCombo::Z => target.set_pos_z(values[0]),
        AxisCombo::XY => target.set_pos_xy(values[0], values[1]),
        AxisCombo::XZ => target.set_pos_xz(values[0], values[1]),
        AxisCombo::YZ => target.set_pos_yz(values[0], values[1]),
        AxisCombo::XYZ => target.set_pos_xyz(values[0], values[1], values[2]),
    }
}

fn apply_ext<T, V>(target: &mut T, axis: &AxisCombo, values: &[V])
where
    T: SetExtent<V>,
    V: Copy,
{
    match axis {
        AxisCombo::X => target.set_ext_x(values[0]),
        AxisCombo::Y => target.set_ext_y(values[0]),
        AxisCombo::Z => target.set_ext_z(values[0]),
        AxisCombo::XY => target.set_ext_xy(values[0], values[1]),
        AxisCombo::XZ => target.set_ext_xz(values[0], values[1]),
        AxisCombo::YZ => target.set_ext_yz(values[0], values[1]),
        AxisCombo::XYZ => target.set_ext_xyz(values[0], values[1], values[2]),
    }
}
