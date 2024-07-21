use rust_simple_MLP::layer::Layer;
use rust_simple_MLP::model::Model;
use rust_simple_MLP::activation::Activation;
use ndarray::*;

fn get_learn_data() -> (Vec<Array1<f64>>, Vec<Array1<f64>>) {
    let mut x = Vec::new();

    // XOR
    //let data0 = array![0., 0.];
    //let data1 = array![1., 0.];
    //let data2 = array![0., 1.];
    //let data3 = array![1., 1.];

    // y = 2x**2
    let data0 = array![2.];
    let data1 = array![4.];
    let data2 = array![9.];
    let data3 = array![3.];

    x.push(data0);
    x.push(data1);
    x.push(data2);
    x.push(data3);

    let mut t = Vec::new();

    // XOR
    //let teacher_data0 = array![0.];
    //let teacher_data1 = array![1.];
    //let teacher_data2 = array![1.];
    //let teacher_data3 = array![0.];


    let teacher_data0 = array![4.];
    let teacher_data1 = array![16.];
    let teacher_data2 = array![81.];
    let teacher_data3 = array![9.];


    t.push(teacher_data0);
    t.push(teacher_data1);
    t.push(teacher_data2);
    t.push(teacher_data3);

    (x, t)
}

fn main() {
    let (x, t) = get_learn_data();
    let input_size = x[0].len();
    let output_size = t[0].len();

    let hidden_size = 3;

    let mut layers = Vec::new();
    let layer1 = Layer::new(input_size, hidden_size, Activation::Relu);
    layers.push(layer1);
    let layer2 = Layer::new(hidden_size, output_size, Activation::Sigmoid);
    layers.push(layer2);

    let mut model = Model::new(input_size, layers, output_size);
    model.learn_iter(x, t, 400);

    println!("--- Model Evaluation ---");
    // XOR
    //println!("Predicted value = {:?}, Input value = [0, 0], Expected = 0.0", model.predict(array![0., 0.])[0]);
    //println!("Predicted value = {:?}, Input value = [0, 1], Expected = 1.0", model.predict(array![0., 1.])[0]);
    //println!("Predicted value = {:?}, Input value = [1, 0], Expected = 1.0", model.predict(array![1., 0.])[0]);
    //println!("Predicted value = {:?}, Input value = [1, 1], Expected = 0.0", model.predict(array![1., 1.])[0]);

    println!("Predicted value = {:?}, Input value = [10.], Expected = 100.0", model.predict(array![10.])[0]);
    println!("Predicted value = {:?}, Input value = [20.], Expected = 400.0", model.predict(array![20.])[0]);
    println!("Predicted value = {:?}, Input value = [5.], Expected = 25.0", model.predict(array![5.])[0]);
}
