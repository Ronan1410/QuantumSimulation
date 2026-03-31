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
pub fn pauli_x() -> Gate
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

#[allow(unused)]
pub fn sqrt_swap() -> Gate
{
    let alpha_one = c!(0.5f64, 0.5f64);
    let alpha_two = c!(0.5f64, -0.5f64);

    let m = m![Complex::one(), Complex::zero(), Complex::zero(), Complex::zero();
                        Complex::zero(), alpha_one, alpha_two, Complex::zero();
                        Complex::zero(), alpha_two, alpha_one, Complex::zero();
                        Complex::zero(), Complex::zero(), Complex::zero(), Complex::one()];
    
    Gate::new(2, m)
}

#[allow(unused)]
pub fn controlled_not() -> Gate
{
    let m = m_real![1, 0, 0, 0;
                            0, 1, 0, 0;
                            0, 0, 0, 1;
                            0, 0, 1, 0];

    Gate::new(2, m)
}

macro_rules! test_gate 
{
    ($computer:expr, $gate:expr, $from:expr, $to:expr) => 
    {
        $computer.initialize($from);
        $computer.apply($gate);
        $computer.collapse();
        assert_eq!($to, $computer.value());
        $computer.reset();
    };
}

#[allow(unused)]
pub fn controlled(u: &Matrix) -> Gate
{
    assert_eq!(2, u.size());

    let mut m = m_real![1, 0, 0, 0;
                                0, 1, 0, 0;
                                0, 0, 0, 0;
                                0, 0, 0, 0];

    m.embed(&u, 2, 2);
    Gate::new(2, m)
}

#[allow(unused)]
pub fn controlled_x() -> Gate
{
    use crate::gate;
    controlled(pauli_x().matrix())
}

#[allow(unused)]
pub fn controlled_y() -> Gate
{
    controlled(pauli_y().matrix())
}

#[allow(unused)]
pub fn controlled_z() -> Gate
{
    controlled(pauli_z().matrix())
}

#[allow(unused)]
pub fn toffoli() -> Gate
{
    let mut m = Matrix::identify(8);

    let mut exchange = m_real![0, 1;
                                        1, 0];
    m.embed(&exchange, 6, 6);

    Gate::new(3, m)
}

#[allow(unused)]
pub fn fredkin() -> Gate
{
    let mut m = Matrix::identify(8);

    let mut exchange = m_real![0, 1;
                                        1, 0];
    m.embed(&exchange, 5, 5);

    Gate::new(3, m)
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

    test_gate!(c, pauli_x(), 0, 1);
    test_gate!(c, pauli_x(), 1, 0);
}

#[test]
fn pauli_y_test() 
{
    use crate::computer::QuantumComputer;

    let mut c = QuantumComputer::new(1);

    test_gate!(c, pauli_y(), 0, 1);
    test_gate!(c, pauli_y(), 1, 0);
}

#[test]
fn pauli_z_test() 
{
    use crate::computer::QuantumComputer;

    let mut c = QuantumComputer::new(1);

    test_gate!(c, pauli_z(), 0, 0);
    test_gate!(c, pauli_z(), 1, 1);
}

#[test]
fn phase_shift_test() {
    use crate::computer::QuantumComputer;

    let phi = 0.3f64;
    let mut c = QuantumComputer::new(1);

    test_gate!(c, phase_shift(phi), 0, 0);
    test_gate!(c, phase_shift(phi), 1, 1);
}

#[test]
fn swap_test() 
{
    use crate::computer::QuantumComputer;

    let mut c = QuantumComputer::new(2);

    test_gate!(c, swap(), 0, 0);
    test_gate!(c, swap(), 2, 1);
    test_gate!(c, swap(), 1, 2);
    test_gate!(c, swap(), 3, 3);
}

#[test]
pub fn sqrt_swap_test()
{
    use crate::computer::QuantumComputer;

    let mut c = QuantumComputer::new(2);

    test_gate!(c, sqrt_swap(), 0, 0);
    test_gate!(c, sqrt_swap(), 3, 3);
}

#[test]
pub fn controlled_not_test()
{
    use crate::computer::QuantumComputer;

    let mut c = QuantumComputer::new(2);

    test_gate!(c, controlled_not(), 0, 0);
    test_gate!(c, controlled_not(), 1, 1);
    test_gate!(c, controlled_not(), 2, 3);
    test_gate!(c, controlled_not(), 3, 2);
}

#[test]
pub fn controlled_test()
{
    let g = controlled(&m_real![0, 1; 1, 0]);

    assert_eq!(controlled_not(), g);
}

#[test]
fn toffoli_test()
{
    use crate::computer::QuantumComputer;

    let mut c = QuantumComputer::new(3);

    test_gate!(c, toffoli(), 0, 0);
    test_gate!(c, toffoli(), 1, 1);
    test_gate!(c, toffoli(), 2, 2);
    test_gate!(c, toffoli(), 6, 7);
    test_gate!(c, toffoli(), 7, 6);
}

#[test]
fn fredkin_test()
{
    use crate::computer::QuantumComputer;

    let mut c = QuantumComputer::new(3);

    test_gate!(c, fredkin(), 0, 0);
    test_gate!(c, fredkin(), 5, 6);
    test_gate!(c, fredkin(), 6, 5);
    test_gate!(c, fredkin(), 7, 7);
}