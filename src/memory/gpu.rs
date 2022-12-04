use super::MemoryType;



pub struct Gpu{

}

impl MemoryType for Gpu {
    fn read_byte(&self,addr: u16)->u8 {
        0
    }

    fn write_byte(&mut self,addr: u16, val: u8) {
        
    }
}

impl Gpu {
    pub fn new()->Gpu{
        Gpu {  }
    }
}