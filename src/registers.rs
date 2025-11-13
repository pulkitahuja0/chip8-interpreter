pub struct Registers {
    v: [u8; 16],
    i: u16,
}

// TODO: Remove Result<> from set_v and get_v (no possibility of out of bounds register)
impl Registers {
    pub fn new() -> Self {
        Self { v: [0; 16], i: 0 }
    }

    // TODO: Add 12 bit wraparound check
    pub fn set_index_register(&mut self, i: u16) {
        self.i = i;
    }

    pub fn set_v(&mut self, register: u8, value: u8) -> Result<(), &'static str> {
        if register > 15 {
            return Err("Invalid register");
        }

        self.v[register as usize] = value;
        return Ok(());
    }

    pub fn get_v(&self, register: u8) -> Result<u8, &'static str> {
        if register > 15 {
            return Err("Invalid register");
        }

        return Ok(self.v[register as usize]);
    }

    pub fn get_index(&self) -> u16 {
        self.i
    }
}
