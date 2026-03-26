use std::cell::Cell;
use rand;
use crate::classical::ClassicalRegister;
use crate::ket::{self, Ket};

struct QuantumRegister
{
    width: usize,
    collapsed: Cell<bool>,
    ket: Ket,
}

impl QuantumRegister
{
    fn new(width: usize, initial: &ClassicalRegister) -> QuantumRegister
    {
        assert_eq!(width, initial.width());
        QuantumRegister { width, collapsed: Cell::new(false), ket: ket::from_classical(initial) }
    }

    fn collapse(&mut self) -> ClassicalRegister 
    {
        assert_eq!(false, self.collapsed.get());

        self.collapsed = Cell::new(true);

        let sample = rand::random::<f64>() % 1.0;
        let mut cumulative = 0f64;

        for (state, coefficient) in self.ket.iter().enumerate()
        {
            cumulative += coefficient.norm_sqr();

            if sample < cumulative
            {
                return ClassicalRegister::from_state(self.width, state as u32)
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
    assert!(ket::is_classical(&r.ket));
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