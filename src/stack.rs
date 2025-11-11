pub struct Stack {
    sp: u8,
    stack: [u16; 16]
}

impl Stack {
    pub fn new() -> Self {
        Self {
            sp: 0,
            stack: [0; 16]
        }
    }

    pub fn subroutine(&mut self, pc: u16) {
        self.stack[self.sp as usize] = pc;
        self.sp += 1;
    }

    pub fn return_subroutine(&mut self) -> u16 {
        self.sp -= 1;
        let last_addr = self.stack[self.sp as usize];
        self.stack[self.sp as usize] = 0;

        last_addr
    }
}