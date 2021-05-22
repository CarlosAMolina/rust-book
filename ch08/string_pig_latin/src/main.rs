fn main() {
    assert_eq!("Apple-hay", pig_latin(&"Apple"));
    assert_eq!("irst-fay", pig_latin(&"first"));
}

fn pig_latin(s: &str) -> String {
    if starts_with_vowel(s) {
        return format!("{}-hay", s);
    } else {
        return format!("{}-{}ay", &s[1..], &s[0..1]);
    }
}

fn starts_with_vowel(s: &str) -> bool {
    "aeiou".contains(&s[0..1].to_lowercase())
}
