use std::collections::HashMap;

fn main() {
    let vector = vec![5, 4, 1, 4, 3];
    println!("Vector: {:?}", vector);
    assert_eq!(3, mean(&vector));
    assert_eq!(4, median(&vector));
    assert_eq!(4, mode(&vector));
    assert_eq!(vec![5, 4, 1, 4, 3], vector);
}

fn mean(vector: &Vec<usize>) -> usize {
    vector.iter().sum::<usize>() / vector.len()
}

fn median(vector: &Vec<usize>) -> usize {
    let mut vector_aux = vector.clone();
    vector_aux.sort();
    vector_aux[vector.len() / 2]
}

fn mode(vector: &Vec<usize>) -> usize {
    let mut values_count = HashMap::new();
    for number in vector {
        let count = values_count.entry(number).or_insert(0);
        *count += 1;
    }
    let count_max = values_count.values().max().unwrap();
    let mut result_iter = values_count.iter().filter(|(&_x, &y)| &y == count_max);
    let result = result_iter.next().unwrap().0;
    **result as usize
}
