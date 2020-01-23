use std::fmt::Debug;

pub fn char_range_to_string(vec: &Vec<char>, start: usize, end: usize) -> String {
    vec[start..end].iter().cloned().collect::<String>()
}

pub fn join_vec_debug<T: Debug>(vec: &Vec<T>) -> String {
    let mut output = String::new();
    for item in vec.iter() {
        output.push_str(&format!("{:?}\n", item));
    }
    return output;
}

pub fn s(_s: &'static str) -> String {
    _s.to_owned()
}
