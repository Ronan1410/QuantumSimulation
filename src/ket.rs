extern crate float_cmp;

use self::float_cmp::ApproxEqUlps;

use num::Complex;
use num::traits::{One, Zero};

use crate::classical::ClassicalRegister;

pub type Ket = Vec<Complex<f64>>;

pub fn is_valid(ket: &Ket) -> bool
{
    let mut sample_space_sum = 0f64;

    for coefficient in ket
    {
        sample_space_sum += coefficient.norm_sqr()
    }

    sample_space_sum.approx_eq_ulps(&1.0f64, 10)
}

pub fn is_classical(ket: &Ket) -> bool
{
    assert!(is_valid(ket));

    let mut zeros = 0;
    let mut ones = 0;
    let mut others = 0;

    for coefficient in ket
    {
        if coefficient.is_zero()
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

    return 1 == ones && 0 == others;
}

pub fn from_classical(register: &ClassicalRegister) -> Ket
{
    let coefficients = 2usize.pow(register.width() as u32);
    let mut ket: Ket = vec![Complex::zero(); coefficients - 1];

    ket.insert(register.state() as usize, Complex::new(1.0, 0.0));

    ket
}

#[test]
fn valid_test()
{
    let valid: Ket = vec![Complex::zero(), Complex::zero(), Complex::one()];
    let invalid: Ket = vec![Complex::new(0.5, 0.0), Complex::new(0.0, 0.5)];

    assert!(is_valid(&valid));
    assert_eq!(false, is_valid(&invalid));
}

#[test]
fn classical_test()
{
    let sqrt2inv = 2.0f64.sqrt().recip();

    let classical: Ket = vec![Complex::zero(), Complex::zero(), Complex::one()];
    let nonclassical1: Ket = vec![Complex::new(sqrt2inv, 0.0), Complex::new(sqrt2inv, 0.0)];
    let nonclassical2: Ket = vec![Complex::new(0.5, 0.5), Complex::new(0.5, 0.5)];

    assert!(is_classical(&classical));
    assert_eq!(false, is_classical(&nonclassical1));
    assert_eq!(false, is_classical(&nonclassical2));
}

#[test]
fn from_calssical_test()
{
    let r: ClassicalRegister = ClassicalRegister::new(vec![0, 1]);

    let ket: Ket = from_classical(&r);

    assert!(is_classical(&ket));
    assert_eq!(vec![Complex::zero(), Complex::zero(), Complex::one(), Complex::zero()], from_classical(&r));
}