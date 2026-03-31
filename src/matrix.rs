use std::char::MAX;
use std::fmt;
use std::ops::Add;
use std::ops::Mul;

use crate::complex::Complex;
use crate::matrix;
use crate::macros;

pub const MAX_SIZE: usize = 32;
const MAX_ELEMENTS: usize = MAX_SIZE * MAX_SIZE;

pub type Vector = [Complex; MAX_SIZE];

#[allow(missing_copy_implementations)]
pub struct Matrix
{
    size: usize,
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

    pub fn new_from_elements(size: usize, elements: Vec<Complex>) -> Matrix
    {
        assert!(size <= MAX_SIZE);
        assert_eq!(elements.len(), size * size);

        let mut matrix = Matrix::new(size);
        
        for (i, elem) in elements.iter().enumerate()
        {
            matrix.set(i / size, i % size, *elem);
        }

        matrix
    }

    pub fn size(self: &Matrix) -> usize
    {
        self.size
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

    pub fn embed(self: &mut Matrix, other: &Matrix, i: usize, j: usize)
    {
        assert!(i + other.size <= self.size);
        assert!(j + other.size <= self.size);

        for x in 0..other.size
        {
            for y in 0..other.size
            {
                let value = other.get(x, y);
                self.set(i + x, j + y, value);
            }
        }
    }

    pub fn approx_eq(&self, other: &Matrix) -> bool
    {
        if self.size != other.size
        {
            return false;
        }

        for i in 0..self.size
        {
            for j in 0..self.size
            {
                if !self.get(i, j).approx_eq(&other.get(i, j))
                {
                    return false;
                }
            }
        }

        true
    }
}

impl fmt::Debug for Matrix
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "Matrix(size-{}, elemetns=[", self.size).ok();
        for i in 0..self.size
        {
            write!(f, "\n").ok();
            for j in 0..self.size
            {
                write!(f, "[{:?}]", self.get(i, j)).ok();
            }
        }
        write!(f, "]")
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
    let mut m = m_real![1, 2; 3, 4];

    let mut v: Vector = [Complex::zero(); MAX_SIZE];
    v[0] = c!(10f64, 0f64);
    v[1] = c!(20f64, 0f64);

    let mut expected: Vector = [Complex::zero(); MAX_SIZE];
    expected[0] = c!(2f64, 0f64);
    expected[1] = c!(110f64, 0f64);

    let mut added = m_real![2, 4; 6, 8];

    let mut squared = m_real![7, 10; 15, 22];

    assert_eq!(added, &m + &m);
    assert_eq!(squared, &m * &m);
    assert_eq!(expected, &m * &v);
}

#[test]
fn embed_test()
{
    let mut m = m_real![1, 2; 3, 4];
    let mut n = m_real![5];

    m.embed(&n, 1, 1);

    assert_eq!(m_real![1, 2; 3, 4], m);
}