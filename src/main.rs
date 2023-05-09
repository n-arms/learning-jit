mod compile;
mod ir;
mod math;
mod neurons;

use math::vector::*;
use neurons::neuron::Neuron;

use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

use crate::neurons::learning::layer;

pub struct Example {
    input: Vector<f32>,
    target_output: Vector<f32>,
}

fn error(example: &Example, neuron: &impl Neuron, data: VectorView<f32>) -> f32 {
    squared_mag(&sub(
        &neuron.evaluate(&example.input, data),
        &example.target_output,
    ))
}

const EPSILON: f32 = 0.0001;

fn grad(example: &Example, neuron: &impl Neuron, data: &mut Vector<f32>) -> Vector<f32> {
    let mut grad = Vec::new();
    let base_error = error(example, neuron, data);
    let len = data.len();
    for i in 0..len {
        data[i] += EPSILON;

        let derivative = (error(example, neuron, data) - base_error) / EPSILON;

        grad.push(derivative);

        data[i] -= EPSILON;
    }
    grad
}

fn train(
    examples: &[Example],
    neuron: &impl Neuron,
    learning_rate: f32,
    iterations: usize,
) -> Vector<f32> {
    let mut rng = thread_rng();
    let mut data: Vec<_> = (0..neuron.size().data)
        .map(|_| rng.gen_range(-1.0..1.0))
        .collect();

    for iter in 0..iterations {
        let example = examples.choose(&mut rng).unwrap();
        let grad = grad(&example, neuron, &mut data);
        data = sub(&data, &mul(&grad, learning_rate));
    }

    data
}

fn main() {
    let examples = [
        Example {
            input: vec![0.0, 0.0],
            target_output: vec![0.0],
        },
        Example {
            input: vec![1.0, 0.0],
            target_output: vec![1.0],
        },
        Example {
            input: vec![0.0, 1.0],
            target_output: vec![1.0],
        },
        Example {
            input: vec![1.0, 1.0],
            target_output: vec![0.0],
        },
    ];
    let neuron = layer(2, 4).compose(layer(4, 1));
    let data = train(&examples, &neuron, 0.0001, 1_000_000);

    let mut total_error = 0.0;

    for example in &examples {
        let error = error(example, &neuron, &data);
        total_error += error;
        println!(
            "{:?} -> {:?} (expected {:?}, error {})",
            example.input,
            neuron.evaluate(&example.input, &data),
            example.target_output,
            error
        );
    }
    println!(
        "average error is {} on data {:?}",
        total_error as f32 / examples.len() as f32,
        data
    );
}
