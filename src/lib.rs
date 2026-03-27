extern crate float_cmp;

#[macro_use]
extern crate la;
extern crate rand;
mod macros;
pub mod classical;
#[macro_use] mod complex;
mod gate;
mod ket;
mod matrix;
mod register;
mod qbit;