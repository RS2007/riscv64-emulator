use crate::dram::Dram;

pub struct Bus {
    dram: Dram,
}

pub const DRAM_BASE: usize = 0x80000000;

impl Bus {
    pub fn new(dram: Dram) -> Self {
        return Bus { dram };
    }

    pub fn load(&self, addr: usize, size: usize) -> Result<u64, ()> {
        if addr >= DRAM_BASE {
            return self.dram.load(addr, size);
        }
        // Throws for now, will have to change it to accomodate UART and other peripherals
        Err(())
    }
    pub fn store(&mut self, addr: usize, size: usize, value: u64) -> Result<(), ()> {
        if addr <= DRAM_BASE {
            self.dram.store(addr, size, value);
            return Ok(());
        }
        // Throws for now, will have to change it to accomodate UART and other peripherals
        Err(())
    }
}
