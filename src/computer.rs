use crate::gate::Gate;
use crate::registers::ClassicalRegister;
use crate::registers::QuantumRegister;

#[derive(Debug, Eq, PartialEq)]
enum State
{
    Initializing,
    Running,
    Collapsed,
}

pub struct QuantumComputer
{
    state: State,
    width: usize,

    register: QuantumRegister,
    classical: ClassicalRegister,
}

impl QuantumComputer
{
    pub fn new(width: usize) -> QuantumComputer
    {
        QuantumComputer
        {
            state: State::Initializing,
            width: width,
            register: QuantumRegister::new(width, &ClassicalRegister::zeroed(width)),
            classical: ClassicalRegister::zeroed(width)
        }
    }

    pub fn initialize(&mut self, value: u32)
    {
        assert_eq!(State::Initializing, self.state);

        let classical = ClassicalRegister::from_int(self.width, value);
        self.register = QuantumRegister::new(self.width, &classical);

        self.state = State::Running;
    }

    pub fn apply(&mut self, gate: Gate)
    {
        assert_eq!(State::Running, self.state);

        self.register.apply(gate);
    }

    pub fn collapse(&mut self)
    {
        assert_eq!(State::Running, self.state);

        self.classical = self.register.collapse();

        self.state = State::Collapsed;
    }

    pub fn reset(&mut self)
    {
        self.state = State::Initializing;
    }

    pub fn value(&self) -> u32
    {
        assert_eq!(State::Collapsed, self.state);

        self.classical.to_int()
    }

    pub fn probabilities(&self) -> Vec<f64>
    {
        assert_eq!(State::Running, self.state);

        self.register.probablilites()
    }
}

#[test]
fn state_test()
{
    let mut c = QuantumComputer::new(3);
    assert_eq!(State::Initializing, c.state);

    c.initialize(5);
    assert_eq!(State::Running, c.state);

    c.collapse();
    assert_eq!(State::Collapsed, c.state);

    c.value();

    c.reset();
    assert_eq!(State::Initializing, c.state);
}

#[test]
fn compute_test()
{
    use crate::gates;

    let mut c = QuantumComputer::new(3);

    c.initialize(5);

    c.apply(gates::identity(3));

    c.collapse();

    assert_eq!(5, c.value());
}

#[test]
fn probabilities_test()
{
    use float_cmp::ApproxEqUlps;
    use crate::gates;

    let mut c = QuantumComputer::new(1);
    c.initialize(0);

    c.apply(gates::hadamard(1));

    assert_eq!(2, c.probabilities().len());
    assert!(0.5f64.approx_eq_ulps(&c.probabilities()[0], 10));
    assert!(0.5f64.approx_eq_ulps(&c.probabilities()[1], 10));
}