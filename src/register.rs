use std::cell::Cell;

use crate::classical::ClassicalRegister;

struct QuantumRegister
{
    width: usize,
    collapsed: Cell<bool>,
}

impl QuantumRegister
{
    fn new(width: usize) -> QuantumRegister
    {
        QuantumRegister { width, collapsed: Cell::new(false) }
    }

    fn collapse(&mut self) -> ClassicalRegister 
    {
        assert_eq!(false, self.collapsed.get());

        self.collapsed = Cell::new(true);

        ClassicalRegister::new(vec![1, 0])
    }
}
#[test]
fn initialization_test()
{
    let r: QuantumRegister = QuantumRegister::new(5);

    assert_eq!(false, r.collapsed.get());
    assert_eq!(5, r.width)
}

#[test]
fn collaps_test()
{
    let mut r: QuantumRegister = QuantumRegister::new(5);
    r.collapse();

    assert!(r.collapsed.get());
}

#[test]
#[should_panic(expected = "assertion failed")]
fn double_collapse_test()
{
    let mut r: QuantumRegister = QuantumRegister::new(5);
    r.collapse();
    r.collapse();
}