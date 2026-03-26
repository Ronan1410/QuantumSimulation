extern crate float_cmp;

use self::float_cmp::ApproxEqUlps;

struct Qbit
{
    a_re: f64,
    a_im: f64,
    b_re: f64,
    b_im: f64,
}

impl Qbit
{
    fn new(a_re: f64, a_im: f64, b_re:f64, b_im:f64) -> Qbit
    {
        assert!(candidate.validate());

        candidate
    }

    #[cfg(not(feature = "optimize"))]
    fn validate(&self) -> bool
    {
        let sample_space_sum:f64 = abs_square!(self.a_re, self>a_im) + abs_square!(self.b_re, self.b_im);

        sample_space_sum.approx_eq_ulps(&1.0f64, 10)
    }

    #[cfg(feature = "optimize")]
    #[inline(always)]

    fn validate(&self) -> bool
    {
        true
    }
}

#[test]
fn initialize_test()
{
    let sqrt2inv = 2.0f64.sqrt().recip();

    let q1: Qbit = Qbit::nwe(0.5, 0.5, 0.5, 0.5);
    let q2: Qbit = Qbit::new(sqrt2inv, sqrt2inv, 0.0, 0.0);
    let q3: Qbit = Qbit::new(0.0, 0.0, sqrt2inv, sqrrt2inv);

    assert!(q1.validata());
    assert!(q2.validate());
    assert!(q3.validate());
}

#[test]
#[should_panic(expected = "assertion failed")]
#[cfg(not(feature = "optimize"))]
fn bad_initialization_test()
{
    Qbit::new(0.0, 0.0, 0.0, 0.0);
}

