pub fn char_range_to_string(vec: &Vec<char>, start: usize, end: usize) -> String {
    vec[start..end].iter().cloned().collect::<String>()
}
