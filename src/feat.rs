#[derive(Debug, Clone, Copy)]
#[repr(u8)]
// #[repr(transparent)]
pub enum Feature {
    Tall   = 0b00000001,
    Short  = 0b00000010,
    Red    = 0b00000100,
    Blue   = 0b00001000,
    Square = 0b00010000,
    Round  = 0b00100000,
    Hollow = 0b01000000,
    Solid  = 0b10000000,
}

impl Into<u8> for Feature {
    fn into(self) -> u8 {
        self as u8
    }
}