use std::cell::Cell;
use rand;
use crate::ket::Ket;
use crate::gate::Gate;

#[derive(Debug)]
pub struct QuantumRegister
{
    width: usize,
    collapsed: Cell<bool>,
    ket: Ket,
}

impl QuantumRegister
{
    pub fn new(width: usize, initial: &ClassicalRegister) -> QuantumRegister
    {
        assert_eq!(width, initial.width());
        QuantumRegister { width, collapsed: Cell::new(false), ket: Ket::from_classical(initial) }
    }

    pub fn apply(&mut self, gate:Gate)
    {
        assert_eq!(false, self.collapsed.get());
        assert_eq!(self.width, gate.width());

        self.ket.apply(gate);
    }

    pub fn collapse(&mut self) -> ClassicalRegister 
    {
        assert_eq!(false, self.collapsed.get());

        self.collapsed = Cell::new(true);

        let sample = rand::random::<f64>() % 1.0;
        let mut cumulative = 0f64;

        for (state, coefficient) in self.ket.elements.iter().enumerate()
        {
            cumulative += coefficient.norm_sqr();

            if sample < cumulative
            {
                return ClassicalRegister::from_state(self.width, state as u32);
            }
        }
        ClassicalRegister::from_state(self.width, 0)
    }
}
#[test]
fn initialization_test()
{
    let nibble = ClassicalRegister::zeroed(4);
    let r: QuantumRegister = QuantumRegister::new(4, &nibble);

    assert_eq!(false, r.collapsed.get());
    assert_eq!(4, r.width);
    assert!(&r.ket.is_classical());
}

#[test]
fn collaps_test()
{
    let nibble = ClassicalRegister::zeroed(4);
    let mut r: QuantumRegister = QuantumRegister::new(4, &nibble);
    let end: ClassicalRegister = r.collapse();

    assert_eq!(nibble, end);
    assert!(r.collapsed.get());
}

#[test]
#[should_panic(expected = "assertion failed")]
fn double_collapse_test()
{
    let nibble = ClassicalRegister::zeroed(4);
    let mut r: QuantumRegister = QuantumRegister::new(4, &nibble);
    r.collapse();
    r.collapse();
}

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
    pub fn from_int(width: usize, int: u32) -> ClassicalRegister
    {
        ClassicalRegister::from_state(width, int)
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

    pub fn to_int(&self) -> u32
    {
        self.state()
    }
}

#[test]
fn state_test()
{
    let nibble = ClassicalRegister::new(vec![0, 1, 0, 1]);

    assert_eq!(10, nibble.state());
    assert_eq!(nibble, ClassicalRegister::from_state(4, nibble.state()));
}