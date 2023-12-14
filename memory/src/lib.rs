pub const MEMORY_SIZE: usize = 0x10000;

pub struct Memory {
    data: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        Memory { data: [0; MEMORY_SIZE] }
    }

    /// Reads a byte from memory at the given address.
    pub fn read(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    /// Writes a byte to memory at the given address.
    pub fn write(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }

    /// Helper function for the CPU only.
    /// 
    /// # Returns
    /// A 16-bit address at location `0xfffc` and `0xfffd`.
    pub fn get_reset_vector(&self) -> u16 {
        let low_byte: u8 = self.read(0xfffc);
        let high_byte: u8 = self.read(0xfffd);

        (high_byte as u16) << 8 | (low_byte as u16)
    }

    /// Helper function for the CPU only.
    /// 
    /// # Returns
    /// A 16-bit address at location `0xfffe` and `0xffff`.
    pub fn get_interrupt_vector(&self) -> u16 {
        let low_byte: u8 = self.read(0xfffe);
        let high_byte: u8 = self.read(0xffff);

        (high_byte as u16) << 8 | (low_byte as u16)
    }
}