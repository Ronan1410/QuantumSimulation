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

    fn width(&self) -> usize
    {
        self.bits.len()
    }
}