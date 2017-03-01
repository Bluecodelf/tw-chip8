use std::io;
use std::fmt;
use std::error::Error;

// We remove the first 512 bytes from the ram size, as those are reserved
const RAM_SIZE: usize = 0x1000;

#[derive(Debug)]
pub enum MemoryError {
    ReservedAddress(usize),
    UnmappedAddress(usize),
}

impl fmt::Display for MemoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MemoryError::ReservedAddress(addr) => write!(f, "address {} is reserved", addr),
            MemoryError::UnmappedAddress(addr) => write!(f, "address {} is out of bounds", addr),
        }
    }
}

impl Error for MemoryError {
    fn description(&self) -> &str {
        match *self {
            MemoryError::ReservedAddress(_) => "reserved address",
            MemoryError::UnmappedAddress(_) => "address out of bounds",
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

pub struct Memory {
    ram: [u8; RAM_SIZE],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            ram: [0u8; RAM_SIZE],
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        let rom_iter = rom.iter().take(RAM_SIZE - 0x200);
        let iter = self.ram.iter_mut().skip(0x200).zip(rom_iter);
        for (src, dst) in iter {
            *src = *dst;
        }
    }

    pub fn dump<T>(&self, out: &mut T) -> io::Result<usize> where T: io::Write {
        out.write(&self.ram)
    }

    pub fn read_byte(&self, addr: usize) -> Result<u8, MemoryError> {
        match addr {
            _ if addr < 0x200 => Err(MemoryError::ReservedAddress(addr)),
            _ if addr > 0xFFF => Err(MemoryError::UnmappedAddress(addr)),
            _ => Ok(self.ram[addr]),
        }
    }

    pub fn write_byte(&mut self, addr: usize, b: u8) -> Result<(), MemoryError> {
        match addr {
            _ if addr < 0x200 => Err(MemoryError::ReservedAddress(addr)),
            _ if addr > 0xFFF => Err(MemoryError::UnmappedAddress(addr)),
            _ => {
                self.ram[addr] = b;
                Ok(())
            },
        }
    }
}

