fn main() {
    let vector = vec![5, 4, 1, 2, 3];
    println!("Vector: {:?}", vector);
    assert_eq!(3, mean(&vector));
    assert_eq!(3, median(&vector));
    assert_eq!(vec![5, 4, 1, 2, 3], vector);
}

fn mean(vector: &[usize]) -> usize {
    vector.iter().sum::<usize>() / vector.len()
}

fn median(vector: &Vec<usize>) -> usize {
    let mut vector_aux = vector.clone();
    vector_aux.sort();
    vector_aux[vector.len() / 2]
}
