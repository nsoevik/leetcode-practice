pub fn restore_string(s: String, indices: Vec<i32>) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut ret_chars = vec!['a'; indices.len()];
    for (write, read) in indices.iter().enumerate() {
        ret_chars[*read as usize] = chars[write as usize];
    }

    ret_chars.iter().collect::<String>()
}
