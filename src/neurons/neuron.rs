use crate::math::number::Number;
use crate::math::vector::{Vector, VectorView};

pub struct Dimensions {
    pub data: usize,
    pub input: usize,
    pub output: usize,
}

pub trait Neuron {
    fn evaluate<T: Number>(&self, input: VectorView<T>, data: VectorView<T>) -> Vector<T>;
    fn size(&self) -> Dimensions;
}
