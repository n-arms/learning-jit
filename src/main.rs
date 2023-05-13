mod bounded;
mod compile;
mod eval;
mod ir;
mod math;
mod neurons;

use ir::expr::Expr;
use ir::register::Register;
use math::vector::*;
use neurons::neuron::Neuron;

use eval::{expr, register};

use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

use std::collections::HashMap;

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
    let mut rng = thread_rng();
    let neuron = layer(1, 4).compose(layer(4, 4)).compose(layer(4, 1));
    let data: Vec<_> = (0..neuron.size().data).map(|i| Expr::Variable(i)).collect();
    let input: Vec<_> = (0..neuron.size().input)
        .map(|i| Expr::Variable(i + neuron.size().data))
        .collect();

    let values: Vec<f32> = (0..neuron.size().data + neuron.size().input)
        .map(|_| rng.gen_range(-1.0..1.0))
        .collect();

    let index_env: HashMap<_, _> = (0..neuron.size().data + neuron.size().input)
        .map(|index| (index, values[index]))
        .collect();

    let register_env: HashMap<_, _> = index_env
        .iter()
        .map(|(index, value)| (Register { index: *index }, *value))
        .collect();

    let expr = neuron.evaluate(&input, &data).pop().unwrap();
    let original_value = eval::expr::evaluate(&expr, &index_env);
    let mut program =
        compile::flatten::to_program(&expr, register_env.clone().into_keys().collect());
    let old_value = register::evaluate(&program, register_env.clone());
    println!("{:#?} = {}", program, old_value);
    println!("{:?} = {}", expr, original_value);

    assert_eq!(
        old_value, original_value,
        "transformation from expression tree to register instructions failed"
    );

    let old_input = program.input.clone();
    let registers = compile::register_alloc::realloc(&mut program);

    let new_register_env = program
        .input
        .iter()
        .zip(&old_input)
        .map(|(new, old)| (*new, register_env[old]))
        .collect();

    println!("{:#?}", program);
    let new_value = register::evaluate(&program, new_register_env);
    println!(
        "\t= {}, {} registers, {} instructions",
        new_value,
        registers,
        program.statements.len()
    );

    assert!(registers < 50);
    assert_eq!(old_value, new_value, "register allocation failed");
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::eval::register;

    use super::*;

    #[test]
    fn reg_alloc_identity() {
        let mut rng = thread_rng();
        let neuron = layer(1, 4).compose(layer(4, 4)).compose(layer(4, 1));
        let data: Vec<_> = (0..neuron.size().data).map(|i| Expr::Variable(i)).collect();
        let input: Vec<_> = (0..neuron.size().input)
            .map(|i| Expr::Variable(i + neuron.size().data))
            .collect();

        let values: Vec<f32> = (0..neuron.size().data + neuron.size().input)
            .map(|_| rng.gen_range(-1.0..1.0))
            .collect();

        let index_env: HashMap<_, _> = (0..neuron.size().data + neuron.size().input)
            .map(|index| (index, values[index]))
            .collect();

        let register_env: HashMap<_, _> = index_env
            .iter()
            .map(|(index, value)| (Register { index: *index }, *value))
            .collect();

        let expr = neuron.evaluate(&input, &data).pop().unwrap();
        let original_value = eval::expr::evaluate(&expr, &index_env);
        let mut program =
            compile::flatten::to_program(&expr, register_env.clone().into_keys().collect());
        let old_value = register::evaluate(&program, register_env.clone());
        println!("{:#?} = {}", program, old_value);
        println!("{:?} = {}", expr, original_value);

        assert_eq!(
            old_value, original_value,
            "transformation from expression tree to register instructions failed"
        );

        let old_input = program.input.clone();
        let registers = compile::register_alloc::realloc(&mut program);

        let new_register_env = program
            .input
            .iter()
            .zip(&old_input)
            .map(|(new, old)| (*new, register_env[old]))
            .collect();

        println!("{:#?}", program);
        let new_value = register::evaluate(&program, new_register_env);
        println!("\t= {}", new_value);

        assert!(registers < 50);
        assert_eq!(old_value, new_value, "register allocation failed");
    }
}
