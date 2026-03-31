use crate::ket::Ket;
use crate::matrix::Matrix;

#[derive(Debug, PartialEq)]
pub struct Gate
{
    width: usize,
    matrix: Matrix,
}

impl Gate
{
    pub fn new(width: usize, matrix: Matrix) -> Gate
    {
        assert_eq!(Ket::size(width), matrix.size());

        Gate
        {
            width: width,
            matrix: matrix
        }
    }
    pub fn width(&self) -> usize 
    {
        self.width
    }

    pub fn matrix(&self) -> &Matrix 
    {
        &self.matrix
    }

    pub fn permute(&self, permutation: Vec<usize>) -> Gate
    {
        let m = self.matrix.permute_rows(permutation);

        Gate::new(self.width, m)
    }
}