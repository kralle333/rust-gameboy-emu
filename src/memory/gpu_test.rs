#[cfg(test)]
mod tests {
    use crate::memory::{Memory, gpu::TickMode};


    #[test]
    fn test_modes(){
        let mut memory = Memory::new();
        memory.reset();
        assert_eq!(memory.gpu.mode(),TickMode::HBLANK);
    }
}