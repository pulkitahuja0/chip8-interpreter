pub struct Stack {
    sp: u8,
    stack: [u16; 16],
}

impl Stack {
    pub fn new() -> Self {
        Self {
            sp: 0,
            stack: [0; 16],
        }
    }

    pub fn subroutine(&mut self, pc: u16) -> Result<(), &'static str> {
        self.stack[self.sp as usize] = pc;
        self.sp += 1;

        if self.sp > 15 {
            return Err("Stack overflow")
        }

        Ok(())
    }

    pub fn return_subroutine(&mut self) -> Result<u16, &'static str> {
        if self.sp == 0 {
            return Err("Stack underflow")
        }

        self.sp -= 1;
        let last_addr = self.stack[self.sp as usize];
        self.stack[self.sp as usize] = 0;

        Ok(last_addr)
    }
}
