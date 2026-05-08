/*!
SPE MIDI allows for sending and receiving spatial properties of virtual audio
sources through MIDI SysEx, introducing potential for spatially-aware
processing and adaptive rendering in a digital audio workflow.
*/
#![no_std]
mod message;
mod midi;
mod value;
pub use crate::message::{read_message, write_message};

/// Re-export of xpans SPE
pub mod spe {
    pub use xpans_spe::*;
}
