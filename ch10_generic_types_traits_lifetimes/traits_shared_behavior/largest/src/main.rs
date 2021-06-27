use core::fmt::Debug;

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_reference<T: PartialOrd + Copy + Debug>(list: &[T]) -> &T {
    let mut largest_index = 0;
    println!("{:?}", list[0] > list[1]);

    for &item in list {
        if list[0] > list[2] {
            largest_index = 3;
            println!("hi");
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
    println!("The largest char is {}", result);
}

