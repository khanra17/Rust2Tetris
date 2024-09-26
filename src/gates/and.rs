use super::bit::Bit;

/// Represents an AND gate.
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
        unimplemented!()
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
