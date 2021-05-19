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
    let mut map = HashMap::new();
    for number in vector {
        let count = map.entry(number).or_insert(0);
        *count += 1;
    }
    println!("Map: {:?}", map);
    // TODO
}
