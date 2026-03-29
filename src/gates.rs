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

#[allow(unused, trivial_numeric_casts)]
pub fn hadamard() -> Gate
{
    let sqrt2inv = 2.0f64.sqrt().recip();
    let mut m = m_real![sqrt2inv, sqrt2inv;
                            sqrt2inv, -sqrt2inv];

    Gate::new(1, m)
}

#[allow(unused)]
pub fn paulu_x() -> Gate
{
    let m = m_real![0, 1;
                            1, 0];

    Gate::new(1, m)
}

#[allow(unused)]
pub fn pauli_y() -> Gate
{
    let m = m![Complex::zero(),
                        -Complex::i();
                        Complex::i(),
                        Complex::zero()];

    Gate::new(1, m)
}

#[allow(unused)]
pub fn pauli_z() -> Gate
{
    let m = m_real![1, 0;
                            0, -1];

    Gate::new(1, m)
}
#[allow(unused)]
pub fn phase_shift(phi: f64) -> Gate
{
    let m = m![Complex::one(),
                        Complex::zero();
                        Complex::zero(),
                        Complex::new_euler(1f64, phi)];
    
    Gate::new(1, m)
}

pub fn swap() -> Gate
{
    let m = m_real![1, 0, 0, 0;
                            0, 0, 1, 0;
                            0, 1, 0, 0;
                            0, 0, 0, 1];

    Gate::new(2, m)
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

    for _ in 0..1000
    {
        if 1 == apply_hadamard()
        {
            ones += 1;
        }
    }
    assert!(ones <= 600 && 400 <= ones)
}

#[test]
fn pauli_x_test() 
{
    use crate::computer::QuantumComputer;

    let mut c = QuantumComputer::new(1);

    c.initialize(0);
    c.apply(pauli_y());
    c.collapse();
    assert_eq!(1, c.value());
    c.reset();

    c.initialize(1);
    c.apply(pauli_y());
    c.collapse();
    assert_eq!(0, c.value());
}

#[test]
fn pauli_y_test() 
{
    use crate::computer::QuantumComputer;

    let mut c = QuantumComputer::new(1);

    c.initialize(0);
    c.apply(pauli_y());
    c.collapse();
    assert_eq!(1, c.value());
    c.reset();

    c.initialize(1);
    c.apply(pauli_y());
    c.collapse();
    assert_eq!(0, c.value());
}

#[test]
fn pauli_z_test() 
{
    use crate::computer::QuantumComputer;

    let mut c = QuantumComputer::new(1);

    c.initialize(0);
    c.apply(pauli_z());
    c.collapse();
    assert_eq!(0, c.value());
    c.reset();

    c.initialize(1);
    c.apply(pauli_z());
    c.collapse();
    assert_eq!(1, c.value());
}

#[test]
fn phase_shift_test() {
    use crate::computer::QuantumComputer;

    let phi = 0.3f64;
    let mut c = QuantumComputer::new(1);

    // |0> goes to |0>
    c.initialize(0);
    c.apply(phase_shift(phi));
    c.collapse();
    assert_eq!(0, c.value());
    c.reset();

    // |1> goes to exp(i * phi)|1>
    c.initialize(1);
    c.apply(phase_shift(phi));
    c.collapse();
    assert_eq!(1, c.value());
}

#[test]
fn swap_test() 
{
    use crate::computer::QuantumComputer;

    let mut c = QuantumComputer::new(2);

    c.initialize(0);
    c.apply(swap());
    c.collapse();
    assert_eq!(0, c.value());
    c.reset();

    c.initialize(1);
    c.apply(swap());
    c.collapse();
    assert_eq!(2, c.value());
    c.reset();

    c.initialize(2);
    c.apply(swap());
    c.collapse();
    assert_eq!(1, c.value());
    c.reset();

    c.initialize(3);
    c.apply(swap());
    c.collapse();
    assert_eq!(3, c.value());
}