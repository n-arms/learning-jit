use crate::math::{
    number::Number,
    vector::{Vector, VectorView},
};

use super::neuron::{Dimensions, Neuron};

pub struct Compose<A, B> {
    first: A,
    second: B,
}

impl<A: Neuron, B: Neuron> Compose<A, B> {
    pub fn new(first: A, second: B) -> Self {
        assert_eq!(first.size().output, second.size().input);

        Self { first, second }
    }
}

impl<A: Neuron, B: Neuron> Neuron for Compose<A, B> {
    fn evaluate<T: Number>(&self, input: VectorView<T>, data: VectorView<T>) -> Vector<T> {
        let first_data = &data[..self.first.size().data];
        let second_data = &data[self.first.size().data..];

        let second_input = self.first.evaluate(input, first_data);

        self.second.evaluate(&second_input, second_data)
    }

    fn size(&self) -> Dimensions {
        Dimensions {
            data: self.first.size().data + self.second.size().data,
            input: self.first.size().input,
            output: self.second.size().output,
        }
    }
}

pub struct Repeat<A> {
    pub neuron: A,
    pub repetitions: usize,
}

impl<A: Neuron> Neuron for Repeat<A> {
    fn evaluate<T: Number>(&self, input: VectorView<T>, data: VectorView<T>) -> Vector<T> {
        let mut output = Vec::new();
        let data_size = self.neuron.size().data;

        for i in 0..self.repetitions {
            let local_data = &data[i * data_size..(i + 1) * data_size];
            output.extend(self.neuron.evaluate(input, local_data));
        }

        output
    }

    fn size(&self) -> Dimensions {
        Dimensions {
            data: self.neuron.size().data * self.repetitions,
            input: self.neuron.size().input,
            output: self.neuron.size().output * self.repetitions,
        }
    }
}
