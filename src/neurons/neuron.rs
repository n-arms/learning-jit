use crate::math::number::Number;
use crate::math::vector::{Vector, VectorView};

use super::combinators::{Compose, Repeat};

pub struct Dimensions {
    pub data: usize,
    pub input: usize,
    pub output: usize,
}

pub trait Neuron {
    fn evaluate<T: Number>(&self, input: VectorView<T>, data: VectorView<T>) -> Vector<T>;
    fn size(&self) -> Dimensions;

    fn compose<N: Neuron>(self, next: N) -> Compose<Self, N>
    where
        Self: Sized,
    {
        Compose::new(self, next)
    }

    fn repeat(self, repetitions: usize) -> Repeat<Self>
    where
        Self: Sized,
    {
        Repeat {
            neuron: self,
            repetitions,
        }
    }
}
