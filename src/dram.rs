use crate::bus::DRAM_BASE;

pub struct Dram {
    pub buffer: Vec<u8>,
}

impl Dram {
    pub fn new(mut code: Vec<u8>) -> Self {
        let mut dram: Vec<u8> = vec![0; DRAM_BASE as usize];
        code.iter().for_each(|byte| {
            println!("{:#x}", byte);
        });
        dram.append(&mut code);
        dram.resize(dram.len() + 128 * 1024, 0);
        return Dram { buffer: dram };
    }
    pub fn len(&mut self) -> usize {
        return self.buffer.len();
    }

    pub fn load(&self, addr: usize, size: usize) -> Result<u64, ()> {
        return match size {
            8 => self.load8(addr),
            16 => self.load16(addr),
            32 => self.load32(addr),
            64 => self.load64(addr),
            _ => Err(()),
        };
    }
    pub fn store(&mut self, addr: usize, size: usize, value: u64) -> Result<(), ()> {
        match size {
            8 => self.store8(addr, value),
            16 => self.store16(addr, value),
            32 => self.store32(addr, value),
            64 => self.store64(addr, value),
            _ => Err(()),
        }
    }

    pub fn load8(&self, addr: usize) -> Result<u64, ()> {
        return Ok(self.buffer[addr] as u64);
    }

    pub fn load16(&self, addr: usize) -> Result<u64, ()> {
        return Ok(((self.buffer[addr + 1] as u64) << 8) | (self.buffer[addr] as u64));
    }

    pub fn load32(&self, addr: usize) -> Result<u64, ()> {
        if addr > self.buffer.len() - 1 {
            return Err(());
        }
        return Ok(((self.buffer[addr + 3] as u64) << 24)
            | ((self.buffer[addr + 2] as u64) << 16)
            | ((self.buffer[addr + 1] as u64) << 8)
            | (self.buffer[addr] as u64));
    }

    pub fn load64(&self, addr: usize) -> Result<u64, ()> {
        return Ok(((self.buffer[addr + 7] as u64) << 56)
            | ((self.buffer[addr + 6] as u64) << 48)
            | ((self.buffer[addr + 5] as u64) << 40)
            | ((self.buffer[addr + 4] as u64) << 32)
            | ((self.buffer[addr + 3] as u64) << 24)
            | ((self.buffer[addr + 2] as u64) << 16)
            | ((self.buffer[addr + 1] as u64) << 8)
            | (self.buffer[addr] as u64));
    }

    pub fn store8(&mut self, addr: usize, value: u64) -> Result<(), ()> {
        self.buffer[addr] = value.try_into().unwrap();
        return Ok(());
    }

    pub fn store16(&mut self, addr: usize, value: u64) -> Result<(), ()> {
        self.buffer[addr + 1] = (value << 8).try_into().unwrap();
        self.buffer[addr] = value.try_into().unwrap();
        return Ok(());
    }

    pub fn store32(&mut self, addr: usize, value: u64) -> Result<(), ()> {
        // 0x1f0
        self.buffer[addr + 3] = (value >> 24).try_into().unwrap();
        self.buffer[addr + 2] = ((value >> 16) & 0xff).try_into().unwrap();
        self.buffer[addr + 1] = ((value >> 8) & 0xff).try_into().unwrap();
        self.buffer[addr] = (value & 0xff).try_into().unwrap();
        println!("{}:{}:{}:{}", self.buffer[addr+3],self.buffer[addr+2],self.buffer[addr+1],self.buffer[addr]);
        return Ok(());
    }

    pub fn store64(&mut self, addr: usize, value: u64) -> Result<(), ()> {
        // Reverse byte order
        self.buffer[addr + 7] = value.try_into().unwrap();
        self.buffer[addr + 6] = (value >> 8).try_into().unwrap();
        self.buffer[addr + 5] = (value >> 16).try_into().unwrap();
        self.buffer[addr + 4] = (value >> 24).try_into().unwrap();
        self.buffer[addr + 3] = (value >> 32).try_into().unwrap();
        self.buffer[addr + 2] = (value >> 40).try_into().unwrap();
        self.buffer[addr + 1] = (value >> 48).try_into().unwrap();
        self.buffer[addr + 0] = (value >> 56).try_into().unwrap();
        return Ok(());
    }
}
