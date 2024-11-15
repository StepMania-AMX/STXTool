use num_traits::Float;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy)]
pub struct FloatWrapper<T: Float>(pub T);

impl<T: Float + PartialEq> PartialEq for FloatWrapper<T> {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < T::epsilon()
    }
}

impl<T: Float + PartialEq> Eq for FloatWrapper<T> {}

impl<T: Float + Debug> Hash for FloatWrapper<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let (mantissa, exponent, sign) = self.0.integer_decode();

        // Normalize mantissa and exponent
        let shift = mantissa.leading_zeros();
        let normalized_mantissa = mantissa << shift;
        let normalized_exponent = exponent - (shift as i16);

        normalized_mantissa.hash(state);
        normalized_exponent.hash(state);
        sign.hash(state);
    }
}

impl<T: Float> FloatWrapper<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: Float> From<T> for FloatWrapper<T> {
    fn from(value: T) -> Self {
        FloatWrapper(value)
    }
}
