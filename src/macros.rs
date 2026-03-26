#![macro_use]

macro_rules! square
{
    ($x:expr) => 
    {
        $x * $x
    };
}

macro_rules! abs_square
{
    ($re:expr, $im:expr) => 
    {
        square!($re) + square!($im)
    };
}