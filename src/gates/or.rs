use super::bit::Bit;

/// Represents an OR gate.
#[derive(Debug, Clone, Copy)]
pub struct Or;

impl Or {
    /// Creates a new OR gate.
    pub fn new() -> Self {
        Or
    }

    /// Applies the OR operation to the inputs.
    ///
    /// # Arguments
    ///
    /// * `a` - First input Bit
    /// * `b` - Second input Bit
    ///
    /// # Returns
    ///
    /// The logical OR of the inputs.
    pub fn evaluate(&self, a: Bit, b: Bit) -> Bit {
        match (a, b) {
            (Bit::Zero, Bit::Zero) => Bit::Zero,
            (Bit::Zero, Bit::One) => Bit::One,
            (Bit::One, Bit::Zero) => Bit::One,
            (Bit::One, Bit::One) => Bit::One,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_or_zero_is_zero() {
        let gate = Or::new();
        assert_eq!(gate.evaluate(Bit::Zero, Bit::Zero), Bit::Zero);
    }

    #[test]
    fn zero_or_one_is_one() {
        let gate = Or::new();
        assert_eq!(gate.evaluate(Bit::Zero, Bit::One), Bit::One);
    }

    #[test]
    fn one_or_zero_is_one() {
        let gate = Or::new();
        assert_eq!(gate.evaluate(Bit::One, Bit::Zero), Bit::One);
    }

    #[test]
    fn one_or_one_is_one() {
        let gate = Or::new();
        assert_eq!(gate.evaluate(Bit::One, Bit::One), Bit::One);
    }
}
