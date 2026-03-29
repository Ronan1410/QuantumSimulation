#![macro_use]

#[macro_export]
macro_rules! c
{
    ($re:expr, $im:expr) =>
    {
        Complex::new($re, $im)
    }
}

macro_rules! m_one
{
    ($item:tt) => (1)
}

macro_rules! m_rec
{
    ([$($row:tt),*] [$($i:expr),*]) => ({
        let _rows = 0 $(+ m_one!($row))*;
        let _cols = (0 $(+ m_one!($i))*) / _rows;

        assert_eq!(_rows, _cols);

        Matrix::new_from_elements(_rows, vec![$($i), *])
    })
}

#[macro_export]
macro_rules! m
{
    ( $( $( $i:expr ),* );* ) => ( m_rec!([$([$($i),*]),*] [$($($i),*),*]) )
}

#[macro_export]
macro_rules! m_real
{
    ( $( $( $i:expr ),* );* ) => ( m_rec!([$([$(Complex::new($i as f64, 0f64)),*]),*]
                                        [$($(Complex::new($i as f64, 0f64)),*),*]) )
}