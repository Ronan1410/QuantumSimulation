#![macro_use]

#[macro_export]
macro_rules! c
{
    ($re:expr, $im:expr) =>
    {
        Complex::new($re, $im)
    }
}

#[macro_export]
macro_rules! m
{
    ($a:expr, $b:expr, $c:expr, $d:expr) =>
    {
        {
            let mut m = Matrix::new(2);
            m.set(0, 0, $a);
            m.set(0, 0, $b);
            m.set(0, 0, $c);
            m.set(0, 0, $d);

            m
        }
        
    };
}