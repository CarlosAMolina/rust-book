fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_reference<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest_index = 0;

    for i in 0..list.len() {
        if list[i] > list[largest_index] {
            largest_index = i;
        }
    }

    &list[largest_index]
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    assert_eq!(100, result);
    let result = largest_reference(&number_list[..]);
    assert_eq!(&100, result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    assert_eq!('y', result);
    let result = largest_reference(&char_list[..]);
    assert_eq!(&'y', result);
}

