use crate::complex::Complex;
use crate::gate::Gate;
use crate::matrix::Matrix;
use crate::{deutsch, gates};
use crate::computer::QuantumComputer;

#[allow(unused)]
#[derive(Debug, PartialEq)]
enum DeutschAlgorithmOutput
{
    Balanced,
    Constant
}

pub fn deutsch_gate(f: fn(i32) -> i32) -> Gate
{
    assert!(f(0) == 0 || f(0) == 1);
    assert!(f(1) == 0 || f(1) == 1);

    let mut m = Matrix::identify(4);
    
    let mut exchange = m_real![0, 1;
                                1, 0];

    if f(0) == 1
    {
        println!(">>>embedding");
        m.embed(&exchange, 0, 0);
    }

    if f(1) ==1
    {
        println!(">>>embedding");
        m.embed(&exchange, 2, 2);
    }
    Gate::new(2, m)
}

#[allow(unused)]
pub fn deutsch_algorithm(f: fn(i32) -> i32) -> DeutschAlgorithmOutput
{
    let mut c = QuantumComputer::new(2);
    c.initialize(1);
    c.apply(gates::hadamard(2));
    c.apply(deutsch_gate(f));
    c.apply(gates::hadamard(2));

    c.collapse();

    match c.value()
    {
        1 => DeutschAlgorithmOutput::Constant,
        3 => DeutschAlgorithmOutput::Balanced,
        _ => panic!("unknown error")
    }
}

#[allow(unused)]
fn deutsch_test()
{
    fn constant1(x: i32) -> i32
    {
        assert!(x == 0 || x == 1);

        0
    }

    fn constant2(x: i32) -> i32
    {
        assert!(x == 0 || x == 1);

        1
    }

    fn balanced1(x: i32) -> i32
    {
        assert!(x == 0 || x == 1);

        x
    }

    fn balanced2(x: i32) -> i32
    {
        assert!(x == 0 || x == 1);

        1 - x
    }

    assert_eq!(DeutschAlgorithmOutput::Constant, deutsch_algorithm(constant1));
    assert_eq!(DeutschAlgorithmOutput::Constant, deutsch_algorithm(constant2));
    assert_eq!(DeutschAlgorithmOutput::Balanced, deutsch_algorithm(balanced1));
    assert_eq!(DeutschAlgorithmOutput::Balanced, deutsch_algorithm(balanced2));

}