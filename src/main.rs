#[macro_use]
extern crate bigflags;

bigflags! {
    struct Spices: u32 {
        const SALT: 0b0000_0001;
        const PEPPER: 0b0000_0010;
        const CHILI = 0b0000_0100;
        const SAFFRON = 0b0000_1000;
        const ALL = Self::SALT.bits
        | Self::PEPPER.bits
        | Self::CHILI.bits
        | Self::SAFFRON.bits;
    }
}

impl Spices {

}

fn main() {

}