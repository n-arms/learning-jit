use crate::math::{
    number::Number,
    vector::{Vector, VectorView},
};

use super::neuron::{Dimensions, Neuron};

pub struct WeightedBiasedSum {
    input: usize,
}

impl Neuron for WeightedBiasedSum {
    fn evaluate<T: Number>(&self, input: VectorView<T>, data: VectorView<T>) -> Vector<T> {
        let total = input
            .iter()
            .take(self.input)
            .zip(data.iter().take(self.input))
            .map(|(a, b)| a.clone() * b.clone())
            .fold(data[self.input].clone(), |acc, x| acc + x);

        vec![total]
    }

    fn size(&self) -> Dimensions {
        Dimensions {
            data: self.input + 1,
            input: self.input,
            output: 1,
        }
    }
}

pub struct RectifiedLinear;

impl Neuron for RectifiedLinear {
    fn evaluate<T: Number>(&self, input: VectorView<T>, _data: VectorView<T>) -> Vector<T> {
        let value = input[0]
            .clone()
            .if_positive(input[0].clone(), input[0].clone() / 2.0.into());

        vec![value]
    }

    fn size(&self) -> Dimensions {
        Dimensions {
            data: 0,
            input: 1,
            output: 1,
        }
    }
}

pub fn node(input: usize) -> impl Neuron {
    WeightedBiasedSum { input }.compose(RectifiedLinear)
}

pub fn layer(input: usize, output: usize) -> impl Neuron {
    node(input).repeat(output)
}
