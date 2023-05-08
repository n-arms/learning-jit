use num::traits::NumOps;

pub trait Number: Clone + NumOps<Self> + From<f32> {
    fn if_positive(self, consequent: Self, alternative: Self) -> Self;
}

impl Number for f32 {
    fn if_positive(self, consequent: Self, alternative: Self) -> Self {
        if self >= 0.0 {
            consequent
        } else {
            alternative
        }
    }
}
