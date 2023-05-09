use super::number::Number;

pub type Vector<A> = Vec<A>;
pub type VectorView<'a, A> = &'a [A];

pub fn squared_mag<A: Number>(vector: VectorView<A>) -> A {
    vector
        .iter()
        .map(|x| x.clone() * x.clone())
        .fold(0.0.into(), |acc, x| acc + x)
}

pub fn sub<A: Number>(a: VectorView<A>, b: VectorView<A>) -> Vector<A> {
    a.iter()
        .zip(b)
        .map(|(x, y)| x.clone() - y.clone())
        .collect()
}

pub fn mul<A: Number>(a: VectorView<A>, b: A) -> Vector<A> {
    a.iter().map(|x| x.clone() * b.clone()).collect()
}
