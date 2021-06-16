use std::collections::HashMap;

fn main() {
    let mut hash_map = HashMap::new();
    let key = "Y".to_string();
    hash_map.insert(
        &key,
        vec!["xx".to_string(), "aa".to_string(), "bb".to_string()],
    );
    assert_eq!(
        vec!["aa".to_string(), "bb".to_string(), "xx".to_string()],
        sort_hash_map_values_by_key(&hash_map, &key)
    );
}

fn sort_hash_map_values_by_key(
    hash_map: &HashMap<&String, Vec<String>>,
    key: &String,
) -> Vec<String> {
    let mut v_to_sort: Vec<String> = vec![];
    for v in &hash_map[key] {
        v_to_sort.push(v.to_string());
    }
    v_to_sort.sort();
    v_to_sort
}
