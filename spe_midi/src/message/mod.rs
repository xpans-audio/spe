mod read;
mod write;

pub use read::read_message;
pub use write::write_message;

const SYSEX_PREFIX: u8 = 0xf0;
const SYSEX_SUFFIX: u8 = 0xf7;
