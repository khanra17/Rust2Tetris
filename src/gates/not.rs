use super::bit::Bit;

/// Represents a NOT gate.
///
/// Truth Table:
/// | Input | Output |
/// |-------|--------|
/// | 0     | 1      |
/// | 1     | 0      |
#[derive(Debug, Clone, Copy)]
pub struct Not;

impl Not {
    /// Creates a new NOT gate.
    pub fn new() -> Self {
        Not
    }

    /// Applies the NOT operation to the input.
    ///
    /// # Arguments
    ///
    /// * `input` - A Bit to be negated.
    ///
    /// # Returns
    ///
    /// The logical NOT of the input.
    pub fn evaluate(&self, input: Bit) -> Bit {
        match input {
            Bit::Zero => Bit::One,
            Bit::One => Bit::Zero,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluates_zero_to_one() {
        let gate = Not::new();
        assert_eq!(gate.evaluate(Bit::Zero), Bit::One);
    }

    #[test]
    fn evaluates_one_to_zero() {
        let gate = Not::new();
        assert_eq!(gate.evaluate(Bit::One), Bit::Zero);
    }
}
