/// Working ram, with eight 4KiB banks and bank switching functionality.
/// In original gameboy mode, selected_bank should always be 1;
/// in color mode, it may be 1 ~ 7.
pub struct Wram {
    ram: [[u8; 0x1000]; 8],
    selected_bank: usize,
}

impl Wram {
    /// Create a new working ram.
    pub fn new() -> Self {
        Wram {
            ram: [[0; 0x1000]; 8],
            selected_bank: 1,
        }
    }

    /// Read a byte from working ram.
    /// `addr` should be between 0xc000 (inclusive) and 0xe000 (exclusive),
    /// e.g. valid address into the wram in gameboy memory map.
    pub fn rb(&self, addr: u16) -> u8 {
        debug_assert!(0xc000 <= addr && addr < 0xe000);
        let addr = usize::from(addr);
        if addr < 0xd000 {
            self.ram[0][addr - 0xc000]
        } else {
            self.ram[self.selected_bank][addr - 0xd000]
        }
    }

    /// Write a byte into working ram. `addr` has the same restriction as the
    /// `rb` method.
    pub fn wb(&mut self, addr: u16, value: u8) {
        debug_assert!(0xc000 <= addr && addr < 0xe000);
        let addr = usize::from(addr);
        if addr < 0xd000 {
            self.ram[0][addr - 0xc000] = value;
        } else {
            self.ram[self.selected_bank][addr - 0xd000] = value;
        }
    }

    /// Read the SVBK register, assuming we're in CGB mode.
    ///
    /// SVBK register indicates which RAM bank is selected; This info is
    /// encoded in the lower 3 bits of return value, and the rest are 'unused'
    /// bits. According to AntonioND, these bits return 1. This implementation
    /// follows that behaviour.
    ///
    /// Since CGB mode is assumed, MMU should take care of the DMG case where
    /// the value read will always be 255.
    pub fn read_svbk(&self) -> u8 {
        ((self.selected_bank & 0b111) as u8) | !0b111
    }

    /// Write to the SVBK register, assuming we're in CGB mode.
    ///
    /// Writing into the SVBK register selects the RAM bank to be used. Only
    /// the lowest 3 bits of input are relevant, and the rest are ignored.
    ///
    /// MMU should take care of the DMG case; calling this in DMG mode is very
    /// likely to result in incorrect behaviour.
    pub fn write_svbk(&mut self, value: u8) {
        self.selected_bank = usize::from(value & 0b111);
        if self.selected_bank == 0 {
            self.selected_bank = 1;
        }
    }
}
