use std::char::MAX;
use std::fmt;
use std::ops::Add;
use std::ops::Mul;

use crate::complex::Complex;
use crate::matrix;

pub const MAX_SIZE: usize = 32;
const MAX_ELEMENTS: usize = MAX_SIZE * MAX_SIZE;

pub type Vector = [Complex; MAX_SIZE];

pub struct Matrix
{
    pub size: usize,
    elements: [Complex; MAX_ELEMENTS]
}

impl Matrix
{
    pub fn new(size: usize) -> Matrix
    {
        assert!(size <= MAX_SIZE);

        Matrix
        {
            size: size,
            elements: [Complex::zero(); MAX_ELEMENTS]
        }
    }

    pub fn identify(size: usize) -> Matrix
    {
        assert!(size <= MAX_SIZE);

        let mut elements = [Complex::zero(); MAX_ELEMENTS];

        for i in 0..size
        {
            elements[i * MAX_SIZE + i] = Complex::one();
        }
        Matrix
        {
            size: size,
            elements: elements,
        }
    }

    pub fn get(self: &Matrix, i: usize, j: usize) -> Complex
    {
        self.elements[i * MAX_SIZE + j]
    }

    pub fn set(self: &mut Matrix, i: usize, j: usize, value: Complex)
    {
        self.elements[i * MAX_SIZE + j] = value
    }
}

impl fmt::Debug for Matrix
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "Matrix(size-{}, elemetns=...)", self.size)
    }
}

impl PartialEq for Matrix
{
    fn eq(&self, other: &Matrix) -> bool
    {
        assert_eq!(self.size, other.size);

        for i in 0..MAX_ELEMENTS
        {
            if self.elements[i] != other.elements[i]
            {
                return false
            }
        }

        true
    }
}

impl<'a> Mul<&'a Vector> for &'a Matrix
{
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Vector
    {
        let mut output = [Complex::zero(); MAX_SIZE];

        for i in self.size..MAX_SIZE
        {
            assert_eq!(Complex::zero(), rhs[i])
        }
        for i in 0..self.size
        {
            let mut val = Complex::zero();

            for k in 0..self.size
            {
                val += self.get(i, k) * rhs[k]
            }
            output[i] = val;
        }
        output
    }
}

impl <'a> Add<&'a Matrix> for &'a Matrix
{
    type Output = Matrix;

    fn add(self, rhs: &'a Matrix) -> Matrix
    {
        assert_eq!(self.size, rhs.size);

        let mut m = Matrix::new(self.size);

        for i in 0..self.size
        {
            for j in 0..self.size
            {
                m.set(i, j, self.get(i, j) + rhs.get(i, j));
            }
        }
        m
    }
}

impl<'a> Mul<&'a Matrix> for &'a Matrix 
{
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Matrix 
    {
        assert_eq!(self.size, rhs.size);

        let mut m = Matrix::new(self.size);

        for i in 0..self.size 
        {
            for j in 0..self.size 
            {
                let mut val = Complex::zero();

                for k in 0..self.size 
                {
                    val += self.get(i, k) * rhs.get(k, j)
                }

                m.set(i, j, val);
            }
        }

        m
    }
}


#[test]
fn matrix_test() 
{
    let mut m = Matrix::new(2);
    m.set(0, 0, Complex::new(1f64, 0f64));
    m.set(0, 1, Complex::new(2f64, 0f64));
    m.set(1, 0, Complex::new(3f64, 0f64));
    m.set(1, 1, Complex::new(4f64, 0f64));

    let mut v: Vector = [Complex::zero(); MAX_SIZE];
    v[0] = Complex::new(10f64, 0f64);
    v[1] = Complex::new(20f64, 0f64);

    let mut expected: Vector = [Complex::zero(); MAX_SIZE];
    expected[0] = Complex::new(2f64, 0f64);
    expected[1] = Complex::new(110f64, 0f64);

    let mut added = Matrix::new(2);
    added.set(0, 0, Complex::new(2f64, 0f64));
    added.set(0, 1, Complex::new(4f64, 0f64));
    added.set(1, 0, Complex::new(6f64, 0f64));
    added.set(1, 1, Complex::new(8f64, 0f64));

    let mut squared = Matrix::new(2);
    squared.set(0, 0, Complex::new(7f64, 0f64));
    squared.set(0, 1, Complex::new(10f64, 0f64));
    squared.set(1, 0, Complex::new(15f64, 0f64));
    squared.set(1, 1, Complex::new(22f64, 0f64));

    assert_eq!(added, &m + &m);
    assert_eq!(squared, &m * &m);
    assert_eq!(expected, &m * &v);
}