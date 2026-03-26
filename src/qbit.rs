extern crate float_cmp;

use self::float_cmp::ApproxEqUlps;

struct NonEntangledQbit
{
    a_re: f64,
    a_im: f64,
    b_re: f64,
    b_im: f64,
}

impl NonEntangledQbit
{
    fn new(a_re: f64, a_im: f64, b_re:f64, b_im:f64) -> NonEntangledQbit
    {
        let candidate = NonEntangledQbit
        {
            a_re: a_re,
            a_im: a_im,
            b_re: b_re,
            b_im: b_im,
        };
        assert!(candidate.validate());

        candidate
    }

    #[cfg(not(feature = "optimize"))]
    fn validate(&self) -> bool
    {
        let sample_space_sum:f64 = abs_square!(self.a_re, self.a_im) + abs_square!(self.b_re, self.b_im);

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

    let q1: NonEntangledQbit = NonEntangledQbit::new(0.5, 0.5, 0.5, 0.5);
    let q2: NonEntangledQbit = NonEntangledQbit::new(sqrt2inv, sqrt2inv, 0.0, 0.0);
    let q3: NonEntangledQbit = NonEntangledQbit::new(0.0, 0.0, sqrt2inv, sqrt2inv);

    assert!(q1.validate());
    assert!(q2.validate());
    assert!(q3.validate());
}

#[test]
#[should_panic(expected = "assertion failed")]
#[cfg(not(feature = "optimize"))]
fn bad_initialization_test()
{
    NonEntangledQbit::new(0.0, 0.0, 0.0, 0.0);
}

