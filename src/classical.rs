pub struct ClassicalRegister
{
    bits:Vec<u8>
}

impl ClassicalRegister
{
    pub fn new(bits: Vec<u8>) -> ClassicalRegister
    {
        for bit in &bits
        {
            assert!(0 == *bit || 1 == *bit);
        }
        ClassicalRegister
        {
            bits: bits
        }
    }

    pub fn width(&self) -> usize
    {
        self.bits.len()
    }
    pub fn zeroed(width: usize) -> ClassicalRegister
    {
        ClassicalRegister::new(vec![0; width])
    }

    pub fn state(&self) -> u32
    {
        let mut state = 0u32;

        for (pos, bit) in self.bits.iter().enumerate()
        {
            if 0u8 != *bit
            {
                state += 2u32.pow(pos as u32);
            }
        }
        state
    }
}