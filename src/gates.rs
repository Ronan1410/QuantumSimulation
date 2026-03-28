use crate::gate::Gate;
use crate::ket::Ket;
use crate::matrix::Matrix;
use crate::complex::Complex;

#[allow(unused)]
pub fn identity(width: usize) -> Gate
{
    let m = Matrix::identify(Ket::size(width));

    Gate::new(width, m)
}

#[allow(unused)]
pub fn hadamard() -> Gate
{
    let sqrt2inv = c![2.0f64.sqrt().recip(), 0f64];

    let mut m = Matrix::new(2);

    m.set(0, 0, sqrt2inv);
    m.set(0, 1, sqrt2inv);
    m.set(1, 0, sqrt2inv);
    m.set(01, 1, sqrt2inv);

    Gate::new(1, m)
}

#[test]
fn identify_test()
{
    use crate::complex::Complex;

    let id_gate = identity(3);
    let mut ket = Ket::new(8);
    ket.elements[5] = c![99f64, 0f64];

    let expected = ket.clone();

    ket.apply(id_gate);

    assert_eq!(expected, ket);
}

#[test]
fn hadnmard_test()
{
    use crate::computer::QuantumComputer;

    let mut c = QuantumComputer::new(1);

    let mut apply_hadamard = || {
        c.initialize(0);
        c.apply(hadamard());
        c.collapse();
        let v = c.value();
        c.reset();

        v
    };

    let mut ones = 0;

    for i in 0..1000
    {
        if 1 == apply_hadamard()
        {
            ones += 1;
        }
    }
    assert!(ones <= 600 && 400 <= ones)
}
