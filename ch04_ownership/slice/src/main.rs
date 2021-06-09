fn main() {
    let string = "Add Carlos to develop team".to_string();
    assert_eq!("Add", first_word(&string[..]));
    assert_eq!(3, first_space_index(&string[..]));
    assert_eq!((4, 10), second_word_indexes(&string[..]));
    let word_2_indexes = second_word_indexes(&string[..]);
    assert_eq!("Carlos", &string[word_2_indexes.0..word_2_indexes.1]);
    let spaces_index = blank_spaces_index(&string);
    assert_eq!(vec![3, 10, 13, 21], spaces_index);
    assert_eq!("Carlos", &string[spaces_index[0] + 1..spaces_index[1]]);
    assert_eq!("develop", &string[spaces_index[2] + 1..spaces_index[3]]);
    assert_eq!("team", &string[spaces_index[3] + 1..string.len()]);
}

fn first_space_index(s: &str) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn second_word_indexes(s: &str) -> (usize, usize) {
    let bytes = s.as_bytes();
    let mut first_space = 0;
    let second_space;
    for (i, &item) in bytes.iter().enumerate() {
        //println!("{}", i);
        //println!("{}", &item);
        if first_space == 0 && item == b' ' {
            first_space = i;
            //println!("First: {}", first_space);
        } else if first_space != 0 && item == b' ' {
            second_space = i;
            //println!("Second space: {}", second_space);
            return (first_space + 1, second_space);
        }
    }
    (0, s.len())
}

fn blank_spaces_index(s: &str) -> Vec<usize> {
    let bytes = s.as_bytes();
    let mut indexes = vec![];
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            indexes.push(i);
        }
    }
    indexes
}
