use num::traits::NumOps;

pub trait Number: Clone + NumOps<Self> + From<f32> {}

impl Number for f32 {}
