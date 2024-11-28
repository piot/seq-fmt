use std::fmt::Display;
use std::fmt::Debug;

pub fn comma<T: Display>(values: &[T]) -> String {
    match values.len() {
        0 => String::new(),
        1 => values[0].to_string(),
        _ => {
            let mut result = String::new();
            for (i, value) in values.iter().enumerate() {
                if i > 0 {
                    result.push_str(", ");
                }
                result.push_str(&value.to_string());
            }
            result
        }
    }
}


pub fn comma_tuple<K: Display, V: Display>(values: &[(K, V)]) -> String {
    let mut result = String::new();
    for (i, (key, value)) in values.iter().enumerate() {
        if i > 0 {
            result.push_str(", ");
        }
        result.push_str(format!("{}: {}", key, value).as_str());
    }
    result
}
