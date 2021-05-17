fn main() {
    let vector = vec![5, 4, 1, 2, 3];
    println!("Vector: {:?}", vector);
    assert_eq!(3, mean(vector));
}

fn mean(vector: Vec<usize>) -> usize {
    let mut sum = 0;
    for number in vector.iter() {
        sum += number;
    }
    sum / vector.len()
}
