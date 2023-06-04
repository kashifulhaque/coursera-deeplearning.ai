use ndarray::{Array, Array1, ArrayBase};

fn array<T>(vector: Vec<T>) -> ArrayBase<Vec<T>, ndarray::Dim<[usize; 1]>> where T: Clone {
    Array1::from(vector)
}

fn arange(range: f64, step_size: f64) -> Array<f64, ndarray::Dim<usize, 1>> {
    Array::range(0., range, step_size)
}

fn main() {
    let array_1D = array(vec![25, 12, 1997]);
    let b = arange(10, 2);
    println!("{:?}", array_1D);
    println!("{:?}", b);
}
