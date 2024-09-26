use super::and::And;
use super::bit::Bit;
use super::not::Not;
use super::or::Or;

/// Represents an XOR gate.
///
/// Truth Table:
/// | A | B | Output |
/// |---|---|--------|
/// | 0 | 0 | 0      |
/// | 0 | 1 | 1      |
/// | 1 | 0 | 1      |
/// | 1 | 1 | 0      |
#[derive(Debug, Clone, Copy)]
pub struct Xor {
    not: Not,
    and: And,
    or: Or,
}

impl Xor {
    /// Creates a new XOR gate.
    pub fn new() -> Self {
        Xor {
            not: Not::new(),
            and: And::new(),
            or: Or::new(),
        }
    }

    /// Applies the Xor operation to the inputs.
    ///
    /// # Arguments
    ///
    /// * `a` - First input Bit
    /// * `b` - Second input Bit
    ///
    /// # Returns
    ///
    /// The logical Xor of the inputs.
    pub fn evaluate(&self, a: Bit, b: Bit) -> Bit {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_xor_zero_is_zero() {
        let gate = Xor::new();
        assert_eq!(gate.evaluate(Bit::Zero, Bit::Zero), Bit::Zero);
    }

    #[test]
    fn zero_xor_one_is_one() {
        let gate = Xor::new();
        assert_eq!(gate.evaluate(Bit::Zero, Bit::One), Bit::One);
    }

    #[test]
    fn one_xor_zero_is_one() {
        let gate = Xor::new();
        assert_eq!(gate.evaluate(Bit::One, Bit::Zero), Bit::One);
    }

    #[test]
    fn one_xor_one_is_zero() {
        let gate = Xor::new();
        assert_eq!(gate.evaluate(Bit::One, Bit::One), Bit::Zero);
    }
}
