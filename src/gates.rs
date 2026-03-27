use crate::gate::Gate;
use crate::ket::Ket;
use crate::matrix::Matrix;

#[allow(unused)]
pub fn identity(width: usize) -> Gate
{
    let m: Matrix = Matrix::identify(Ket::size(width));

    Gate::new(width, m)
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
