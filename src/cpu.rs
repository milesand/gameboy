/// Register set in gameboy CPU.
pub struct Registers {
    af: U8Pair,
    bc: U8Pair,
    de: U8Pair,
    hl: U8Pair,
    sp: u16,
    pc: u16,
}

/// Flag bits.
#[repr(u8)]
pub enum Flag {
    Z = 0b1000_0000,
    N = 0b0100_0000,
    H = 0b0010_0000,
    C = 0b0001_0000,
}

/// A type-pun for [u8; 2] and u16.
///
/// General registers in the Gameboy CPU are structured in a way that it may be
/// interpreted as a 16 bit register, or two 8 bit registers, simultaneously.
/// This means writing to either of the 8 bit registers change the value of the
/// 16 bit register, and vice versa. This is awkward to model in safe Rust; The
/// best one can do is probably a wrapper around u16, with getter/setter for
/// the 8-bit registers which do bitwise operations under the hood.
/// Fortunately, unsafe union is a thing in Rust, and we can use it to present
/// a nicer (and hopefully still safe) API.
#[repr(C)]
union U8Pair {
    pair: [u8; 2],
    whole: u16,
}

impl U8Pair {
    fn from_u16(value: u16) -> Self {
        U8Pair { whole: value }
    }

    fn as_u16(&mut self) -> &mut u16 {
        unsafe { &mut self.whole }
    }
}

#[cfg(target_endian = "little")]
impl U8Pair {
    fn as_hi(&mut self) -> &mut u8 {
        unsafe { &mut self.pair[1] }
    }

    fn as_lo(&mut self) -> &mut u8 {
        unsafe { &mut self.pair[0] }
    }
}

#[cfg(target_endian = "big")]
impl U8Pair {
    fn as_hi(&mut self) -> &mut u8 {
        unsafe { &mut self.pair[0] }
    }

    fn as_lo(&mut self) -> &mut u8 {
        unsafe { &mut self.pair[1] }
    }
}

impl Registers {
    /// Returns a reference to the `a` register.
    pub fn a(&mut self) -> &mut u8 {
        self.af.as_hi()
    }

    /// Returns a reference to the `b` register,
    /// high byte of the `bc` register.
    pub fn b(&mut self) -> &mut u8 {
        self.bc.as_hi()
    }

    /// Returns a reference to the `c` register,
    /// low byte of the `bc` register.
    pub fn c(&mut self) -> &mut u8 {
        self.bc.as_lo()
    }

    /// Returns a reference to the `d` register,
    /// high byte of the `de` register.
    pub fn d(&mut self) -> &mut u8 {
        self.de.as_hi()
    }

    /// Returns a reference to the `e` register,
    /// low byte of the `de` register.
    pub fn e(&mut self) -> &mut u8 {
        self.de.as_lo()
    }

    /// Returns a reference to the `h` register,
    /// high byte of the `hl` register.
    pub fn h(&mut self) -> &mut u8 {
        self.hl.as_hi()
    }

    /// Returns a reference to the `l` register,
    /// low byte of the `hl` register.
    pub fn l(&mut self) -> &mut u8 {
        self.hl.as_lo()
    }

    /// Returns a reference to the `bc` register,
    /// comprised of `b` high byte and `c` low byte.
    pub fn bc(&mut self) -> &mut u16 {
        self.bc.as_u16()
    }

    /// Returns a reference to the `de` register,
    /// comprised of `d` high byte and `e` low byte.
    pub fn de(&mut self) -> &mut u16 {
        self.de.as_u16()
    }

    /// Returns a reference to the `hl` register,
    /// comprised of `h` high byte and `l` low byte.
    pub fn hl(&mut self) -> &mut u16 {
        self.hl.as_u16()
    }

    /// Set the corresponding flag in the flag register to the given value.
    pub fn set_flag(&mut self, flag: Flag, value: bool) {
        let flag = flag as u8;
        if value {
            *self.af.as_lo() |= flag;
        } else {
            *self.af.as_lo() &= !flag;
        }
    }

    /// Get the corresponding flag from the flag register.
    pub fn get_flag(&mut self, flag: Flag) -> bool {
        let flag = flag as u8;
        *self.af.as_lo() & flag == flag
    }
}
