use super::bit::Bit;

/// Represents an AND gate.
///
/// Truth Table:
/// | A | B | Output |
/// |---|---|--------|
/// | 0 | 0 | 0      |
/// | 0 | 1 | 0      |
/// | 1 | 0 | 0      |
/// | 1 | 1 | 1      |
#[derive(Debug, Clone, Copy)]
pub struct And;

impl And {
    /// Creates a new AND gate.
    pub fn new() -> Self {
        And
    }

    /// Applies the AND operation to the inputs.
    ///
    /// # Arguments
    ///
    /// * `a` - First input Bit
    /// * `b` - Second input Bit
    ///
    /// # Returns
    ///
    /// The logical AND of the inputs.
    pub fn evaluate(&self, a: Bit, b: Bit) -> Bit {
        match (a, b) {
            (Bit::Zero, Bit::Zero) => Bit::Zero,
            (Bit::Zero, Bit::One) => Bit::Zero,
            (Bit::One, Bit::Zero) => Bit::Zero,
            (Bit::One, Bit::One) => Bit::One,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_and_zero_is_zero() {
        let gate = And::new();
        assert_eq!(gate.evaluate(Bit::Zero, Bit::Zero), Bit::Zero);
    }

    #[test]
    fn zero_and_one_is_zero() {
        let gate = And::new();
        assert_eq!(gate.evaluate(Bit::Zero, Bit::One), Bit::Zero);
    }

    #[test]
    fn one_and_zero_is_zero() {
        let gate = And::new();
        assert_eq!(gate.evaluate(Bit::One, Bit::Zero), Bit::Zero);
    }

    #[test]
    fn one_and_one_is_one() {
        let gate = And::new();
        assert_eq!(gate.evaluate(Bit::One, Bit::One), Bit::One);
    }
}
