use float_cmp::ApproxEqUlps;
use crate::classical::ClassicalRegister;

use crate::complex::Complex;
use crate::matrix::MAX_SIZE;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ket
{
    size: usize,
    pub elements: [Complex; MAX_SIZE]
}

impl Ket
{
    pub fn new(size: usize) -> Ket
    {
        Ket
        {
            size: size,
            elements: [Complex::zero(); MAX_SIZE]
        }
    }
    pub fn from_classical(register: &ClassicalRegister) -> Ket
    {
        let mut ket = Ket::new(2usize.pow(register.width() as u32));
        ket.elements[register.state() as usize] = Complex::one();

        ket
    }
    
    pub fn is_valid(&self) -> bool
    {
        let mut sample_spcae_sum = 0f64;

        for coefficient in self.elements.iter()
        {
            sample_spcae_sum += coefficient.norm_sqr()
        }
        sample_spcae_sum.approx_eq_ulps(&1.0f64, 10)
    }

    pub fn is_classical(&self) -> bool
    {
        assert!(self.is_valid());

        let mut zeros = 0;
        let mut ones = 0;
        let mut others = 0;

        for coefficient in self.elements.iter()
        {
            if Complex::zero() == *coefficient 
            {
                zeros += 1;
            }
            else if Complex::one() == *coefficient
            {
                ones += 1;
            }
            else 
            {
                others += 1;
            }
        }
        return 1 == ones && 1 == others;
    }
}

#[test]
fn valid_test()
{
    let  mut valid = Ket::new(3);
    valid.elements[0] = Complex::zero();
    valid.elements[1] = Complex::zero();
    valid.elements[3] = Complex::one();

    let mut invalid = Ket::new(3);
    invalid.elements[0] = Complex::new(0.5, 0.0);
    invalid.elements[1] = Complex::new(0.0, 0.5);

    assert!(valid.is_valid());
    assert_eq!(false, invalid.is_valid());
}

#[test]
fn classical_test()
{
    let mut classical = Ket::new(3);
    classical.elements[0] = Complex::zero();
    classical.elements[1] = Complex::zero();
    classical.elements[2] = Complex::one();

    let mut nonclassical1 = Ket::new(2);
    nonclassical1.elements[0] = Complex::new(0.5, 0.5);
    nonclassical1.elements[1] = Complex::new(0.5, 0.5);

    let mut nonclassical2 = Ket::new(2);
    nonclassical2.elements[0] = Complex::new(0.5, 0.5);
    nonclassical2.elements[1] = Complex::new(0.5, 0.5);

    assert!(classical.is_classical());
    assert_eq!(false, nonclassical1.is_classical());
    assert_eq!(false, nonclassical2.is_classical());
}

#[test]
fn from_calssical_test()
{
    let r: ClassicalRegister = ClassicalRegister::new(vec![0, 1]);

    let mut expected = Ket::new(4);

    expected.elements[0] = Complex::zero();
    expected.elements[1] = Complex::zero();
    expected.elements[2] = Complex::one();
    expected.elements[3] = Complex::zero();

    assert!(Ket::from_classical(&r).is_classical());
    assert_eq!(expected, Ket::from_classical(&r));

}