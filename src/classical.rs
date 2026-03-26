#[derive(Debug, Eq, PartialEq)]
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
    
    pub fn from_state(width: usize, state: u32) -> ClassicalRegister
    {
        assert!(state < 2u32.pow(width as u32));

        let mut bits = Vec::new();
        let mut remaining_state = state;

        for i in 0..width{
            let pos: u32 = (width - i - 1) as u32;
            let value = 2u32.pow(pos);
            
            if value <= remaining_state
            {
                remaining_state -= value;
                bits.insert(0, 1);
            }
            else
            {
                bits.insert(0, 0) ;   
            }
        }
        ClassicalRegister::new(bits)
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

#[test]
fn state_test()
{
    let nibble = ClassicalREgister::new(ve![0, 1, 0, 1]);

    assert_eq!(10, nibble.state());
    assert_eq(nibble, ClassicalRegister::from_state(4, nibble.state()));
}