fn main() {
    let string = "Hello, world !".to_string();
    //let word_1_last_index = first_word_last_index(&string[..]);
    //let word_1 = first_word(&string[..]);
    let word_2_indexes = second_word_indexes(&string[..]);
    //println!("First word: {}", word_1);
    //println!("First word last index: {}", word_1_last_index);
    println!("Second word indexes: {:?}", word_2_indexes);
    println!(
        "Second word: {:?}",
        &string[word_2_indexes.0..word_2_indexes.1]
    );
}

fn first_word_last_index(s: &str) -> usize {
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
    let mut second_space = 0;
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
