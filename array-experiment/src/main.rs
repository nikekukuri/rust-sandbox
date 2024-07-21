use ndarray::{Array1, Array2};
use ndarray::array;

fn main() {
    let arr1 = array![1., 2., 3.];
    let arr2 = array![
        [1., 2., 3.],
        [2., 3., 4.],
        [3., 4., 5.],
    ];

    let result = arr1.dot(&arr2);
    println!("{:?}", result);
}
